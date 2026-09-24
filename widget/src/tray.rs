//! In-process notification-area (system tray) icon for the Net Flow widget host.
//!
//! The tray is deliberately scoped to the lifetime of the widget COM-server
//! process: it is added with `Shell_NotifyIconW(NIM_ADD)` when the process
//! becomes active and removed with `NIM_DELETE` during shutdown. When no widgets
//! remain pinned the host idles out and exits, and the tray icon disappears with
//! it — matching the documented idle-shutdown behaviour.
//!
//! The public surface is intentionally tiny (`spawn` / `destroy`) so this can be
//! relocated to a dedicated persistent host process in a future release without
//! redesigning the widget itself.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};

use net_flow_core::format_bandwidth;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Shell::{
    NIF_GUID, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_MODIFY, NOTIFYICONDATAW,
    Shell_NotifyIconW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DestroyWindow,
    DispatchMessageW, GWLP_USERDATA, GetCursorPos, GetMessageW, GetWindowLongPtrW, HICON,
    HWND_MESSAGE, IDI_APPLICATION, LoadIconW, MF_SEPARATOR, MF_STRING, MSG, PostMessageW,
    PostQuitMessage, RegisterClassExW, SetForegroundWindow, SetTimer, SetWindowLongPtrW,
    TPM_BOTTOMALIGN, TPM_RIGHTALIGN, TrackPopupMenu, TranslateMessage, WINDOW_EX_STYLE,
    WINDOW_STYLE, WM_APP, WNDCLASSEXW,
};
use windows::core::{GUID, PCWSTR, w};

use crate::provider::{LockExt, ProviderState, RwLockExt};

/// Stable identity for the notification-area icon. Microsoft recommends a fixed
/// GUID so Windows consistently associates the icon with this application.
const TRAY_GUID: GUID = GUID::from_u128(0x7d3f9c21_5a48_4e6b_9f10_2c8b7a4d6e51);

/// Custom callback message delivered to the window proc for tray icon events.
const WM_TRAYICON: u32 = WM_APP + 1;
/// Timer identifier driving periodic tooltip refreshes (~1 Hz).
const TOOLTIP_TIMER_ID: usize = 0x4E46;
const TOOLTIP_TIMER_MS: u32 = 1000;

/// Context-menu command identifiers.
const IDM_RESET_SESSION: usize = 1001;
const IDM_EXIT: usize = 1002;

const WM_LBUTTONUP: u32 = 0x0202;
const WM_RBUTTONUP: u32 = 0x0205;

// Window messages handled by the tray window proc. Defined locally to avoid
// depending on which windows-crate feature gate happens to re-export them.
const WM_NULL: u32 = 0x0000;
const WM_CREATE: u32 = 0x0001;
const WM_DESTROY: u32 = 0x0002;
const WM_CLOSE: u32 = 0x0010;
const WM_COMMAND: u32 = 0x0111;
const WM_TIMER: u32 = 0x0113;

/// Per-window context stashed in `GWLP_USERDATA` so the plain-fn window proc can
/// reach the shared provider state and the process quit flag.
struct TrayContext {
    state: Arc<Mutex<ProviderState>>,
    quit: Arc<AtomicBool>,
}

/// Handle to the running tray icon. Dropping is a no-op; call [`TrayIcon::destroy`]
/// to remove the icon and join the message-loop thread deterministically.
pub struct TrayIcon {
    /// Raw `HWND` value, kept as `isize` because window handles are not `Send`.
    hwnd: isize,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl TrayIcon {
    /// Creates the tray icon on a dedicated message-loop thread.
    ///
    /// Returns `None` if the window or notification icon could not be created,
    /// in which case the widget continues to operate without a tray presence.
    pub fn spawn(state: Arc<Mutex<ProviderState>>, quit: Arc<AtomicBool>) -> Option<TrayIcon> {
        let (tx, rx) = mpsc::channel::<isize>();
        let thread = std::thread::Builder::new()
            .name("netflow-tray".to_string())
            .spawn(move || run_tray(state, quit, tx))
            .ok()?;
        // A zero handle means window/icon creation failed on the tray thread.
        let hwnd = rx.recv().unwrap_or(0);
        if hwnd == 0 {
            let _ = thread.join();
            return None;
        }
        Some(TrayIcon {
            hwnd,
            thread: Some(thread),
        })
    }

    /// Removes the notification icon, destroys the hidden window, and joins the thread.
    pub fn destroy(mut self) {
        unsafe {
            let _ = PostMessageW(
                Some(HWND(self.hwnd as *mut c_void)),
                WM_CLOSE,
                WPARAM(0),
                LPARAM(0),
            );
        }
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// Entry point for the tray message-loop thread.
fn run_tray(state: Arc<Mutex<ProviderState>>, quit: Arc<AtomicBool>, tx: mpsc::Sender<isize>) {
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

        // A message-only window (HWND_MESSAGE parent) hosts the tray icon without
        // appearing on the taskbar or alt-tab.
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
                // Reclaim the context box; WM_CREATE never ran to take ownership.
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

/// Builds a zeroed `NOTIFYICONDATAW` bound to this window and the stable GUID.
fn base_notify_data(hwnd: HWND) -> NOTIFYICONDATAW {
    NOTIFYICONDATAW {
        cbSize: size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uFlags: NIF_GUID,
        guidItem: TRAY_GUID,
        ..Default::default()
    }
}

/// Loads the embedded application icon, falling back to the shell default.
///
/// `PCWSTR(1 as *const u16)` is the Win32 `MAKEINTRESOURCEW(1)` idiom: the numeric
/// resource identifier winres assigns to the embedded app icon. It is an integer
/// handle that is never dereferenced, so the dangling-pointer lint is a false
/// positive here (Clippy's `ptr::dangling` suggestion would change the value).
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

/// Writes a NUL-terminated wide string into the fixed 128-char tooltip buffer.
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

/// Refreshes the tooltip with the current aggregate download/upload rates.
unsafe fn update_tooltip(hwnd: HWND) {
    unsafe {
        let Some(ctx) = context(hwnd) else { return };
        let (rx_bps, tx_bps) = {
            let state = ctx.state.lock_safe();
            let snapshot = state.latest_snapshot.read_safe();
            (snapshot.rx_bps, snapshot.tx_bps)
        };
        let tip = format!(
            "Net Flow  \u{2193} {}  \u{2191} {}",
            format_bandwidth(rx_bps),
            format_bandwidth(tx_bps),
        );
        let mut data = base_notify_data(hwnd);
        data.uFlags = NIF_GUID | NIF_TIP;
        set_tip(&mut data, &tip);
        let _ = Shell_NotifyIconW(NIM_MODIFY, &data);
    }
}

/// Shows the right-click context menu anchored at the cursor.
unsafe fn show_menu(hwnd: HWND) {
    unsafe {
        let Ok(menu) = CreatePopupMenu() else { return };
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
        // Required so the menu dismisses correctly when the user clicks elsewhere.
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

/// Resets the cumulative session counters, mirroring the in-card Reset action.
fn reset_session(state: &Arc<Mutex<ProviderState>>) {
    let state = state.lock_safe();
    {
        let mut backend = state.backend.lock_safe();
        backend.reset_session();
        let snapshot = backend.sample().unwrap_or_default();
        *state.latest_snapshot.write_safe() = snapshot;
    }
    state.ui_dirty.store(true, Ordering::SeqCst);
    if let Some(worker) = &state.worker {
        worker.shutdown.1.notify_all();
    }
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
                    WM_LBUTTONUP => update_tooltip(hwnd),
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
