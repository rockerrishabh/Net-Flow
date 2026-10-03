use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;

use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows::Win32::Graphics::Dwm::{DWMWINDOWATTRIBUTE, DwmSetWindowAttribute};
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::Input::KeyboardAndMouse::{TME_LEAVE, TRACKMOUSEEVENT, TrackMouseEvent};
use windows::Win32::UI::Shell::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::{GUID, PCWSTR, Result, w};

use crate::provider::RwLockExt;

/// Custom message posted by the Tray Worker when new telemetry is ready for the flyout.
pub const WM_FLYOUT_SNAPSHOT_UPDATED: u32 = WM_USER + 101;
pub const WM_MOUSELEAVE: u32 = 0x02A3;

/// Baseline logical DIP dimensions for the compact flyout companion.
pub const FLYOUT_WIDTH_DIP: i32 = 328;
pub const FLYOUT_HEIGHT_DIP: i32 = 456;
pub const FLYOUT_MARGIN_DIP: i32 = 8;

/// Window class name registered for the Win32 native flyout.
const FLYOUT_WINDOW_CLASS: PCWSTR = w!("NetFlowFlyoutWindowClass");

/// DWM Window Attributes (from dwmapi.h).
pub const DWMWA_USE_IMMERSIVE_DARK_MODE: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(20);
pub const DWMWA_WINDOW_CORNER_PREFERENCE: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(33);

/// DWM Window Corner Preference enum values.
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DwmWindowCornerPreference {
    Default = 0,
    DoNotRound = 1,
    Round = 2,
    RoundSmall = 3,
}

/// Taskbar screen edge alignment for positioning calculations.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TaskbarEdge {
    Bottom,
    Top,
    Left,
    Right,
}

/// Lifecycle states of the Win32 flyout window.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FlyoutLifecycleState {
    Hidden,
    Opening,
    Visible,
    Closing,
}

/// Per-application bandwidth usage item for the flyout.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FlyoutAppItem {
    pub name: String,
    pub rx_bps: u64,
    pub tx_bps: u64,
}

/// Immutable telemetry snapshot prepared on the TrayWorker thread for presentation.
#[derive(Clone, Debug, Default)]
pub struct FlyoutSnapshot {
    pub rx_bps: f64,
    pub tx_bps: f64,
    pub latency_ms: Option<u32>,
    pub jitter_ms: Option<u32>,
    pub packet_loss_pct: f32,
    pub latency_health: net_flow_core::backend::LatencyHealth,
    pub latency_target_label: String,
    pub primary_medium: net_flow_core::backend::InterfaceMedium,
    pub physical_link_summary: Option<String>,
    pub session_rx_bytes: u64,
    pub session_tx_bytes: u64,
    pub session_duration_secs: u64,
    pub budget_usage_pct: Option<u16>,
    pub budget_days_left: Option<u32>,
    pub budget_cap_bytes: u64,
    pub budget_used_bytes: u64,
    pub active_apps: Vec<FlyoutAppItem>,
    pub sparkline_history: Vec<(f32, f32)>,
}

/// Context stored in `GWLP_USERDATA` for the flyout window procedure.
struct FlyoutContext {
    snapshot: Arc<RwLock<FlyoutSnapshot>>,
    lifecycle: Arc<RwLock<FlyoutLifecycleState>>,
    on_reset: Arc<dyn Fn() + Send + Sync>,
    on_export: Arc<dyn Fn(HWND) + Send + Sync>,
    dpi: AtomicU32,
    open_time: Mutex<Instant>,
    hover_button: AtomicU32, // 0 = none, 1 = export, 2 = reset, 3 = close
    export_rect: Mutex<RECT>,
    reset_rect: Mutex<RECT>,
    close_rect: Mutex<RECT>,
    tracking_mouse: AtomicBool,
}

#[inline]
pub const fn rgb(r: u8, g: u8, b: u8) -> COLORREF {
    COLORREF((r as u32) | ((g as u32) << 8) | ((b as u32) << 16))
}

/// Converts logical device-independent pixels (DIPs) to physical pixels based on monitor DPI.
#[inline]
pub fn dpi_scale(dip: i32, dpi: u32) -> i32 {
    let effective_dpi = if dpi == 0 { 96 } else { dpi };
    ((dip as i64 * effective_dpi as i64 + 48) / 96) as i32
}

/// Point-in-rectangle hit test helper.
#[inline]
pub fn pt_in_rect(rc: &RECT, pt: POINT) -> bool {
    pt.x >= rc.left && pt.x < rc.right && pt.y >= rc.top && pt.y < rc.bottom
}

/// Formats duration into compact human-readable string (e.g., "2h 15m", "4m 20s", "15s").
pub fn format_duration_short(secs: u64) -> String {
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    let s = secs % 60;
    if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else if mins > 0 {
        format!("{}m {}s", mins, s)
    } else {
        format!("{}s", s)
    }
}

/// Pure positioning calculation: places flyout adjacent to tray icon or taskbar edge,
/// clamped strictly inside the active monitor's work area rectangle.
pub fn calculate_flyout_geometry(
    icon_rect: Option<RECT>,
    fallback_edge: TaskbarEdge,
    work_area: RECT,
    flyout_width: i32,
    flyout_height: i32,
    margin: i32,
) -> (i32, i32) {
    let mut x: i32;
    let mut y: i32;

    if let Some(icon) = icon_rect {
        let icon_center_x = (icon.left + icon.right) / 2;
        let icon_center_y = (icon.top + icon.bottom) / 2;

        match fallback_edge {
            TaskbarEdge::Bottom => {
                x = icon_center_x - (flyout_width / 2);
                y = icon.top - flyout_height - margin;
            }
            TaskbarEdge::Top => {
                x = icon_center_x - (flyout_width / 2);
                y = icon.bottom + margin;
            }
            TaskbarEdge::Left => {
                x = icon.right + margin;
                y = icon_center_y - (flyout_height / 2);
            }
            TaskbarEdge::Right => {
                x = icon.left - flyout_width - margin;
                y = icon_center_y - (flyout_height / 2);
            }
        }
    } else {
        match fallback_edge {
            TaskbarEdge::Bottom => {
                x = work_area.right - flyout_width - margin;
                y = work_area.bottom - flyout_height - margin;
            }
            TaskbarEdge::Top => {
                x = work_area.right - flyout_width - margin;
                y = work_area.top + margin;
            }
            TaskbarEdge::Left => {
                x = work_area.left + margin;
                y = work_area.bottom - flyout_height - margin;
            }
            TaskbarEdge::Right => {
                x = work_area.right - flyout_width - margin;
                y = work_area.bottom - flyout_height - margin;
            }
        }
    }

    // Screen-edge clamping to monitor work area
    if x + flyout_width > work_area.right {
        x = work_area.right - flyout_width - margin;
    }
    if x < work_area.left {
        x = work_area.left + margin;
    }
    if y + flyout_height > work_area.bottom {
        y = work_area.bottom - flyout_height - margin;
    }
    if y < work_area.top {
        y = work_area.top + margin;
    }

    (x, y)
}

/// Applies modern DWM window attributes: immersive dark mode and rounded corners.
pub fn apply_window_chrome(hwnd: HWND, dark_mode: bool) -> Result<()> {
    unsafe {
        let dark: i32 = if dark_mode { 1 } else { 0 };
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_USE_IMMERSIVE_DARK_MODE,
            &dark as *const _ as *const c_void,
            std::mem::size_of::<i32>() as u32,
        );

        let corner = DwmWindowCornerPreference::Round as u32;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            &corner as *const _ as *const c_void,
            std::mem::size_of::<u32>() as u32,
        );
    }
    Ok(())
}

/// Checks Windows Registry to determine if system/app theme is currently Dark Mode.
pub fn is_dark_mode_active() -> bool {
    use windows::Win32::System::Registry::{
        HKEY_CURRENT_USER, KEY_READ, REG_DWORD, RegCloseKey, RegOpenKeyExW, RegQueryValueExW,
    };
    unsafe {
        let mut hkey = windows::Win32::System::Registry::HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            w!("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize"),
            Some(0),
            KEY_READ,
            &mut hkey,
        )
        .is_ok()
        {
            let mut value: u32 = 0;
            let mut size = std::mem::size_of::<u32>() as u32;
            let mut val_type = REG_DWORD;
            let res = RegQueryValueExW(
                hkey,
                w!("AppsUseLightTheme"),
                None,
                Some(&mut val_type),
                Some(&mut value as *mut u32 as *mut u8),
                Some(&mut size),
            );
            let _ = RegCloseKey(hkey);
            if res.is_ok() {
                return value == 0;
            }
        }
    }
    true
}

/// Queries exact notification area tray icon bounding rectangle via Shell_NotifyIconGetRect.
pub fn get_tray_icon_rect(tray_hwnd: HWND, tray_guid: GUID) -> Option<RECT> {
    unsafe {
        let identifier = NOTIFYICONIDENTIFIER {
            cbSize: std::mem::size_of::<NOTIFYICONIDENTIFIER>() as u32,
            hWnd: tray_hwnd,
            uID: 0,
            guidItem: tray_guid,
        };
        Shell_NotifyIconGetRect(&identifier).ok()
    }
}

/// Queries taskbar edge orientation and monitor work area.
pub fn get_taskbar_edge_and_work_area(tray_hwnd: HWND) -> (TaskbarEdge, RECT) {
    unsafe {
        let mut abd = APPBARDATA {
            cbSize: std::mem::size_of::<APPBARDATA>() as u32,
            hWnd: HWND::default(),
            uCallbackMessage: 0,
            uEdge: 0,
            rc: RECT::default(),
            lParam: LPARAM(0),
        };
        let _ = SHAppBarMessage(ABM_GETTASKBARPOS, &mut abd);
        let edge = match abd.uEdge {
            0 => TaskbarEdge::Left,
            1 => TaskbarEdge::Top,
            2 => TaskbarEdge::Right,
            _ => TaskbarEdge::Bottom,
        };

        let monitor = MonitorFromWindow(tray_hwnd, MONITOR_DEFAULTTONEAREST);
        let mut mi = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            rcMonitor: RECT::default(),
            rcWork: RECT::default(),
            dwFlags: 0,
        };
        if GetMonitorInfoW(monitor, &mut mi).as_bool() {
            (edge, mi.rcWork)
        } else {
            let width = GetSystemMetrics(SM_CXSCREEN);
            let height = GetSystemMetrics(SM_CYSCREEN);
            (
                edge,
                RECT {
                    left: 0,
                    top: 0,
                    right: width,
                    bottom: height,
                },
            )
        }
    }
}

/// Creates the hidden Win32 native companion flyout window.
pub fn create_flyout_window(
    hinstance: windows::Win32::Foundation::HINSTANCE,
    snapshot: Arc<RwLock<FlyoutSnapshot>>,
    lifecycle: Arc<RwLock<FlyoutLifecycleState>>,
    on_reset: Arc<dyn Fn() + Send + Sync>,
    on_export: Arc<dyn Fn(HWND) + Send + Sync>,
) -> Result<HWND> {
    unsafe {
        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW | CS_DBLCLKS,
            lpfnWndProc: Some(flyout_wndproc),
            hInstance: hinstance,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            lpszClassName: FLYOUT_WINDOW_CLASS,
            ..Default::default()
        };

        let _ = RegisterClassExW(&wc);

        let ctx = Box::new(FlyoutContext {
            snapshot,
            lifecycle,
            on_reset,
            on_export,
            dpi: AtomicU32::new(96),
            open_time: Mutex::new(Instant::now()),
            hover_button: AtomicU32::new(0),
            export_rect: Mutex::new(RECT::default()),
            reset_rect: Mutex::new(RECT::default()),
            close_rect: Mutex::new(RECT::default()),
            tracking_mouse: AtomicBool::new(false),
        });
        let ctx_ptr = Box::into_raw(ctx);

        let hwnd = CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
            FLYOUT_WINDOW_CLASS,
            w!("Net Flow Companion"),
            WS_POPUP | WS_BORDER,
            0,
            0,
            100,
            100,
            None,
            None,
            Some(hinstance),
            Some(ctx_ptr as *const c_void),
        )?;

        let _ = apply_window_chrome(hwnd, is_dark_mode_active());

        Ok(hwnd)
    }
}

/// Toggles the flyout window visibility with state machine transitions and screen-edge positioning.
pub fn toggle_flyout(flyout_hwnd: HWND, tray_hwnd: HWND, tray_guid: GUID) {
    let ctx = unsafe {
        let ptr = GetWindowLongPtrW(flyout_hwnd, GWLP_USERDATA) as *mut FlyoutContext;
        if ptr.is_null() {
            return;
        }
        &*ptr
    };

    let current = *ctx.lifecycle.read_safe();
    if current == FlyoutLifecycleState::Visible || current == FlyoutLifecycleState::Opening {
        *ctx.lifecycle.write_safe() = FlyoutLifecycleState::Closing;
        unsafe {
            let _ = ShowWindow(flyout_hwnd, SW_HIDE);
        }
        *ctx.lifecycle.write_safe() = FlyoutLifecycleState::Hidden;
    } else {
        *ctx.lifecycle.write_safe() = FlyoutLifecycleState::Opening;

        let dpi = unsafe { GetDpiForWindow(flyout_hwnd) };
        let dpi = if dpi == 0 { 96 } else { dpi };
        ctx.dpi.store(dpi, Ordering::SeqCst);

        let width = dpi_scale(FLYOUT_WIDTH_DIP, dpi);
        let height = dpi_scale(FLYOUT_HEIGHT_DIP, dpi);
        let margin = dpi_scale(FLYOUT_MARGIN_DIP, dpi);

        let icon_rect = get_tray_icon_rect(tray_hwnd, tray_guid);
        let (fallback_edge, work_area) = get_taskbar_edge_and_work_area(tray_hwnd);
        let (x, y) =
            calculate_flyout_geometry(icon_rect, fallback_edge, work_area, width, height, margin);

        unsafe {
            let _ = apply_window_chrome(flyout_hwnd, is_dark_mode_active());
            let _ = SetWindowPos(
                flyout_hwnd,
                Some(HWND_TOPMOST),
                x,
                y,
                width,
                height,
                SWP_SHOWWINDOW,
            );
            let _ = ShowWindow(flyout_hwnd, SW_SHOW);
            let _ = SetForegroundWindow(flyout_hwnd);
            let _ = InvalidateRect(Some(flyout_hwnd), None, false);
        }

        if let Ok(mut t) = ctx.open_time.lock() {
            *t = Instant::now();
        }
        *ctx.lifecycle.write_safe() = FlyoutLifecycleState::Visible;
    }
}

/// Safely retrieves `FlyoutContext` from `GWLP_USERDATA`.
unsafe fn get_flyout_context(hwnd: HWND) -> Option<&'static FlyoutContext> {
    unsafe {
        let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut FlyoutContext;
        if ptr.is_null() { None } else { Some(&*ptr) }
    }
}

/// Helper to render GDI text with UTF-16 conversion.
fn gdi_draw_text(hdc: HDC, text: &str, rect: &mut RECT, format: DRAW_TEXT_FORMAT) {
    let mut wide: Vec<u16> = text.encode_utf16().collect();
    unsafe {
        let _ = DrawTextW(hdc, &mut wide, rect, format);
    }
}

/// Helper to create a scaled UI font for GDI rendering.
fn make_font(size_dip: i32, weight: i32, dpi: u32) -> HFONT {
    unsafe {
        CreateFontW(
            -dpi_scale(size_dip, dpi),
            0,
            0,
            0,
            weight,
            0,
            0,
            0,
            FONT_CHARSET(1),
            FONT_OUTPUT_PRECISION(0),
            FONT_CLIP_PRECISION(0),
            FONT_QUALITY(5),
            0,
            w!("Segoe UI"),
        )
    }
}

/// Pure GDI double-buffered rendering implementation.
fn render_flyout(
    hdc: HDC,
    width: i32,
    height: i32,
    snapshot: &FlyoutSnapshot,
    ctx: &FlyoutContext,
) {
    let is_dark = is_dark_mode_active();
    let dpi = ctx.dpi.load(Ordering::SeqCst);

    // Color definitions
    let bg_color = if is_dark {
        rgb(24, 24, 27)
    } else {
        rgb(248, 249, 250)
    };
    let card_bg = if is_dark {
        rgb(36, 36, 40)
    } else {
        rgb(255, 255, 255)
    };
    let card_border = if is_dark {
        rgb(54, 54, 60)
    } else {
        rgb(226, 232, 240)
    };
    let text_primary = if is_dark {
        rgb(244, 244, 245)
    } else {
        rgb(15, 23, 42)
    };
    let text_secondary = if is_dark {
        rgb(161, 161, 170)
    } else {
        rgb(100, 116, 139)
    };
    let text_muted = if is_dark {
        rgb(113, 113, 122)
    } else {
        rgb(148, 163, 184)
    };
    let rx_green = if is_dark {
        rgb(16, 185, 129)
    } else {
        rgb(5, 150, 105)
    };
    let tx_blue = if is_dark {
        rgb(59, 130, 246)
    } else {
        rgb(37, 99, 235)
    };
    let warn_amber = rgb(245, 158, 11);
    let alert_red = rgb(239, 68, 68);

    unsafe {
        // 1. Fill main background
        let bg_brush = CreateSolidBrush(bg_color);
        let window_rect = RECT {
            left: 0,
            top: 0,
            right: width,
            bottom: height,
        };
        FillRect(hdc, &window_rect, bg_brush);
        let _ = DeleteObject(bg_brush.into());

        SetBkMode(hdc, TRANSPARENT);

        // Fonts
        let font_title = make_font(14, 600, dpi);
        let font_body = make_font(11, 400, dpi);
        let font_small = make_font(10, 400, dpi);
        let font_speed = make_font(17, 700, dpi);

        let margin_x = dpi_scale(12, dpi);
        let content_w = width - (margin_x * 2);

        // --- SECTION 1: Header (Net Flow + Medium) ---
        let mut header_rect = RECT {
            left: margin_x,
            top: dpi_scale(12, dpi),
            right: margin_x + dpi_scale(120, dpi),
            bottom: dpi_scale(34, dpi),
        };
        let old_font = SelectObject(hdc, font_title.into());
        SetTextColor(hdc, text_primary);
        gdi_draw_text(
            hdc,
            "Net Flow",
            &mut header_rect,
            DT_LEFT | DT_VCENTER | DT_SINGLELINE,
        );

        let medium_str = match snapshot.primary_medium {
            net_flow_core::backend::InterfaceMedium::Ethernet => "Ethernet",
            net_flow_core::backend::InterfaceMedium::Wifi => "Wi-Fi",
            net_flow_core::backend::InterfaceMedium::Loopback => "Loopback",
            net_flow_core::backend::InterfaceMedium::Cellular => "Cellular",
            _ => "Network",
        };
        let medium_label = if let Some(summary) = &snapshot.physical_link_summary {
            format!("{medium_str} • {summary}")
        } else {
            medium_str.to_string()
        };
        let mut med_rect = RECT {
            left: margin_x + dpi_scale(110, dpi),
            top: dpi_scale(12, dpi),
            right: width - margin_x,
            bottom: dpi_scale(34, dpi),
        };
        SelectObject(hdc, font_small.into());
        SetTextColor(hdc, text_secondary);
        gdi_draw_text(
            hdc,
            &medium_label,
            &mut med_rect,
            DT_RIGHT | DT_VCENTER | DT_SINGLELINE | DT_END_ELLIPSIS,
        );

        // --- SECTION 2: Download / Upload Cards Side-by-Side ---
        let card_gap = dpi_scale(8, dpi);
        let col_w = (content_w - card_gap) / 2;
        let card_y = dpi_scale(38, dpi);
        let card_h = dpi_scale(64, dpi);

        let card_brush = CreateSolidBrush(card_bg);
        let card_pen = CreatePen(PS_SOLID, 1, card_border);

        // Download Card
        let old_brush = SelectObject(hdc, card_brush.into());
        let old_pen = SelectObject(hdc, card_pen.into());
        let _ = RoundRect(
            hdc,
            margin_x,
            card_y,
            margin_x + col_w,
            card_y + card_h,
            dpi_scale(6, dpi),
            dpi_scale(6, dpi),
        );

        // Upload Card
        let _ = RoundRect(
            hdc,
            margin_x + col_w + card_gap,
            card_y,
            width - margin_x,
            card_y + card_h,
            dpi_scale(6, dpi),
            dpi_scale(6, dpi),
        );

        // Download texts
        let mut dl_lbl = RECT {
            left: margin_x + dpi_scale(10, dpi),
            top: card_y + dpi_scale(6, dpi),
            right: margin_x + col_w - dpi_scale(10, dpi),
            bottom: card_y + dpi_scale(24, dpi),
        };
        SelectObject(hdc, font_small.into());
        SetTextColor(hdc, rx_green);
        gdi_draw_text(
            hdc,
            "DOWNLOAD ↓",
            &mut dl_lbl,
            DT_LEFT | DT_VCENTER | DT_SINGLELINE,
        );

        let dl_val = net_flow_core::format::format_bandwidth(snapshot.rx_bps);
        let mut dl_val_rect = RECT {
            left: margin_x + dpi_scale(10, dpi),
            top: card_y + dpi_scale(26, dpi),
            right: margin_x + col_w - dpi_scale(10, dpi),
            bottom: card_y + card_h - dpi_scale(6, dpi),
        };
        SelectObject(hdc, font_speed.into());
        SetTextColor(hdc, text_primary);
        gdi_draw_text(
            hdc,
            &dl_val,
            &mut dl_val_rect,
            DT_LEFT | DT_VCENTER | DT_SINGLELINE,
        );

        // Upload texts
        let mut ul_lbl = RECT {
            left: margin_x + col_w + card_gap + dpi_scale(10, dpi),
            top: card_y + dpi_scale(6, dpi),
            right: width - margin_x - dpi_scale(10, dpi),
            bottom: card_y + dpi_scale(24, dpi),
        };
        SelectObject(hdc, font_small.into());
        SetTextColor(hdc, tx_blue);
        gdi_draw_text(
            hdc,
            "UPLOAD ↑",
            &mut ul_lbl,
            DT_LEFT | DT_VCENTER | DT_SINGLELINE,
        );

        let ul_val = net_flow_core::format::format_bandwidth(snapshot.tx_bps);
        let mut ul_val_rect = RECT {
            left: margin_x + col_w + card_gap + dpi_scale(10, dpi),
            top: card_y + dpi_scale(26, dpi),
            right: width - margin_x - dpi_scale(10, dpi),
            bottom: card_y + card_h - dpi_scale(6, dpi),
        };
        SelectObject(hdc, font_speed.into());
        SetTextColor(hdc, text_primary);
        gdi_draw_text(
            hdc,
            &ul_val,
            &mut ul_val_rect,
            DT_LEFT | DT_VCENTER | DT_SINGLELINE,
        );

        // --- SECTION 3: Latency & Health Pill ---
        let pill_y = card_y + card_h + dpi_scale(8, dpi);
        let pill_h = dpi_scale(28, dpi);

        let _ = RoundRect(
            hdc,
            margin_x,
            pill_y,
            width - margin_x,
            pill_y + pill_h,
            dpi_scale(6, dpi),
            dpi_scale(6, dpi),
        );

        let (health_col, health_str) = match snapshot.latency_health {
            net_flow_core::backend::LatencyHealth::Healthy => (rx_green, "Healthy"),
            net_flow_core::backend::LatencyHealth::Degraded => (warn_amber, "Degraded"),
            net_flow_core::backend::LatencyHealth::Timeout => (alert_red, "Timeout"),
            _ => (text_secondary, "Unavailable"),
        };

        // Health dot
        let dot_r = dpi_scale(4, dpi);
        let dot_cx = margin_x + dpi_scale(12, dpi);
        let dot_cy = pill_y + (pill_h / 2);
        let dot_brush = CreateSolidBrush(health_col);
        let dot_pen = CreatePen(PS_SOLID, 1, health_col);
        let prev_b = SelectObject(hdc, dot_brush.into());
        let prev_p = SelectObject(hdc, dot_pen.into());
        let _ = RoundRect(
            hdc,
            dot_cx - dot_r,
            dot_cy - dot_r,
            dot_cx + dot_r,
            dot_cy + dot_r,
            dot_r * 2,
            dot_r * 2,
        );
        SelectObject(hdc, prev_b);
        SelectObject(hdc, prev_p);
        let _ = DeleteObject(dot_brush.into());
        let _ = DeleteObject(dot_pen.into());

        let lat_txt = if let Some(ms) = snapshot.latency_ms {
            let j = snapshot.jitter_ms.unwrap_or(0);
            format!(
                "{ms} ms (±{j}ms) • {:.0}% loss • {health_str}",
                snapshot.packet_loss_pct
            )
        } else {
            "ICMP Diagnostics Active".to_string()
        };
        let mut pill_rect = RECT {
            left: margin_x + dpi_scale(24, dpi),
            top: pill_y,
            right: width - margin_x - dpi_scale(8, dpi),
            bottom: pill_y + pill_h,
        };
        SelectObject(hdc, font_small.into());
        SetTextColor(hdc, text_secondary);
        gdi_draw_text(
            hdc,
            &lat_txt,
            &mut pill_rect,
            DT_LEFT | DT_VCENTER | DT_SINGLELINE | DT_END_ELLIPSIS,
        );

        // --- SECTION 4: Live Sparkline Card ---
        let spark_y = pill_y + pill_h + dpi_scale(8, dpi);
        let spark_h = dpi_scale(80, dpi);

        let _ = RoundRect(
            hdc,
            margin_x,
            spark_y,
            width - margin_x,
            spark_y + spark_h,
            dpi_scale(6, dpi),
            dpi_scale(6, dpi),
        );

        let mut spark_lbl = RECT {
            left: margin_x + dpi_scale(10, dpi),
            top: spark_y + dpi_scale(5, dpi),
            right: width - margin_x - dpi_scale(10, dpi),
            bottom: spark_y + dpi_scale(18, dpi),
        };
        SetTextColor(hdc, text_muted);
        gdi_draw_text(
            hdc,
            "TRAFFIC ACTIVITY (LAST 30s)",
            &mut spark_lbl,
            DT_LEFT | DT_VCENTER | DT_SINGLELINE,
        );

        // Draw sparklines
        let chart_l = margin_x + dpi_scale(10, dpi);
        let chart_r = width - margin_x - dpi_scale(10, dpi);
        let chart_t = spark_y + dpi_scale(22, dpi);
        let chart_b = spark_y + spark_h - dpi_scale(8, dpi);
        let chart_w = (chart_r - chart_l).max(1);
        let chart_ch = (chart_b - chart_t).max(1);

        // Subtle baseline
        let grid_pen = CreatePen(PS_SOLID, 1, card_border);
        let prev_grid = SelectObject(hdc, grid_pen.into());
        let _ = MoveToEx(hdc, chart_l, chart_b, None);
        let _ = LineTo(hdc, chart_r, chart_b);
        SelectObject(hdc, prev_grid);
        let _ = DeleteObject(grid_pen.into());

        if !snapshot.sparkline_history.is_empty() {
            let n = snapshot.sparkline_history.len();
            let mut rx_pts: Vec<POINT> = Vec::with_capacity(n);
            let mut tx_pts: Vec<POINT> = Vec::with_capacity(n);

            for (i, (rx_norm, tx_norm)) in snapshot.sparkline_history.iter().enumerate() {
                let px = chart_l + ((i as i32 * chart_w) / (n.max(2) - 1) as i32);
                let py_rx = chart_b - (*rx_norm * chart_ch as f32).round() as i32;
                let py_tx = chart_b - (*tx_norm * chart_ch as f32).round() as i32;
                rx_pts.push(POINT { x: px, y: py_rx });
                tx_pts.push(POINT { x: px, y: py_tx });
            }

            let rx_pen = CreatePen(PS_SOLID, 2, rx_green);
            let prev_rx = SelectObject(hdc, rx_pen.into());
            let _ = Polyline(hdc, &rx_pts);
            SelectObject(hdc, prev_rx);
            let _ = DeleteObject(rx_pen.into());

            let tx_pen = CreatePen(PS_SOLID, 1, tx_blue);
            let prev_tx = SelectObject(hdc, tx_pen.into());
            let _ = Polyline(hdc, &tx_pts);
            SelectObject(hdc, prev_tx);
            let _ = DeleteObject(tx_pen.into());
        }

        // --- SECTION 5: Session & Budget Card ---
        let sess_y = spark_y + spark_h + dpi_scale(8, dpi);
        let sess_h = dpi_scale(54, dpi);

        let _ = RoundRect(
            hdc,
            margin_x,
            sess_y,
            width - margin_x,
            sess_y + sess_h,
            dpi_scale(6, dpi),
            dpi_scale(6, dpi),
        );

        let dur_str = format_duration_short(snapshot.session_duration_secs);
        let rx_tot = net_flow_core::format::format_bytes(snapshot.session_rx_bytes);
        let tx_tot = net_flow_core::format::format_bytes(snapshot.session_tx_bytes);
        let sess_txt = format!("Session ({dur_str}): {rx_tot} ↓   {tx_tot} ↑");

        let mut sess_rect = RECT {
            left: margin_x + dpi_scale(10, dpi),
            top: sess_y + dpi_scale(6, dpi),
            right: width - margin_x - dpi_scale(10, dpi),
            bottom: sess_y + dpi_scale(22, dpi),
        };
        SelectObject(hdc, font_small.into());
        SetTextColor(hdc, text_primary);
        gdi_draw_text(
            hdc,
            &sess_txt,
            &mut sess_rect,
            DT_LEFT | DT_VCENTER | DT_SINGLELINE,
        );

        if let Some(pct) = snapshot.budget_usage_pct {
            let cap_str = net_flow_core::format::format_bytes(snapshot.budget_cap_bytes);
            let used_str = net_flow_core::format::format_bytes(snapshot.budget_used_bytes);
            let days_str = snapshot
                .budget_days_left
                .map(|d| format!("{d}d left"))
                .unwrap_or_default();
            let bgt_txt = format!("Budget: {used_str} / {cap_str} ({pct}%) {days_str}");

            let mut bgt_rect = RECT {
                left: margin_x + dpi_scale(10, dpi),
                top: sess_y + dpi_scale(24, dpi),
                right: width - margin_x - dpi_scale(10, dpi),
                bottom: sess_y + dpi_scale(38, dpi),
            };
            SetTextColor(hdc, text_secondary);
            gdi_draw_text(
                hdc,
                &bgt_txt,
                &mut bgt_rect,
                DT_LEFT | DT_VCENTER | DT_SINGLELINE,
            );

            // Progress bar
            let bar_l = margin_x + dpi_scale(10, dpi);
            let bar_r = width - margin_x - dpi_scale(10, dpi);
            let bar_t = sess_y + dpi_scale(42, dpi);
            let bar_b = sess_y + dpi_scale(46, dpi);
            let fill_w = (((bar_r - bar_l) as f32 * (pct as f32 / 100.0)).round() as i32)
                .clamp(0, bar_r - bar_l);

            let bar_track_brush = CreateSolidBrush(card_border);
            let bar_fill_col = if pct >= 100 {
                alert_red
            } else if pct >= 80 {
                warn_amber
            } else {
                rx_green
            };
            let bar_fill_brush = CreateSolidBrush(bar_fill_col);

            let track_rc = RECT {
                left: bar_l,
                top: bar_t,
                right: bar_r,
                bottom: bar_b,
            };
            FillRect(hdc, &track_rc, bar_track_brush);
            let fill_rc = RECT {
                left: bar_l,
                top: bar_t,
                right: bar_l + fill_w,
                bottom: bar_b,
            };
            FillRect(hdc, &fill_rc, bar_fill_brush);

            let _ = DeleteObject(bar_track_brush.into());
            let _ = DeleteObject(bar_fill_brush.into());
        } else {
            let mut sub_rect = RECT {
                left: margin_x + dpi_scale(10, dpi),
                top: sess_y + dpi_scale(26, dpi),
                right: width - margin_x - dpi_scale(10, dpi),
                bottom: sess_y + dpi_scale(44, dpi),
            };
            SetTextColor(hdc, text_muted);
            gdi_draw_text(
                hdc,
                "Continuous physical network telemetry active",
                &mut sub_rect,
                DT_LEFT | DT_VCENTER | DT_SINGLELINE,
            );
        }

        // --- SECTION 6: Top Network Apps Card ---
        let app_y = sess_y + sess_h + dpi_scale(8, dpi);
        let app_h = dpi_scale(76, dpi);

        let _ = RoundRect(
            hdc,
            margin_x,
            app_y,
            width - margin_x,
            app_y + app_h,
            dpi_scale(6, dpi),
            dpi_scale(6, dpi),
        );

        let mut app_lbl = RECT {
            left: margin_x + dpi_scale(10, dpi),
            top: app_y + dpi_scale(5, dpi),
            right: width - margin_x - dpi_scale(10, dpi),
            bottom: app_y + dpi_scale(19, dpi),
        };
        SetTextColor(hdc, text_muted);
        gdi_draw_text(
            hdc,
            "TOP NETWORK CONSUMERS",
            &mut app_lbl,
            DT_LEFT | DT_VCENTER | DT_SINGLELINE,
        );

        if snapshot.active_apps.is_empty() {
            let mut empty_rc = RECT {
                left: margin_x + dpi_scale(10, dpi),
                top: app_y + dpi_scale(28, dpi),
                right: width - margin_x - dpi_scale(10, dpi),
                bottom: app_y + dpi_scale(48, dpi),
            };
            SetTextColor(hdc, text_secondary);
            gdi_draw_text(
                hdc,
                "No per-process network activity detected",
                &mut empty_rc,
                DT_LEFT | DT_VCENTER | DT_SINGLELINE,
            );
        } else {
            let row_h = dpi_scale(16, dpi);
            for (idx, app) in snapshot.active_apps.iter().take(3).enumerate() {
                let ry = app_y + dpi_scale(22, dpi) + (idx as i32 * row_h);
                let mut name_rc = RECT {
                    left: margin_x + dpi_scale(10, dpi),
                    top: ry,
                    right: margin_x + dpi_scale(150, dpi),
                    bottom: ry + row_h,
                };
                SelectObject(hdc, font_small.into());
                SetTextColor(hdc, text_primary);
                gdi_draw_text(
                    hdc,
                    &app.name,
                    &mut name_rc,
                    DT_LEFT | DT_VCENTER | DT_SINGLELINE | DT_END_ELLIPSIS,
                );

                let rx_str = net_flow_core::format::format_bandwidth(app.rx_bps as f64);
                let tx_str = net_flow_core::format::format_bandwidth(app.tx_bps as f64);
                let rate_txt = format!("{rx_str} ↓  {tx_str} ↑");
                let mut rate_rc = RECT {
                    left: margin_x + dpi_scale(150, dpi),
                    top: ry,
                    right: width - margin_x - dpi_scale(10, dpi),
                    bottom: ry + row_h,
                };
                SetTextColor(hdc, text_secondary);
                gdi_draw_text(
                    hdc,
                    &rate_txt,
                    &mut rate_rc,
                    DT_RIGHT | DT_VCENTER | DT_SINGLELINE,
                );
            }
        }

        // --- SECTION 7: Action Buttons (3-button balanced bar) ---
        let btn_y = app_y + app_h + dpi_scale(8, dpi);
        let btn_h = dpi_scale(30, dpi);
        let btn_w = (content_w - card_gap * 2) / 3;

        let hover = ctx.hover_button.load(Ordering::SeqCst);

        let export_rc = RECT {
            left: margin_x,
            top: btn_y,
            right: margin_x + btn_w,
            bottom: btn_y + btn_h,
        };
        let reset_rc = RECT {
            left: margin_x + btn_w + card_gap,
            top: btn_y,
            right: margin_x + btn_w * 2 + card_gap,
            bottom: btn_y + btn_h,
        };
        let close_rc = RECT {
            left: margin_x + (btn_w + card_gap) * 2,
            top: btn_y,
            right: width - margin_x,
            bottom: btn_y + btn_h,
        };

        if let Ok(mut r) = ctx.export_rect.lock() {
            *r = export_rc;
        }
        if let Ok(mut r) = ctx.reset_rect.lock() {
            *r = reset_rc;
        }
        if let Ok(mut r) = ctx.close_rect.lock() {
            *r = close_rc;
        }

        // Export Button (hover = 1)
        let export_bg = if hover == 1 {
            if is_dark {
                rgb(56, 56, 64)
            } else {
                rgb(226, 232, 240)
            }
        } else {
            card_bg
        };
        let export_brush = CreateSolidBrush(export_bg);
        let b0 = SelectObject(hdc, export_brush.into());
        let _ = RoundRect(
            hdc,
            export_rc.left,
            export_rc.top,
            export_rc.right,
            export_rc.bottom,
            dpi_scale(6, dpi),
            dpi_scale(6, dpi),
        );
        SelectObject(hdc, b0);
        let _ = DeleteObject(export_brush.into());

        let mut e_txt_rc = export_rc;
        SelectObject(hdc, font_body.into());
        SetTextColor(hdc, text_primary);
        gdi_draw_text(
            hdc,
            "Export",
            &mut e_txt_rc,
            DT_CENTER | DT_VCENTER | DT_SINGLELINE,
        );

        // Reset Button (hover = 2)
        let reset_bg = if hover == 2 {
            if is_dark {
                rgb(56, 56, 64)
            } else {
                rgb(226, 232, 240)
            }
        } else {
            card_bg
        };
        let reset_brush = CreateSolidBrush(reset_bg);
        let b1 = SelectObject(hdc, reset_brush.into());
        let _ = RoundRect(
            hdc,
            reset_rc.left,
            reset_rc.top,
            reset_rc.right,
            reset_rc.bottom,
            dpi_scale(6, dpi),
            dpi_scale(6, dpi),
        );
        SelectObject(hdc, b1);
        let _ = DeleteObject(reset_brush.into());

        let mut r_txt_rc = reset_rc;
        SelectObject(hdc, font_body.into());
        SetTextColor(hdc, text_primary);
        gdi_draw_text(
            hdc,
            "Reset",
            &mut r_txt_rc,
            DT_CENTER | DT_VCENTER | DT_SINGLELINE,
        );

        // Close Button (hover = 3)
        let close_bg = if hover == 3 {
            if is_dark {
                rgb(56, 56, 64)
            } else {
                rgb(226, 232, 240)
            }
        } else {
            card_bg
        };
        let close_brush = CreateSolidBrush(close_bg);
        let b2 = SelectObject(hdc, close_brush.into());
        let _ = RoundRect(
            hdc,
            close_rc.left,
            close_rc.top,
            close_rc.right,
            close_rc.bottom,
            dpi_scale(6, dpi),
            dpi_scale(6, dpi),
        );
        SelectObject(hdc, b2);
        let _ = DeleteObject(close_brush.into());

        let mut c_txt_rc = close_rc;
        SelectObject(hdc, font_body.into());
        SetTextColor(hdc, text_primary);
        gdi_draw_text(
            hdc,
            "Close",
            &mut c_txt_rc,
            DT_CENTER | DT_VCENTER | DT_SINGLELINE,
        );

        // Cleanup GDI objects
        SelectObject(hdc, old_brush);
        SelectObject(hdc, old_pen);
        SelectObject(hdc, old_font);

        let _ = DeleteObject(card_brush.into());
        let _ = DeleteObject(card_pen.into());
        let _ = DeleteObject(font_title.into());
        let _ = DeleteObject(font_body.into());
        let _ = DeleteObject(font_small.into());
        let _ = DeleteObject(font_speed.into());
    }
}

/// Window procedure for the flyout window.
unsafe extern "system" fn flyout_wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_NCCREATE => unsafe {
            let cs = lparam.0 as *const CREATESTRUCTW;
            if !cs.is_null() {
                let ctx_ptr = (*cs).lpCreateParams as isize;
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, ctx_ptr);
            }
            DefWindowProcW(hwnd, msg, wparam, lparam)
        },
        WM_PAINT => {
            unsafe {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);
                if !hdc.is_invalid() {
                    let mut client_rect = RECT::default();
                    let _ = GetClientRect(hwnd, &mut client_rect);
                    let width = client_rect.right - client_rect.left;
                    let height = client_rect.bottom - client_rect.top;

                    if width > 0
                        && height > 0
                        && let Some(ctx) = get_flyout_context(hwnd)
                    {
                        let snapshot = ctx.snapshot.read_safe().clone();
                        let mem_dc = CreateCompatibleDC(Some(hdc));
                        let mem_bmp = CreateCompatibleBitmap(hdc, width, height);
                        let old_bmp = SelectObject(mem_dc, mem_bmp.into());

                        render_flyout(mem_dc, width, height, &snapshot, ctx);

                        let _ = BitBlt(hdc, 0, 0, width, height, Some(mem_dc), 0, 0, SRCCOPY);

                        SelectObject(mem_dc, old_bmp);
                        let _ = DeleteObject(mem_bmp.into());
                        let _ = DeleteDC(mem_dc);
                    }
                    let _ = EndPaint(hwnd, &ps);
                }
            }
            LRESULT(0)
        }
        WM_FLYOUT_SNAPSHOT_UPDATED => {
            unsafe {
                let _ = InvalidateRect(Some(hwnd), None, false);
            }
            LRESULT(0)
        }
        WM_ACTIVATE => {
            let activation = (wparam.0 & 0xffff) as u32;
            if activation == WA_INACTIVE {
                unsafe {
                    if let Some(ctx) = get_flyout_context(hwnd) {
                        let elapsed = ctx
                            .open_time
                            .lock()
                            .map(|t| t.elapsed())
                            .unwrap_or_default();
                        if elapsed.as_millis() > 150 {
                            let cur = *ctx.lifecycle.read_safe();
                            if cur == FlyoutLifecycleState::Visible {
                                *ctx.lifecycle.write_safe() = FlyoutLifecycleState::Closing;
                                let _ = ShowWindow(hwnd, SW_HIDE);
                                *ctx.lifecycle.write_safe() = FlyoutLifecycleState::Hidden;
                            }
                        }
                    }
                }
            }
            LRESULT(0)
        }
        WM_KILLFOCUS => {
            unsafe {
                if let Some(ctx) = get_flyout_context(hwnd) {
                    let elapsed = ctx
                        .open_time
                        .lock()
                        .map(|t| t.elapsed())
                        .unwrap_or_default();
                    if elapsed.as_millis() > 150 {
                        let cur = *ctx.lifecycle.read_safe();
                        if cur == FlyoutLifecycleState::Visible {
                            *ctx.lifecycle.write_safe() = FlyoutLifecycleState::Closing;
                            let _ = ShowWindow(hwnd, SW_HIDE);
                            *ctx.lifecycle.write_safe() = FlyoutLifecycleState::Hidden;
                        }
                    }
                }
            }
            LRESULT(0)
        }
        WM_DPICHANGED => {
            let new_dpi = (wparam.0 & 0xffff) as u32;
            unsafe {
                if let Some(ctx) = get_flyout_context(hwnd) {
                    ctx.dpi.store(new_dpi, Ordering::SeqCst);
                    let lprc = lparam.0 as *const RECT;
                    if !lprc.is_null() {
                        let r = *lprc;
                        let _ = SetWindowPos(
                            hwnd,
                            Some(HWND_TOPMOST),
                            r.left,
                            r.top,
                            r.right - r.left,
                            r.bottom - r.top,
                            SWP_NOACTIVATE | SWP_NOZORDER,
                        );
                    }
                    let _ = InvalidateRect(Some(hwnd), None, false);
                }
            }
            LRESULT(0)
        }
        WM_MOUSEMOVE => {
            let x = (lparam.0 & 0xffff) as i16 as i32;
            let y = ((lparam.0 >> 16) & 0xffff) as i16 as i32;
            let pt = POINT { x, y };

            unsafe {
                if let Some(ctx) = get_flyout_context(hwnd) {
                    if !ctx.tracking_mouse.swap(true, Ordering::SeqCst) {
                        let mut tme = TRACKMOUSEEVENT {
                            cbSize: std::mem::size_of::<TRACKMOUSEEVENT>() as u32,
                            dwFlags: TME_LEAVE,
                            hwndTrack: hwnd,
                            dwHoverTime: 0,
                        };
                        let _ = TrackMouseEvent(&mut tme);
                    }

                    let mut new_hover = 0;
                    if ctx.export_rect.lock().is_ok_and(|r| pt_in_rect(&r, pt)) {
                        new_hover = 1;
                    } else if ctx.reset_rect.lock().is_ok_and(|r| pt_in_rect(&r, pt)) {
                        new_hover = 2;
                    } else if ctx.close_rect.lock().is_ok_and(|r| pt_in_rect(&r, pt)) {
                        new_hover = 3;
                    }

                    let old_hover = ctx.hover_button.swap(new_hover, Ordering::SeqCst);
                    if old_hover != new_hover {
                        let _ = InvalidateRect(Some(hwnd), None, false);
                    }
                }
            }
            LRESULT(0)
        }
        WM_MOUSELEAVE => {
            unsafe {
                if let Some(ctx) = get_flyout_context(hwnd) {
                    ctx.tracking_mouse.store(false, Ordering::SeqCst);
                    let old_hover = ctx.hover_button.swap(0, Ordering::SeqCst);
                    if old_hover != 0 {
                        let _ = InvalidateRect(Some(hwnd), None, false);
                    }
                }
            }
            LRESULT(0)
        }
        WM_LBUTTONUP => {
            let x = (lparam.0 & 0xffff) as i16 as i32;
            let y = ((lparam.0 >> 16) & 0xffff) as i16 as i32;
            let pt = POINT { x, y };

            unsafe {
                if let Some(ctx) = get_flyout_context(hwnd) {
                    let export_clicked = ctx
                        .export_rect
                        .lock()
                        .map(|r| pt_in_rect(&r, pt))
                        .unwrap_or(false);
                    let reset_clicked = ctx
                        .reset_rect
                        .lock()
                        .map(|r| pt_in_rect(&r, pt))
                        .unwrap_or(false);
                    let close_clicked = ctx
                        .close_rect
                        .lock()
                        .map(|r| pt_in_rect(&r, pt))
                        .unwrap_or(false);

                    if export_clicked {
                        (ctx.on_export)(hwnd);
                    } else if reset_clicked {
                        (ctx.on_reset)();
                        let _ = InvalidateRect(Some(hwnd), None, false);
                    } else if close_clicked {
                        *ctx.lifecycle.write_safe() = FlyoutLifecycleState::Closing;
                        let _ = ShowWindow(hwnd, SW_HIDE);
                        *ctx.lifecycle.write_safe() = FlyoutLifecycleState::Hidden;
                    }
                }
            }
            LRESULT(0)
        }

        WM_NCDESTROY => unsafe {
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut FlyoutContext;
            if !ptr.is_null() {
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
                drop(Box::from_raw(ptr));
            }
            DefWindowProcW(hwnd, msg, wparam, lparam)
        },
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_flyout_geometry_bottom_taskbar() {
        let work_area = RECT {
            left: 0,
            top: 0,
            right: 1920,
            bottom: 1040,
        };
        let icon_rect = Some(RECT {
            left: 1800,
            top: 1040,
            right: 1832,
            bottom: 1080,
        });
        let flyout_w = 328;
        let flyout_h = 456;
        let margin = 8;

        let (x, y) = calculate_flyout_geometry(
            icon_rect,
            TaskbarEdge::Bottom,
            work_area,
            flyout_w,
            flyout_h,
            margin,
        );

        // Clamped inside work area
        assert!(x + flyout_w <= work_area.right);
        assert!(x >= work_area.left);
        assert!(y + flyout_h <= work_area.bottom);
        assert!(y >= work_area.top);
        assert_eq!(y, 1040 - flyout_h - margin);
    }

    #[test]
    fn test_calculate_flyout_geometry_top_taskbar() {
        let work_area = RECT {
            left: 0,
            top: 40,
            right: 1920,
            bottom: 1080,
        };
        let icon_rect = Some(RECT {
            left: 1800,
            top: 0,
            right: 1832,
            bottom: 40,
        });
        let (x, y) = calculate_flyout_geometry(icon_rect, TaskbarEdge::Top, work_area, 328, 456, 8);

        assert!(x + 328 <= work_area.right);
        assert!(y >= work_area.top);
        assert_eq!(y, 40 + 8);
    }

    #[test]
    fn test_calculate_flyout_geometry_vertical_taskbars() {
        // Left taskbar
        let work_area_left = RECT {
            left: 60,
            top: 0,
            right: 1920,
            bottom: 1080,
        };
        let icon_rect_left = Some(RECT {
            left: 0,
            top: 1000,
            right: 60,
            bottom: 1032,
        });
        let (x, y) = calculate_flyout_geometry(
            icon_rect_left,
            TaskbarEdge::Left,
            work_area_left,
            328,
            456,
            8,
        );
        assert_eq!(x, 60 + 8);
        assert!(y + 456 <= work_area_left.bottom);

        // Right taskbar
        let work_area_right = RECT {
            left: 0,
            top: 0,
            right: 1860,
            bottom: 1080,
        };
        let icon_rect_right = Some(RECT {
            left: 1860,
            top: 1000,
            right: 1920,
            bottom: 1032,
        });
        let (x, y) = calculate_flyout_geometry(
            icon_rect_right,
            TaskbarEdge::Right,
            work_area_right,
            328,
            456,
            8,
        );
        assert_eq!(x, 1860 - 328 - 8);
        assert!(y + 456 <= work_area_right.bottom);
    }

    #[test]
    fn test_calculate_flyout_geometry_secondary_monitor_clamping() {
        let secondary_work_area = RECT {
            left: 1920,
            top: 0,
            right: 3840,
            bottom: 1040,
        };
        let icon_rect = Some(RECT {
            left: 3800,
            top: 1040,
            right: 3832,
            bottom: 1080,
        });
        let (x, y) = calculate_flyout_geometry(
            icon_rect,
            TaskbarEdge::Bottom,
            secondary_work_area,
            328,
            456,
            8,
        );

        assert!(x >= secondary_work_area.left);
        assert!(x + 328 <= secondary_work_area.right);
        assert_eq!(x, 3840 - 328 - 8);
        assert_eq!(y, 1040 - 456 - 8);
    }

    #[test]
    fn test_dpi_scale_values() {
        assert_eq!(dpi_scale(328, 96), 328);
        assert_eq!(dpi_scale(328, 144), 492);
        assert_eq!(dpi_scale(328, 192), 656);
        assert_eq!(dpi_scale(0, 96), 0);
    }

    #[test]
    fn test_format_duration_short_cases() {
        assert_eq!(format_duration_short(15), "15s");
        assert_eq!(format_duration_short(125), "2m 5s");
        assert_eq!(format_duration_short(3660), "1h 1m");
        assert_eq!(format_duration_short(7325), "2h 2m");
    }

    #[test]
    fn test_pt_in_rect_hit_test() {
        let rc = RECT {
            left: 10,
            top: 20,
            right: 100,
            bottom: 50,
        };
        assert!(pt_in_rect(&rc, POINT { x: 10, y: 20 }));
        assert!(pt_in_rect(&rc, POINT { x: 50, y: 35 }));
        assert!(!pt_in_rect(&rc, POINT { x: 9, y: 20 }));
        assert!(!pt_in_rect(&rc, POINT { x: 100, y: 35 }));
        assert!(!pt_in_rect(&rc, POINT { x: 50, y: 50 }));
    }

    #[test]
    fn test_lifecycle_state_machine() {
        let state = Arc::new(RwLock::new(FlyoutLifecycleState::Hidden));
        assert_eq!(*state.read_safe(), FlyoutLifecycleState::Hidden);

        *state.write_safe() = FlyoutLifecycleState::Opening;
        assert_eq!(*state.read_safe(), FlyoutLifecycleState::Opening);

        *state.write_safe() = FlyoutLifecycleState::Visible;
        assert_eq!(*state.read_safe(), FlyoutLifecycleState::Visible);

        *state.write_safe() = FlyoutLifecycleState::Closing;
        assert_eq!(*state.read_safe(), FlyoutLifecycleState::Closing);

        *state.write_safe() = FlyoutLifecycleState::Hidden;
        assert_eq!(*state.read_safe(), FlyoutLifecycleState::Hidden);
    }

    #[test]
    fn test_flyout_snapshot_100_cycle_stability() {
        let snapshot_store = Arc::new(RwLock::new(FlyoutSnapshot::default()));
        for cycle in 0..100 {
            let snap = FlyoutSnapshot {
                rx_bps: cycle as f64 * 1024.0,
                tx_bps: cycle as f64 * 512.0,
                latency_ms: Some(15 + (cycle as u32 % 5)),
                jitter_ms: Some(2),
                packet_loss_pct: 0.0,
                latency_health: net_flow_core::backend::LatencyHealth::Healthy,
                latency_target_label: "Balanced".to_string(),
                primary_medium: net_flow_core::backend::InterfaceMedium::Ethernet,
                physical_link_summary: Some("1.0 Gbps".to_string()),
                session_rx_bytes: cycle * 10_000,
                session_tx_bytes: cycle * 5_000,
                session_duration_secs: cycle,
                budget_usage_pct: Some(15),
                budget_days_left: Some(20),
                budget_cap_bytes: 100_000_000,
                budget_used_bytes: 15_500_000,
                active_apps: vec![FlyoutAppItem {
                    name: "app.exe".to_string(),
                    rx_bps: 1000,
                    tx_bps: 500,
                }],
                sparkline_history: vec![(0.5, 0.2); 30],
            };
            *snapshot_store.write_safe() = snap;
        }

        let final_snap = snapshot_store.read_safe();
        assert_eq!(final_snap.session_duration_secs, 99);
        assert_eq!(final_snap.active_apps.len(), 1);
        assert_eq!(final_snap.sparkline_history.len(), 30);
    }
}
