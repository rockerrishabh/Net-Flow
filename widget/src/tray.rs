//! Dedicated persistent notification-area (system tray) host for Net Flow.
//!
//! Architecture (v0.4.0):
//! - The persistent Tray Host process owns authoritative cumulative session telemetry,
//!   independent bandwidth alert state machines, and the Win32 ICMP latency probe loop.
//! - Runs as a single instance via named mutex `Global\NetFlow_Tray_Mutex` (with `Local\` fallback).
//! - Signals session resets to/from the Widget COM server via `Global\NetFlow_ResetSession_Event`.
//! - Periodically persists authoritative snapshots to `%LOCALAPPDATA%\NetFlow\session_state.json`.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock, mpsc};
use std::time::{Duration, Instant};

use net_flow_core::backend::{NetworkBackend, NetworkSnapshot};
use net_flow_core::card::load_user_config;
use net_flow_core::{
    BandwidthAlertConfig, BandwidthAlertEngine, format_bandwidth, sample_latency_snapshot,
};
use windows::Win32::Foundation::{
    CloseHandle, GetLastError, HINSTANCE, HWND, LPARAM, LRESULT, POINT, WAIT_OBJECT_0, WPARAM,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::{
    CreateEventW, CreateMutexW, EVENT_MODIFY_STATE, OpenEventW, OpenMutexW, ReleaseMutex, SetEvent,
    WaitForSingleObject,
};
use windows::Win32::UI::Shell::{
    NIF_GUID, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_MODIFY, NOTIFYICONDATAW,
    Shell_NotifyIconW, ShellExecuteW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DestroyWindow,
    DispatchMessageW, GWLP_USERDATA, GetCursorPos, GetMessageW, GetWindowLongPtrW, HICON,
    HWND_MESSAGE, IDI_APPLICATION, LoadIconW, MF_SEPARATOR, MF_STRING, MSG, PostMessageW,
    PostQuitMessage, RegisterClassExW, SW_SHOWNORMAL, SetForegroundWindow, SetTimer,
    SetWindowLongPtrW, TPM_BOTTOMALIGN, TPM_RIGHTALIGN, TrackPopupMenu, TranslateMessage,
    WINDOW_EX_STYLE, WINDOW_STYLE, WM_APP, WNDCLASSEXW,
};
use windows::core::{GUID, PCWSTR, w};

use crate::provider::{LockExt, RwLockExt};

use windows::Win32::System::Threading::SYNCHRONIZATION_ACCESS_RIGHTS;

const SYNCHRONIZE: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(0x0010_0000);

/// Stable identity for the notification-area icon.
const TRAY_GUID: GUID = GUID::from_u128(0x7d3f9c21_5a48_4e6b_9f10_2c8b7a4d6e51);

/// Custom callback message delivered to the window proc for tray icon events.
const WM_TRAYICON: u32 = WM_APP + 1;
/// Timer identifier driving periodic tooltip refreshes (~1 Hz).
const TOOLTIP_TIMER_ID: usize = 0x4E46;
const TOOLTIP_TIMER_MS: u32 = 1000;

/// Context-menu command identifiers.
const IDM_OPEN_WIDGETS: usize = 1000;
const IDM_RESET_SESSION: usize = 1001;
const IDM_EXIT: usize = 1002;

const WM_LBUTTONUP: u32 = 0x0202;
const WM_RBUTTONUP: u32 = 0x0205;

// Window messages handled by the tray window proc.
const WM_NULL: u32 = 0x0000;
const WM_CREATE: u32 = 0x0001;
const WM_DESTROY: u32 = 0x0002;
const WM_CLOSE: u32 = 0x0010;
const WM_COMMAND: u32 = 0x0111;
const WM_TIMER: u32 = 0x0113;

/// Guard holding the single-instance mutex for the tray process.
pub struct TrayMutexGuard {
    handle: windows::Win32::Foundation::HANDLE,
}

impl Drop for TrayMutexGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = ReleaseMutex(self.handle);
            let _ = CloseHandle(self.handle);
        }
    }
}

/// Attempts to acquire the named single-instance mutex for the tray host.
pub fn try_acquire_tray_mutex() -> Option<TrayMutexGuard> {
    unsafe {
        let mut handle = CreateMutexW(None, true, w!("Global\\NetFlow_Tray_Mutex")).ok();
        if handle.is_none() || GetLastError().0 == 5 {
            handle = CreateMutexW(None, true, w!("Local\\NetFlow_Tray_Mutex")).ok();
        }
        let handle = handle?;
        if handle.is_invalid() {
            return None;
        }
        if GetLastError().0 == 183 {
            let _ = CloseHandle(handle);
            return None;
        }
        Some(TrayMutexGuard { handle })
    }
}

/// Checks whether an instance of the persistent tray host is currently running.
pub fn is_tray_running() -> bool {
    unsafe {
        if let Ok(h) = OpenMutexW(SYNCHRONIZE, false, w!("Global\\NetFlow_Tray_Mutex"))
            && !h.is_invalid()
        {
            let _ = CloseHandle(h);
            return true;
        }
        if let Ok(h) = OpenMutexW(SYNCHRONIZE, false, w!("Local\\NetFlow_Tray_Mutex"))
            && !h.is_invalid()
        {
            let _ = CloseHandle(h);
            return true;
        }
        false
    }
}

/// Spawns the persistent tray host as a detached process (self-healing recovery mechanism).
pub fn spawn_tray_host_detached() {
    if let Ok(exe_path) = std::env::current_exe() {
        use std::process::Command;
        let _ = Command::new(exe_path).arg("--tray").spawn();
    }
}

/// Creates or opens the named auto-reset event for session reset synchronization.
pub fn create_reset_event() -> Option<windows::Win32::Foundation::HANDLE> {
    unsafe {
        let mut handle =
            CreateEventW(None, false, false, w!("Global\\NetFlow_ResetSession_Event")).ok();
        if handle.is_none() || GetLastError().0 == 5 {
            handle = CreateEventW(None, false, false, w!("Local\\NetFlow_ResetSession_Event")).ok();
        }
        handle.filter(|h| !h.is_invalid())
    }
}

/// Signals the named session reset event across processes.
pub fn signal_reset_session_event() {
    unsafe {
        if let Ok(h) = OpenEventW(
            EVENT_MODIFY_STATE,
            false,
            w!("Global\\NetFlow_ResetSession_Event"),
        ) && !h.is_invalid()
        {
            let _ = SetEvent(h);
            let _ = CloseHandle(h);
            return;
        }
        if let Ok(h) = OpenEventW(
            EVENT_MODIFY_STATE,
            false,
            w!("Local\\NetFlow_ResetSession_Event"),
        ) && !h.is_invalid()
        {
            let _ = SetEvent(h);
            let _ = CloseHandle(h);
        }
    }
}

/// Authoritative state owned by the persistent Tray Host.
pub struct TrayState {
    pub backend: Arc<Mutex<NetworkBackend>>,
    pub latest_snapshot: Arc<RwLock<NetworkSnapshot>>,
    pub alert_config: Arc<Mutex<BandwidthAlertConfig>>,
    pub ui_dirty: Arc<AtomicBool>,
}

/// Per-window context stashed in `GWLP_USERDATA` for the window proc.
struct TrayContext {
    state: Arc<TrayState>,
    quit: Arc<AtomicBool>,
}

/// Main entry point for the persistent Tray Host process.
pub fn run_tray_host() -> windows_core::Result<()> {
    // 1. Ensure single-instance execution
    let _mutex = match try_acquire_tray_mutex() {
        Some(m) => m,
        None => {
            // Already running
            return Ok(());
        }
    };

    let backend = Arc::new(Mutex::new(NetworkBackend::load_or_create(
        net_flow_core::AggregateMode::PhysicalTransport,
    )));
    let initial_snapshot = {
        let mut b = backend.lock_safe();
        b.sample().unwrap_or_default()
    };
    let latest_snapshot = Arc::new(RwLock::new(initial_snapshot));
    let alert_config = Arc::new(Mutex::new(net_flow_core::load_alert_config()));
    let ui_dirty = Arc::new(AtomicBool::new(false));
    let running = Arc::new(AtomicBool::new(true));

    let state = Arc::new(TrayState {
        backend: Arc::clone(&backend),
        latest_snapshot: Arc::clone(&latest_snapshot),
        alert_config: Arc::clone(&alert_config),
        ui_dirty: Arc::clone(&ui_dirty),
    });

    // 2. Spawn dedicated telemetry, latency probe, and alert worker
    let running_worker = Arc::clone(&running);
    let state_worker = Arc::clone(&state);
    let worker_thread = std::thread::Builder::new()
        .name("netflow-tray-worker".to_string())
        .spawn(move || {
            run_tray_worker(running_worker, state_worker);
        })
        .map_err(|_| windows_core::Error::empty())?;

    // 3. Spawn dedicated UI message loop thread
    let (tx, rx) = mpsc::channel::<isize>();
    let running_tray = Arc::clone(&running);
    let state_tray = Arc::clone(&state);
    let tray_thread = std::thread::Builder::new()
        .name("netflow-tray-ui".to_string())
        .spawn(move || {
            run_tray_ui(state_tray, running_tray, tx);
        })
        .map_err(|_| windows_core::Error::empty())?;

    let hwnd = rx.recv().unwrap_or(0);
    if hwnd == 0 {
        running.store(false, Ordering::SeqCst);
        let _ = tray_thread.join();
        let _ = worker_thread.join();
        return Ok(());
    }

    // Wait until exit requested
    while running.load(Ordering::SeqCst) {
        std::thread::sleep(Duration::from_millis(250));
    }

    // Clean shutdown: post close to window, join threads, persist session
    unsafe {
        let _ = PostMessageW(
            Some(HWND(hwnd as *mut c_void)),
            WM_CLOSE,
            WPARAM(0),
            LPARAM(0),
        );
    }
    let _ = tray_thread.join();
    let _ = worker_thread.join();

    {
        let b = backend.lock_safe();
        b.persist_session();
    }

    Ok(())
}

/// Telemetry sampling, IPv4 ICMP latency probing, and bandwidth alert loop.
fn run_tray_worker(running: Arc<AtomicBool>, state: Arc<TrayState>) {
    let reset_event = create_reset_event();
    let mut alert_engine = BandwidthAlertEngine::default();
    let sample_period = Duration::from_millis(net_flow_core::SAMPLING_INTERVAL_MS);
    let persist_period = Duration::from_secs(5);
    let latency_period = Duration::from_secs(2);

    let mut next_sample = Instant::now() + sample_period;
    let mut last_persist = Instant::now();
    let mut last_latency = Instant::now() - latency_period;

    while running.load(Ordering::SeqCst) {
        let now = Instant::now();
        let wait = next_sample.saturating_duration_since(now);
        let wait_ms = (wait.as_millis() as u32).min(250);

        // Check reset event if available
        if let Some(event) = reset_event {
            unsafe {
                let status = WaitForSingleObject(event, wait_ms);
                if status == WAIT_OBJECT_0 {
                    let mut b = state.backend.lock_safe();
                    b.reset_session();
                    let s = b.sample().unwrap_or_default();
                    *state.latest_snapshot.write_safe() = s;
                    state.ui_dirty.store(true, Ordering::SeqCst);
                }
            }
        } else {
            std::thread::sleep(Duration::from_millis(wait_ms as u64));
        }

        if !running.load(Ordering::SeqCst) {
            break;
        }

        // 1. Latency Probing every 2s
        if last_latency.elapsed() >= latency_period {
            let user_cfg = load_user_config();
            let snap = sample_latency_snapshot(user_cfg.latency_target, 0, 1000);
            {
                let mut b = state.backend.lock_safe();
                b.set_latency(snap);
            }
            last_latency = Instant::now();
        }

        // 2. Bandwidth Sampling
        let started = Instant::now();
        {
            let mut b = state.backend.lock_safe();
            if let Ok(s) = b.sample() {
                *state.latest_snapshot.write_safe() = s;
            }
            if last_persist.elapsed() >= persist_period {
                b.persist_session();
                last_persist = Instant::now();
            }
        }

        // 3. Alert Engine Evaluation
        let snapshot = state.latest_snapshot.read_safe().clone();
        let alert_prefs = state.alert_config.lock_safe().clone();
        for event in alert_engine.evaluate(
            &alert_prefs,
            snapshot.rx_bps,
            snapshot.tx_bps,
            Instant::now(),
        ) {
            let _ = crate::toast::show_bandwidth_alert(event);
        }

        next_sample = started + sample_period;
    }
}

/// Message loop thread for the notification-area icon.
fn run_tray_ui(state: Arc<TrayState>, quit: Arc<AtomicBool>, tx: mpsc::Sender<isize>) {
    unsafe {
        let hinstance = HINSTANCE(GetModuleHandleW(None).unwrap_or_default().0);
        let class_name = w!("NetFlowTrayWindow");
        let wc = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            lpfnWndProc: Some(wndproc),
            hInstance: hinstance,
            lpszClassName: class_name,
            ..Default::default()
        };
        if RegisterClassExW(&wc) == 0 {
            let _ = tx.send(0);
            return;
        }

        let ctx = Box::new(TrayContext { state, quit });
        let ctx_ptr = Box::into_raw(ctx) as *const c_void;

        let hwnd_result = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            class_name,
            w!("Net Flow"),
            WINDOW_STYLE(0),
            0,
            0,
            0,
            0,
            Some(HWND_MESSAGE),
            None,
            Some(hinstance),
            Some(ctx_ptr),
        );

        let hwnd = match hwnd_result {
            Ok(hwnd) => hwnd,
            Err(_) => {
                drop(Box::from_raw(ctx_ptr as *mut TrayContext));
                let _ = tx.send(0);
                return;
            }
        };

        let _ = tx.send(hwnd.0 as isize);

        add_icon(hwnd);
        SetTimer(Some(hwnd), TOOLTIP_TIMER_ID, TOOLTIP_TIMER_MS, None);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

fn base_notify_data(hwnd: HWND) -> NOTIFYICONDATAW {
    NOTIFYICONDATAW {
        cbSize: size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uFlags: NIF_GUID,
        guidItem: TRAY_GUID,
        ..Default::default()
    }
}

#[allow(clippy::manual_dangling_ptr)]
unsafe fn load_icon() -> HICON {
    unsafe {
        let hinstance = HINSTANCE(GetModuleHandleW(None).unwrap_or_default().0);
        if let Ok(icon) = LoadIconW(Some(hinstance), PCWSTR(1 as *const u16))
            && !icon.is_invalid()
        {
            return icon;
        }
        LoadIconW(None, IDI_APPLICATION).unwrap_or_default()
    }
}

fn set_tip(data: &mut NOTIFYICONDATAW, text: &str) {
    let mut chars = text.encode_utf16().take(data.szTip.len() - 1);
    for slot in data.szTip.iter_mut() {
        *slot = chars.next().unwrap_or(0);
    }
}

unsafe fn add_icon(hwnd: HWND) {
    unsafe {
        let mut data = base_notify_data(hwnd);
        data.uFlags = NIF_GUID | NIF_MESSAGE | NIF_ICON | NIF_TIP;
        data.uCallbackMessage = WM_TRAYICON;
        data.hIcon = load_icon();
        set_tip(&mut data, "Net Flow");
        let _ = Shell_NotifyIconW(NIM_ADD, &data);
    }
}

unsafe fn remove_icon(hwnd: HWND) {
    unsafe {
        let data = base_notify_data(hwnd);
        let _ = Shell_NotifyIconW(NIM_DELETE, &data);
    }
}

unsafe fn update_tooltip(hwnd: HWND) {
    unsafe {
        let Some(ctx) = context(hwnd) else { return };
        let (rx_bps, tx_bps, latency) = {
            let snapshot = ctx.state.latest_snapshot.read_safe();
            (snapshot.rx_bps, snapshot.tx_bps, snapshot.latency)
        };
        let tip = format!(
            "Net Flow\n\u{2193} {}  \u{2191} {}\nLatency: {}",
            format_bandwidth(rx_bps),
            format_bandwidth(tx_bps),
            latency.detailed_display_text(),
        );
        let mut data = base_notify_data(hwnd);
        data.uFlags = NIF_GUID | NIF_TIP;
        set_tip(&mut data, &tip);
        let _ = Shell_NotifyIconW(NIM_MODIFY, &data);
    }
}

unsafe fn show_menu(hwnd: HWND) {
    unsafe {
        let Ok(menu) = CreatePopupMenu() else { return };
        let _ = AppendMenuW(menu, MF_STRING, IDM_OPEN_WIDGETS, w!("Open Widgets Board"));
        let _ = AppendMenuW(menu, MF_SEPARATOR, 0, PCWSTR::null());
        let _ = AppendMenuW(
            menu,
            MF_STRING,
            IDM_RESET_SESSION,
            w!("Reset session totals"),
        );
        let _ = AppendMenuW(menu, MF_SEPARATOR, 0, PCWSTR::null());
        let _ = AppendMenuW(menu, MF_STRING, IDM_EXIT, w!("Exit Net Flow"));

        let mut point = POINT::default();
        let _ = GetCursorPos(&mut point);
        let _ = SetForegroundWindow(hwnd);
        let _ = TrackPopupMenu(
            menu,
            TPM_BOTTOMALIGN | TPM_RIGHTALIGN,
            point.x,
            point.y,
            None,
            hwnd,
            None,
        );
        let _ = PostMessageW(Some(hwnd), WM_NULL, WPARAM(0), LPARAM(0));
        let _ = DestroyMenu(menu);
    }
}

fn reset_session(state: &Arc<TrayState>) {
    {
        let mut backend = state.backend.lock_safe();
        backend.reset_session();
        let snapshot = backend.sample().unwrap_or_default();
        *state.latest_snapshot.write_safe() = snapshot;
    }
    state.ui_dirty.store(true, Ordering::SeqCst);
    signal_reset_session_event();
}

unsafe fn context(hwnd: HWND) -> Option<&'static TrayContext> {
    unsafe {
        let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const TrayContext;
        if ptr.is_null() { None } else { Some(&*ptr) }
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            WM_CREATE => {
                let create =
                    &*(lparam.0 as *const windows::Win32::UI::WindowsAndMessaging::CREATESTRUCTW);
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, create.lpCreateParams as isize);
                LRESULT(0)
            }
            WM_TRAYICON => {
                let event = (lparam.0 as u32) & 0xFFFF;
                match event {
                    WM_RBUTTONUP => show_menu(hwnd),
                    WM_LBUTTONUP => {
                        update_tooltip(hwnd);
                    }
                    _ => {}
                }
                LRESULT(0)
            }
            WM_TIMER => {
                if wparam.0 == TOOLTIP_TIMER_ID {
                    update_tooltip(hwnd);
                }
                LRESULT(0)
            }
            WM_COMMAND => {
                let id = wparam.0 & 0xFFFF;
                match id {
                    IDM_OPEN_WIDGETS => {
                        let _ = ShellExecuteW(
                            None,
                            w!("open"),
                            w!("ms-widgets:"),
                            None,
                            None,
                            SW_SHOWNORMAL,
                        );
                    }
                    IDM_RESET_SESSION => {
                        if let Some(ctx) = context(hwnd) {
                            reset_session(&ctx.state);
                        }
                    }
                    IDM_EXIT => {
                        if let Some(ctx) = context(hwnd) {
                            ctx.quit.store(true, Ordering::SeqCst);
                        }
                        let _ = PostMessageW(Some(hwnd), WM_CLOSE, WPARAM(0), LPARAM(0));
                    }
                    _ => {}
                }
                LRESULT(0)
            }
            WM_CLOSE => {
                let _ = DestroyWindow(hwnd);
                LRESULT(0)
            }
            WM_DESTROY => {
                remove_icon(hwnd);
                let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
                if ptr != 0 {
                    drop(Box::from_raw(ptr as *mut TrayContext));
                    SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
                }
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}
