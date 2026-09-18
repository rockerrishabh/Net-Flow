#![windows_subsystem = "windows"]

#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unused_variables,
    unused_qualifications,
    clippy::all,
    warnings
)]
mod bindings;
mod factory;
mod provider;

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use windows_core::{GUID, HRESULT, IUnknown, Interface};

use crate::bindings::Microsoft::Windows::Widgets::Providers::WidgetManager;
use crate::factory::NetFlowClassFactory;
use crate::provider::{
    InternalWidgetInfo, LockExt, NetFlowWidgetProvider, ProviderState, log_widget,
};
use net_flow_core::card::WidgetConfig;

// CLSID: {A8E4C976-3F5D-4B2E-9C1A-7D6E8F0B2A4C}
// Registered in appxmanifest as the out-of-process COM server for the widget
pub const CLSID_NET_FLOW_WIDGET_PROVIDER: GUID =
    GUID::from_u128(0xA8E4C976_3F5D_4B2E_9C1A_7D6E8F0B2A4C);

const COINIT_MULTITHREADED: u32 = 0x0;
const CLSCTX_LOCAL_SERVER: u32 = 0x4;
const REGCLS_MULTIPLEUSE: u32 = 0x1;

#[link(name = "ole32")]
unsafe extern "system" {
    fn CoInitializeEx(pv_reserved: *const core::ffi::c_void, dw_co_init: u32) -> HRESULT;
    fn CoRegisterClassObject(
        rclsid: *const GUID,
        p_unk: *mut core::ffi::c_void,
        dw_cls_context: u32,
        flags: u32,
        lpdw_register: *mut u32,
    ) -> HRESULT;
    fn CoRevokeClassObject(dw_register: u32) -> HRESULT;
    fn CoUninitialize();
}

fn main() -> windows_core::Result<()> {
    // 1. Initialize COM MTA (Multi-Threaded Apartment) for widget IPC
    unsafe {
        let hr = CoInitializeEx(core::ptr::null(), COINIT_MULTITHREADED);
        if hr.0 < 0 {
            return Err(windows_core::Error::from_hresult(hr));
        }
    }

    let state = Arc::new(Mutex::new(ProviderState::new()));

    // 2. Query WidgetManager to recover any widgets already pinned to the user's board
    if let Ok(manager) = WidgetManager::GetDefault()
        && let Ok(infos) = manager.GetWidgetInfos()
    {
        let mut s = state.lock_safe();
        for info in infos.as_slice().iter().flatten() {
            if let Ok(ctx) = info.WidgetContext()
                && let (Ok(id), Ok(size)) = (ctx.Id(), ctx.Size())
            {
                let id_str = id.to_string_lossy();
                let is_active = ctx.IsActive().unwrap_or(false);
                if is_active {
                    s.active_count += 1;
                }

                // Parse persisted config from CustomState
                let custom_state_str = info
                    .CustomState()
                    .map(|cs| cs.to_string_lossy())
                    .unwrap_or_default();
                let config: WidgetConfig = if custom_state_str.is_empty() {
                    WidgetConfig::default()
                } else {
                    serde_json::from_str(&custom_state_str).unwrap_or_default()
                };

                s.widgets.insert(
                    id_str.clone(),
                    InternalWidgetInfo {
                        id: id_str,
                        size,
                        is_active,
                        in_customization: false,
                        custom_state: config,
                        draft_state: None,
                        customization_requested_at: None,
                    },
                );
            }
        }
        s.has_had_widgets = !s.widgets.is_empty();
        s.last_empty_at = if s.widgets.is_empty() {
            Some(Instant::now())
        } else {
            None
        };
    }

    // 3. Spin up the background telemetry worker thread
    let provider_helper = NetFlowWidgetProvider::new(Arc::clone(&state));
    provider_helper.ensure_worker();

    // 4. Register the COM Class Factory with OLE so Windows can activate the widget provider
    let factory = NetFlowClassFactory::new(Arc::clone(&state));
    let factory_unk: IUnknown = factory.into();
    let mut registration_cookie: u32 = 0;

    unsafe {
        let hr = CoRegisterClassObject(
            &CLSID_NET_FLOW_WIDGET_PROVIDER,
            Interface::as_raw(&factory_unk),
            CLSCTX_LOCAL_SERVER,
            REGCLS_MULTIPLEUSE,
            &mut registration_cookie,
        );
        hr.ok()?;
    }

    // 5. Keep the server running until signaled or until an idle timeout expires
    let running = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = Arc::clone(&running);
    let _ = ctrlc_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    });

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        std::thread::sleep(Duration::from_millis(500));

        let (is_empty, has_had, last_empty) = {
            let s = state.lock_safe();
            (s.widgets.is_empty(), s.has_had_widgets, s.last_empty_at)
        };

        // If no widgets are pinned, shut down cleanly after a grace period (30-60s)
        // to avoid consuming background system resources
        if is_empty {
            let grace = if has_had {
                Duration::from_secs(30)
            } else {
                Duration::from_secs(60)
            };

            if let Some(t) = last_empty
                && t.elapsed() >= grace
            {
                log_widget("Idle timeout reached with 0 widgets; initiating clean shutdown");
                // Revoke class object first so Windows does not route new calls to shutting-down server
                unsafe {
                    let _ = CoRevokeClassObject(registration_cookie);
                }
                registration_cookie = 0;

                // Double check if any widget was added concurrently before revoke
                let still_empty = {
                    let s = state.lock_safe();
                    s.widgets.is_empty()
                };

                if !still_empty {
                    log_widget("Widget registered during shutdown; re-registering class factory");
                    let mut new_cookie = 0;
                    let hr = unsafe {
                        CoRegisterClassObject(
                            &CLSID_NET_FLOW_WIDGET_PROVIDER,
                            Interface::as_raw(&factory_unk),
                            CLSCTX_LOCAL_SERVER,
                            REGCLS_MULTIPLEUSE,
                            &mut new_cookie,
                        )
                    };
                    if hr.0 >= 0 {
                        registration_cookie = new_cookie;
                        continue;
                    }
                }

                break;
            }
        }
    }

    // 6. Stop background worker and release COM registration
    let worker = {
        let mut s = state.lock_safe();
        s.worker.take()
    };
    if let Some(w) = worker {
        w.stop();
    }

    unsafe {
        if registration_cookie != 0 {
            let _ = CoRevokeClassObject(registration_cookie);
        }
        CoUninitialize();
    }

    Ok(())
}

fn ctrlc_handler<F: FnOnce() + Send + 'static>(handler: F) -> bool {
    unsafe extern "system" fn console_ctrl_handler(ctrl_type: u32) -> i32 {
        if ctrl_type == 0 || ctrl_type == 2 {
            if let Some(h) = GLOBAL_HANDLER.lock_safe().take() {
                h();
            }
            1
        } else {
            0
        }
    }

    static GLOBAL_HANDLER: Mutex<Option<Box<dyn FnOnce() + Send>>> = Mutex::new(None);
    *GLOBAL_HANDLER.lock_safe() = Some(Box::new(handler));

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn SetConsoleCtrlHandler(
            handler_routine: Option<unsafe extern "system" fn(u32) -> i32>,
            add: i32,
        ) -> i32;
    }

    unsafe { SetConsoleCtrlHandler(Some(console_ctrl_handler), 1) != 0 }
}
