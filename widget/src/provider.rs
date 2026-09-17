use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use windows_core::{HSTRING, Ref, implement};

use crate::bindings::Microsoft::Windows::Widgets::Providers::{
    IWidgetProvider, IWidgetProvider_Impl, IWidgetProvider2, IWidgetProvider2_Impl,
    WidgetActionInvokedArgs, WidgetContext, WidgetContextChangedArgs,
    WidgetCustomizationRequestedArgs, WidgetManager, WidgetUpdateRequestOptions,
};
use crate::bindings::Microsoft::Windows::Widgets::WidgetSize;
use net_flow_core::backend::{AggregateMode, NetworkBackend, NetworkSnapshot};
use net_flow_core::card::{WidgetConfig, build_adaptive_card, build_settings_card_for_size};
use net_flow_core::format::SpeedUnit;
use net_flow_core::{SAMPLING_INTERVAL_MS, UPDATE_INTERVAL_MS};

pub trait LockExt<T> {
    fn lock_safe(&self) -> std::sync::MutexGuard<'_, T>;
}

impl<T> LockExt<T> for Mutex<T> {
    fn lock_safe(&self) -> std::sync::MutexGuard<'_, T> {
        self.lock().unwrap_or_else(|e| e.into_inner())
    }
}

pub trait RwLockExt<T> {
    fn read_safe(&self) -> std::sync::RwLockReadGuard<'_, T>;
    fn write_safe(&self) -> std::sync::RwLockWriteGuard<'_, T>;
}

impl<T> RwLockExt<T> for RwLock<T> {
    fn read_safe(&self) -> std::sync::RwLockReadGuard<'_, T> {
        self.read().unwrap_or_else(|e| e.into_inner())
    }
    fn write_safe(&self) -> std::sync::RwLockWriteGuard<'_, T> {
        self.write().unwrap_or_else(|e| e.into_inner())
    }
}

pub fn widget_size_to_str(size: WidgetSize) -> &'static str {
    match size {
        WidgetSize::Small => "Small",
        WidgetSize::Large => "Large",
        _ => "Medium",
    }
}

pub struct InternalWidgetInfo {
    pub id: String,
    pub size: WidgetSize,
    pub is_active: bool,
    pub in_customization: bool,
    pub custom_state: WidgetConfig,
    pub draft_state: Option<WidgetConfig>,
    pub customization_requested_at: Option<Instant>,
}

pub struct ProviderState {
    pub widgets: HashMap<String, InternalWidgetInfo>,
    pub active_count: usize,
    pub has_had_widgets: bool,
    pub last_empty_at: Option<Instant>,
    pub worker: Option<WorkerHandle>,
    pub backend: Arc<Mutex<NetworkBackend>>,
    pub latest_snapshot: Arc<RwLock<NetworkSnapshot>>,
    /// Set when session/chart state must be pushed before the next 1s UI tick.
    pub ui_dirty: Arc<AtomicBool>,
}

impl ProviderState {
    pub fn new() -> Self {
        let backend = Arc::new(Mutex::new(NetworkBackend::load_or_create(
            AggregateMode::PhysicalTransport,
        )));
        let initial_snapshot = {
            let mut b = backend.lock_safe();
            b.sample().unwrap_or_default()
        };
        Self {
            widgets: HashMap::new(),
            active_count: 0,
            has_had_widgets: false,
            last_empty_at: Some(Instant::now()),
            worker: None,
            backend,
            latest_snapshot: Arc::new(RwLock::new(initial_snapshot)),
            ui_dirty: Arc::new(AtomicBool::new(false)),
        }
    }
}

pub struct WorkerHandle {
    pub handle: JoinHandle<()>,
    pub shutdown: Arc<(Mutex<bool>, Condvar)>,
}

impl WorkerHandle {
    pub fn stop(self) {
        let (lock, cvar) = &*self.shutdown;
        *lock.lock_safe() = true;
        cvar.notify_all();
        let _ = self.handle.join();
    }
}

pub struct WidgetTarget {
    pub id: String,
    pub size: WidgetSize,
    pub config: WidgetConfig,
    pub in_customization: bool,
}

#[implement(IWidgetProvider, IWidgetProvider2)]
pub struct NetFlowWidgetProvider {
    pub state: Arc<Mutex<ProviderState>>,
}

impl NetFlowWidgetProvider {
    pub fn new(state: Arc<Mutex<ProviderState>>) -> Self {
        Self { state }
    }

    pub fn ensure_worker(&self) {
        let mut state = self.state.lock_safe();
        if state.worker.is_none() {
            let shutdown = Arc::new((Mutex::new(false), Condvar::new()));
            let shutdown_clone = Arc::clone(&shutdown);
            let state_clone = Arc::clone(&self.state);
            let backend_clone = Arc::clone(&state.backend);
            let snapshot_ref = Arc::clone(&state.latest_snapshot);
            let ui_dirty = Arc::clone(&state.ui_dirty);

            let handle = std::thread::spawn(move || {
                worker_loop(
                    shutdown_clone,
                    state_clone,
                    backend_clone,
                    snapshot_ref,
                    ui_dirty,
                );
            });
            state.worker = Some(WorkerHandle { handle, shutdown });
        }
    }
}

pub fn matches_widget_id(stored_id: &str, target_id: &str) -> bool {
    if stored_id.eq_ignore_ascii_case(target_id) {
        return true;
    }
    let a = stored_id.trim_matches(|c| c == '{' || c == '}');
    let b = target_id.trim_matches(|c| c == '{' || c == '}');
    a.eq_ignore_ascii_case(b)
}

static LOG_MUTEX: Mutex<()> = Mutex::new(());

pub fn log_widget(msg: &str) {
    use std::io::Write;
    let _guard = LOG_MUTEX.lock_safe();
    if let Ok(temp) = std::env::var("TEMP") {
        let path = std::path::Path::new(&temp).join("netflow_widget.log");
        let old_path = std::path::Path::new(&temp).join("netflow_widget.log.old");

        if let Ok(meta) = std::fs::metadata(&path)
            && meta.len() >= 1024 * 1024 {
                if old_path.exists() {
                    let _ = std::fs::remove_file(&old_path);
                }
                let _ = std::fs::rename(&path, &old_path);
            }

        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
        {
            let _ = writeln!(f, "[{:?}] {}", std::time::SystemTime::now(), msg);
        }
    }
}

pub fn log_widget_verbose(msg: &str) {
    static VERBOSE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    let is_verbose = *VERBOSE.get_or_init(|| {
        std::env::var("NETFLOW_VERBOSE_LOG")
            .map(|v| v == "1")
            .unwrap_or(false)
    });
    if is_verbose {
        log_widget(msg);
    }
}

/// Centralized function to push the correct card (live or settings) for a widget.
fn update_widget(
    manager: &WidgetManager,
    widget_id: &str,
    size: WidgetSize,
    config: &WidgetConfig,
    in_customization: bool,
    snapshot: &NetworkSnapshot,
) {
    let (template, custom_state_json) = if in_customization {
        let settings_card = build_settings_card_for_size(
            config,
            widget_size_to_str(size),
            snapshot.session_duration_secs,
        );
        let state_json = serde_json::to_string(config).unwrap_or_else(|_| "{}".to_string());
        (settings_card, state_json)
    } else {
        let live_card = build_adaptive_card(snapshot, widget_size_to_str(size), config);
        let state_json = serde_json::to_string(config).unwrap_or_else(|_| "{}".to_string());
        (live_card, state_json)
    };

    match WidgetUpdateRequestOptions::CreateInstance(&HSTRING::from(widget_id)) {
        Ok(opts) => {
            let _ = opts.SetTemplate(&HSTRING::from(&template));
            let _ = opts.SetData(&HSTRING::from("{}"));
            let _ = opts.SetCustomState(&HSTRING::from(&custom_state_json));
            let res = manager.UpdateWidget(&opts);
            match res {
                Ok(()) => {
                    log_widget_verbose(&format!(
                        "UpdateWidget id={} in_custom={}: Ok",
                        widget_id, in_customization
                    ));
                }
                Err(ref e) => {
                    log_widget(&format!(
                        "UpdateWidget id={} in_custom={} failed: {:?}",
                        widget_id, in_customization, e
                    ));
                }
            }
        }
        Err(e) => {
            log_widget(&format!("CreateInstance failed id={}: {:?}", widget_id, e));
        }
    }
}

impl IWidgetProvider_Impl for NetFlowWidgetProvider_Impl {
    fn CreateWidget(&self, widget_context: Ref<WidgetContext>) -> windows_core::Result<()> {
        let ctx = widget_context.ok()?;
        let id = ctx.Id()?.to_string_lossy();
        let size = ctx.Size()?;
        log_widget(&format!("CreateWidget id={id} size={size:?}"));

        let config = {
            let state = self.state.lock_safe();
            state
                .widgets
                .get(&id)
                .map(|w| w.custom_state.clone())
                .unwrap_or_default()
        };

        {
            let mut state = self.state.lock_safe();
            state.has_had_widgets = true;
            state.last_empty_at = None;
            state.widgets.insert(
                id.clone(),
                InternalWidgetInfo {
                    id: id.clone(),
                    size,
                    is_active: false,
                    in_customization: false,
                    custom_state: config.clone(),
                    draft_state: None,
                    customization_requested_at: None,
                },
            );
        }

        // Push initial card from latest snapshot
        let snapshot = {
            let state = self.state.lock_safe();
            state.latest_snapshot.read_safe().clone()
        };

        if let Ok(manager) = WidgetManager::GetDefault() {
            update_widget(&manager, &id, size, &config, false, &snapshot);
        }

        self.ensure_worker();
        Ok(())
    }

    fn DeleteWidget(
        &self,
        widget_id: &HSTRING,
        _custom_state: &HSTRING,
    ) -> windows_core::Result<()> {
        let id = widget_id.to_string_lossy();
        log_widget(&format!("DeleteWidget id={id}"));
        {
            let mut state = self.state.lock_safe();
            let mut removed_active = false;
            state.widgets.retain(|wid, w| {
                if matches_widget_id(wid, &id) {
                    if w.is_active {
                        removed_active = true;
                    }
                    false
                } else {
                    true
                }
            });
            if removed_active && state.active_count > 0 {
                state.active_count -= 1;
            }
            if state.widgets.is_empty() {
                state.last_empty_at = Some(Instant::now());
            }
        }
        Ok(())
    }

    fn OnActionInvoked(
        &self,
        action_invoked_args: Ref<WidgetActionInvokedArgs>,
    ) -> windows_core::Result<()> {
        let args = action_invoked_args.ok()?;
        let verb = args.Verb()?.to_string_lossy();
        let widget_ctx = args.WidgetContext()?;
        let widget_id = widget_ctx.Id()?.to_string_lossy();
        let data_json = args.Data().map(|d| d.to_string_lossy()).unwrap_or_default();
        log_widget(&format!(
            "OnActionInvoked verb={} id={} data={}",
            verb, widget_id, data_json
        ));

        match verb.as_str() {
            "save_settings" => {
                let mut target_id = widget_id.clone();
                let new_config = {
                    let state = self.state.lock_safe();
                    let current = state
                        .widgets
                        .iter()
                        .find(|(id, _)| matches_widget_id(id, &widget_id))
                        .map(|(id, w)| {
                            target_id = id.clone();
                            w.custom_state.clone()
                        })
                        .or_else(|| {
                            if state.widgets.len() == 1 {
                                state.widgets.iter().next().map(|(id, w)| {
                                    target_id = id.clone();
                                    w.custom_state.clone()
                                })
                            } else {
                                None
                            }
                        })
                        .unwrap_or_default();
                    parse_settings_form(&data_json, &current)
                };
                {
                    let mut state = self.state.lock_safe();
                    let mut found = false;
                    for (id, w) in state.widgets.iter_mut() {
                        if matches_widget_id(id, &target_id) {
                            w.custom_state = new_config.clone();
                            w.in_customization = false;
                            w.draft_state = None;
                            w.customization_requested_at = None;
                            target_id = id.clone();
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        if state.widgets.len() == 1 {
                            if let Some((id, w)) = state.widgets.iter_mut().next() {
                                log_widget(&format!(
                                    "save_settings fallback: id={widget_id} not found, applying to unique widget {id}"
                                ));
                                w.custom_state = new_config.clone();
                                w.in_customization = false;
                                w.draft_state = None;
                                w.customization_requested_at = None;
                                target_id = id.clone();
                            }
                        } else {
                            log_widget(&format!(
                                "save_settings warning: id={widget_id} not found among {} registered widgets; ignoring",
                                state.widgets.len()
                            ));
                            return Ok(());
                        }
                    }
                    state.ui_dirty.store(true, Ordering::SeqCst);
                    if let Some(worker) = &state.worker {
                        worker.shutdown.1.notify_all();
                    }
                }
                self.push_current_card(&target_id);
                if target_id != widget_id {
                    self.push_current_card(&widget_id);
                }
            }
            "toggle_apps" => {
                let mut target_id = widget_id.clone();
                let mut found = false;
                {
                    let mut state = self.state.lock_safe();
                    for (id, w) in state.widgets.iter_mut() {
                        if matches_widget_id(id, &widget_id) {
                            w.custom_state.apps_expanded = !w.custom_state.apps_expanded;
                            w.custom_state.apps_page = 0;
                            target_id = id.clone();
                            found = true;
                            break;
                        }
                    }
                    if !found && state.widgets.len() == 1
                        && let Some((id, w)) = state.widgets.iter_mut().next() {
                            w.custom_state.apps_expanded = !w.custom_state.apps_expanded;
                            w.custom_state.apps_page = 0;
                            target_id = id.clone();
                            found = true;
                        }
                }
                if found {
                    self.push_current_card(&target_id);
                }
            }
            "next_apps_page" => {
                let mut target_id = widget_id.clone();
                let mut found = false;
                {
                    let mut state = self.state.lock_safe();
                    for (id, w) in state.widgets.iter_mut() {
                        if matches_widget_id(id, &widget_id) {
                            w.custom_state.apps_page = w.custom_state.apps_page.saturating_add(1);
                            target_id = id.clone();
                            found = true;
                            break;
                        }
                    }
                    if !found && state.widgets.len() == 1
                        && let Some((id, w)) = state.widgets.iter_mut().next() {
                            w.custom_state.apps_page = w.custom_state.apps_page.saturating_add(1);
                            target_id = id.clone();
                            found = true;
                        }
                }
                if found {
                    self.push_current_card(&target_id);
                }
            }
            "prev_apps_page" => {
                let mut target_id = widget_id.clone();
                let mut found = false;
                {
                    let mut state = self.state.lock_safe();
                    for (id, w) in state.widgets.iter_mut() {
                        if matches_widget_id(id, &widget_id) {
                            w.custom_state.apps_page = w.custom_state.apps_page.saturating_sub(1);
                            target_id = id.clone();
                            found = true;
                            break;
                        }
                    }
                    if !found && state.widgets.len() == 1
                        && let Some((id, w)) = state.widgets.iter_mut().next() {
                            w.custom_state.apps_page = w.custom_state.apps_page.saturating_sub(1);
                            target_id = id.clone();
                            found = true;
                        }
                }
                if found {
                    self.push_current_card(&target_id);
                }
            }
            "reset_session" => {
                let mut target_id = widget_id.clone();
                let mut found = false;
                {
                    let state = self.state.lock_safe();
                    if let Some((id, _)) = state
                        .widgets
                        .iter()
                        .find(|(id, _)| matches_widget_id(id, &widget_id))
                    {
                        target_id = id.clone();
                        found = true;
                    } else if state.widgets.len() == 1
                        && let Some((id, _)) = state.widgets.iter().next() {
                            target_id = id.clone();
                            found = true;
                        }
                    if found {
                        let mut backend = state.backend.lock_safe();
                        backend.reset_session();
                        let s = backend.sample().unwrap_or_default();
                        let mut snap_write = state.latest_snapshot.write_safe();
                        *snap_write = s;
                        state.ui_dirty.store(true, Ordering::SeqCst);
                        if let Some(worker) = &state.worker {
                            worker.shutdown.1.notify_all();
                        }
                    }
                }
                if found {
                    self.push_current_card(&target_id);
                }
            }
            "open_settings" => {
                log_widget(&format!("Handling open_settings for id={widget_id}"));
                let mut target_id = widget_id.clone();
                let mut found = false;
                {
                    let mut state = self.state.lock_safe();
                    for (id, w) in state.widgets.iter_mut() {
                        if matches_widget_id(id, &widget_id) {
                            w.in_customization = true;
                            w.draft_state = Some(w.custom_state.clone());
                            w.customization_requested_at = None;
                            target_id = id.clone();
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        if state.widgets.len() == 1 {
                            if let Some((id, w)) = state.widgets.iter_mut().next() {
                                log_widget(&format!(
                                    "open_settings fallback: id={widget_id} not found, applying to unique widget {id}"
                                ));
                                w.in_customization = true;
                                w.draft_state = Some(w.custom_state.clone());
                                w.customization_requested_at = None;
                                target_id = id.clone();
                            }
                        } else {
                            log_widget(&format!(
                                "open_settings warning: id={widget_id} not found among {} registered widgets; ignoring",
                                state.widgets.len()
                            ));
                            return Ok(());
                        }
                    }
                    state.ui_dirty.store(true, Ordering::SeqCst);
                    if let Some(worker) = &state.worker {
                        worker.shutdown.1.notify_all();
                    }
                }
                self.push_current_card(&target_id);
                if target_id != widget_id {
                    self.push_current_card(&widget_id);
                }
            }
            "cancel_settings" | "cancel" | "exit" | "exitCustomization" | "dismiss" => {
                log_widget(&format!("Handling cancel_settings for id={widget_id}"));
                let mut target_id = widget_id.clone();
                let mut found = false;
                {
                    let mut state = self.state.lock_safe();
                    for (id, w) in state.widgets.iter_mut() {
                        if matches_widget_id(id, &widget_id) {
                            w.in_customization = false;
                            w.draft_state = None;
                            w.customization_requested_at = None;
                            target_id = id.clone();
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        if state.widgets.len() == 1 {
                            if let Some((id, w)) = state.widgets.iter_mut().next() {
                                log_widget(&format!(
                                    "cancel_settings fallback: id={widget_id} not found, applying to unique widget {id}"
                                ));
                                w.in_customization = false;
                                w.draft_state = None;
                                w.customization_requested_at = None;
                                target_id = id.clone();
                            }
                        } else {
                            log_widget(&format!(
                                "cancel_settings warning: id={widget_id} not found among {} registered widgets; ignoring",
                                state.widgets.len()
                            ));
                            return Ok(());
                        }
                    }
                    state.ui_dirty.store(true, Ordering::SeqCst);
                    if let Some(worker) = &state.worker {
                        worker.shutdown.1.notify_all();
                    }
                }
                self.push_current_card(&target_id);
                if target_id != widget_id {
                    self.push_current_card(&widget_id);
                }
            }
            _ => {
                log_widget(&format!("Unknown verb invoked: {verb}"));
            }
        }

        Ok(())
    }

    fn OnWidgetContextChanged(
        &self,
        context_changed_args: Ref<WidgetContextChangedArgs>,
    ) -> windows_core::Result<()> {
        let args = context_changed_args.ok()?;
        let ctx = args.WidgetContext()?;
        let id = ctx.Id()?.to_string_lossy();
        let new_size = ctx.Size()?;
        log_widget(&format!(
            "OnWidgetContextChanged id={id} new_size={new_size:?}"
        ));

        let mut target_id = id.clone();
        {
            let mut state = self.state.lock_safe();
            for (wid, w) in state.widgets.iter_mut() {
                if matches_widget_id(wid, &id) {
                    w.size = new_size;
                    target_id = wid.clone();
                    break;
                }
            }
        }

        self.push_current_card(&target_id);
        Ok(())
    }

    fn Activate(&self, widget_context: Ref<WidgetContext>) -> windows_core::Result<()> {
        let ctx = widget_context.ok()?;
        let id = ctx.Id()?.to_string_lossy();
        let size = ctx.Size()?;
        log_widget(&format!("Activate id={id} size={size:?}"));

        let mut target_id = id.clone();
        {
            let mut state = self.state.lock_safe();
            state.has_had_widgets = true;
            state.last_empty_at = None;
            let mut became_active = false;
            let mut found = false;
            for (wid, w) in state.widgets.iter_mut() {
                if matches_widget_id(wid, &id) {
                    if !w.is_active {
                        w.is_active = true;
                        became_active = true;
                    }
                    w.size = size;
                    // Transition to active flyout (or normal activation) is complete.
                    // Clear the pending transition timestamp so future deactivations reset cleanly.
                    w.customization_requested_at = None;
                    target_id = wid.clone();
                    found = true;
                    break;
                }
            }
            if !found {
                state.widgets.insert(
                    id.clone(),
                    InternalWidgetInfo {
                        id: id.clone(),
                        size,
                        is_active: true,
                        in_customization: false,
                        custom_state: WidgetConfig::default(),
                        draft_state: None,
                        customization_requested_at: None,
                    },
                );
                became_active = true;
            }
            if became_active {
                state.active_count += 1;
            }
        }

        self.ensure_worker();
        // Push current continuous snapshot immediately on activation
        self.push_current_card(&target_id);
        if target_id != id {
            self.push_current_card(&id);
        }
        Ok(())
    }

    fn Deactivate(&self, widget_id: &HSTRING) -> windows_core::Result<()> {
        let id = widget_id.to_string_lossy();
        log_widget(&format!("Deactivate id={id}"));
        {
            let mut state = self.state.lock_safe();
            let mut became_inactive = false;
            for (wid, w) in state.widgets.iter_mut() {
                if matches_widget_id(wid, &id) {
                    if w.is_active {
                        w.is_active = false;
                        became_inactive = true;
                    }
                    // If OnCustomizationRequested was fired within the last 1.5 seconds,
                    // this Deactivate is Windows 11 transitioning from the board widget
                    // to the modal customization flyout. Do NOT reset in_customization!
                    let is_opening_flyout = w
                        .customization_requested_at
                        .map(|t| t.elapsed() < Duration::from_millis(1500))
                        .unwrap_or(false);

                    if is_opening_flyout {
                        log_widget(&format!(
                            "Deactivate id={id} during flyout opening transition; keeping in_customization=true"
                        ));
                    } else {
                        // When the widget is deactivated (e.g. board closed or customization flyout dismissed),
                        // reset customization mode so reopening never stays stuck in settings!
                        w.in_customization = false;
                        w.draft_state = None;
                        w.customization_requested_at = None;
                    }
                }
            }
            if became_inactive && state.active_count > 0 {
                state.active_count -= 1;
            }
        }
        // Keep backend alive and background telemetry continuous
        Ok(())
    }
}

impl IWidgetProvider2_Impl for NetFlowWidgetProvider_Impl {
    fn OnCustomizationRequested(
        &self,
        customization_requested_args: Ref<WidgetCustomizationRequestedArgs>,
    ) -> windows_core::Result<()> {
        let args = customization_requested_args.ok()?;
        let ctx = args.WidgetContext()?;
        let id = ctx.Id()?.to_string_lossy();

        let custom_state_str = args
            .CustomState()
            .map(|cs| cs.to_string_lossy())
            .unwrap_or_default();
        log_widget(&format!(
            "OnCustomizationRequested id={id} custom_state={custom_state_str}"
        ));

        let mut target_id = id.clone();
        let parsed_config: WidgetConfig = {
            let state = self.state.lock_safe();
            state
                .widgets
                .iter()
                .find(|(wid, _)| matches_widget_id(wid, &id))
                .map(|(wid, w)| {
                    target_id = wid.clone();
                    w.custom_state.clone()
                })
                .unwrap_or_else(|| {
                    if custom_state_str.is_empty() {
                        WidgetConfig::default()
                    } else {
                        serde_json::from_str(&custom_state_str).unwrap_or_default()
                    }
                })
        };

        let (board_size, board_config) = {
            let mut state = self.state.lock_safe();
            state.has_had_widgets = true;
            state.last_empty_at = None;
            let mut found = false;
            let mut current_size = WidgetSize::Medium;
            for (wid, w) in state.widgets.iter_mut() {
                if matches_widget_id(wid, &id) {
                    w.in_customization = true;
                    w.customization_requested_at = Some(Instant::now());
                    w.draft_state = Some(parsed_config.clone());
                    w.custom_state = parsed_config.clone();
                    current_size = w.size;
                    target_id = wid.clone();
                    found = true;
                    break;
                }
            }
            if !found {
                current_size = ctx.Size().unwrap_or(WidgetSize::Medium);
                state.widgets.insert(
                    id.clone(),
                    InternalWidgetInfo {
                        id: id.clone(),
                        size: current_size,
                        is_active: true,
                        in_customization: true,
                        custom_state: parsed_config.clone(),
                        draft_state: Some(parsed_config.clone()),
                        customization_requested_at: Some(Instant::now()),
                    },
                );
            }
            state.ui_dirty.store(true, Ordering::SeqCst);
            if let Some(worker) = &state.worker {
                worker.shutdown.1.notify_all();
            }
            (current_size, parsed_config)
        };

        // Render the live dashboard to the background board widget before Windows deactivates it
        // and displays the modal customization flyout over it!
        // This guarantees that behind the flyout, the user sees their live dashboard,
        // while the upcoming Activate call sends the settings card directly into the flyout.
        let snapshot = {
            let state = self.state.lock_safe();
            state.latest_snapshot.read_safe().clone()
        };
        if let Ok(manager) = WidgetManager::GetDefault() {
            update_widget(
                &manager,
                &target_id,
                board_size,
                &board_config,
                false,
                &snapshot,
            );
            if target_id != id {
                update_widget(&manager, &id, board_size, &board_config, false, &snapshot);
            }
        }
        Ok(())
    }
}

impl NetFlowWidgetProvider_Impl {
    /// Push the correct card for a widget based on its current state.
    fn push_current_card(&self, widget_id: &str) {
        let (actual_id, size, config, in_customization) = {
            let state = self.state.lock_safe();
            let found = state
                .widgets
                .iter()
                .find(|(id, _)| matches_widget_id(id, widget_id))
                .map(|(id, w)| {
                    (
                        id.clone(),
                        w.size,
                        w.custom_state.clone(),
                        w.in_customization,
                    )
                });

            match found {
                Some(data) => data,
                None => {
                    if state.widgets.len() == 1 {
                        if let Some((id, w)) = state.widgets.iter().next() {
                            log_widget(&format!(
                                "push_current_card mismatch: widget_id={} not found, falling back to unique widget {}",
                                widget_id, id
                            ));
                            (
                                id.clone(),
                                w.size,
                                w.custom_state.clone(),
                                w.in_customization,
                            )
                        } else {
                            return;
                        }
                    } else {
                        log_widget(&format!(
                            "push_current_card warning: widget_id={} not found among {} registered widgets; ignoring",
                            widget_id,
                            state.widgets.len()
                        ));
                        return;
                    }
                }
            }
        };

        let snapshot = {
            let state = self.state.lock_safe();
            state.latest_snapshot.read_safe().clone()
        };

        log_widget_verbose(&format!(
            "push_current_card req_id={} -> actual_id={} size={:?} in_custom={}",
            widget_id, actual_id, size, in_customization
        ));

        if let Ok(manager) = WidgetManager::GetDefault() {
            update_widget(
                &manager,
                &actual_id,
                size,
                &config,
                in_customization,
                &snapshot,
            );
            if actual_id != widget_id && !widget_id.is_empty() {
                update_widget(
                    &manager,
                    widget_id,
                    size,
                    &config,
                    in_customization,
                    &snapshot,
                );
            }
        }
    }
}

/// Parse the settings form data JSON from OnActionInvoked.
fn parse_settings_form(data_json: &str, current_config: &WidgetConfig) -> WidgetConfig {
    if data_json.is_empty() {
        return current_config.clone();
    }

    let parsed: serde_json::Value = match serde_json::from_str(data_json) {
        Ok(v) => v,
        Err(_) => return current_config.clone(),
    };

    let speed_unit = parsed
        .get("speed_unit")
        .and_then(|v| v.as_str())
        .map(SpeedUnit::from_str_value)
        .unwrap_or(current_config.speed_unit);

    let chart_window = parsed
        .get("chart_window")
        .and_then(|v| {
            if let Some(n) = v.as_u64() {
                Some(n as u32)
            } else if let Some(s) = v.as_str() {
                s.parse::<u32>().ok()
            } else {
                None
            }
        })
        .map(net_flow_core::clamp_chart_window)
        .unwrap_or(current_config.chart_window);

    let apps_expanded = parsed
        .get("apps_expanded")
        .map(|v| {
            if let Some(b) = v.as_bool() {
                b
            } else if let Some(s) = v.as_str() {
                s.eq_ignore_ascii_case("true") || s == "1"
            } else {
                false
            }
        })
        .unwrap_or(current_config.apps_expanded);

    WidgetConfig {
        speed_unit,
        chart_window,
        apps_expanded,
        apps_page: 0,
    }
}

fn worker_loop(
    shutdown: Arc<(Mutex<bool>, Condvar)>,
    state: Arc<Mutex<ProviderState>>,
    backend: Arc<Mutex<NetworkBackend>>,
    snapshot_ref: Arc<RwLock<NetworkSnapshot>>,
    ui_dirty: Arc<AtomicBool>,
) {
    let sample_period = Duration::from_millis(SAMPLING_INTERVAL_MS);
    let ui_period = Duration::from_millis(UPDATE_INTERVAL_MS);
    let persist_period = Duration::from_secs(5);
    let mut next_sample = Instant::now() + sample_period;
    let mut last_ui_update = Instant::now();
    let mut last_persist = Instant::now();

    loop {
        let wait = next_sample.saturating_duration_since(Instant::now());
        let (lock, cvar) = &*shutdown;
        let guard = lock.lock_safe();
        let (guard, _) = cvar
            .wait_timeout(guard, wait)
            .unwrap_or_else(|e| e.into_inner());
        if *guard {
            break;
        }
        drop(guard);

        let started = Instant::now();
        {
            let mut b = backend.lock_safe();
            if let Ok(s) = b.sample() {
                let mut snap_write = snapshot_ref.write_safe();
                *snap_write = s;
            }
            if last_persist.elapsed() >= persist_period {
                b.persist_session();
                last_persist = Instant::now();
            }
        }

        let planned = started + sample_period;
        next_sample = if planned > Instant::now() {
            planned
        } else {
            Instant::now() + sample_period
        };

        let force_ui = ui_dirty.swap(false, Ordering::SeqCst);
        if !force_ui && last_ui_update.elapsed() < ui_period {
            continue;
        }
        last_ui_update = Instant::now();

        let snapshot = snapshot_ref.read_safe().clone();

        let targets: Vec<WidgetTarget> = {
            let s = state.lock_safe();
            s.widgets
                .values()
                .filter(|w| w.is_active)
                .map(|w| WidgetTarget {
                    id: w.id.clone(),
                    size: w.size,
                    config: w.custom_state.clone(),
                    in_customization: w.in_customization,
                })
                .collect()
        };

        if targets.is_empty() {
            continue;
        }

        if let Ok(manager) = WidgetManager::GetDefault() {
            for target in &targets {
                if target.in_customization {
                    continue;
                }
                update_widget(
                    &manager,
                    &target.id,
                    target.size,
                    &target.config,
                    false,
                    &snapshot,
                );
            }
        }
    }

    let b = backend.lock_safe();
    b.persist_session();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_safe_recovers_from_poison() {
        let lock = Arc::new(Mutex::new(42));
        let lock_clone = Arc::clone(&lock);
        let _ = std::panic::catch_unwind(move || {
            let _guard = lock_clone.lock_safe();
            panic!("intentional panic to poison lock");
        });
        assert!(lock.is_poisoned());
        let guard = lock.lock_safe();
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_matches_widget_id() {
        assert!(matches_widget_id("123", "123"));
        assert!(matches_widget_id("ABC", "abc"));
        assert!(matches_widget_id("{123-456}", "123-456"));
        assert!(matches_widget_id("123-456", "{123-456}"));
        assert!(matches_widget_id("{ABC-DEF}", "abc-def"));
        assert!(!matches_widget_id("123", "456"));
    }

    #[test]
    fn test_parse_settings_form() {
        let current = WidgetConfig::default();
        // Empty data returns current
        let unchanged = parse_settings_form("", &current);
        assert_eq!(unchanged.speed_unit, current.speed_unit);
        assert_eq!(unchanged.chart_window, current.chart_window);

        // Valid update
        let json_data = r#"{"speed_unit":"mb","chart_window":"60","apps_expanded":"true"}"#;
        let updated = parse_settings_form(json_data, &current);
        assert_eq!(updated.speed_unit, SpeedUnit::Megabytes);
        assert_eq!(updated.chart_window, 60);
        assert!(updated.apps_expanded);

        // Invalid JSON retains current config
        let bad_json = "not a json string";
        let fallback = parse_settings_form(bad_json, &current);
        assert_eq!(fallback.speed_unit, current.speed_unit);
    }

    #[test]
    fn test_flyout_transition_lifecycle() {
        let mut widget = InternalWidgetInfo {
            id: "test-widget-1".to_string(),
            size: WidgetSize::Medium,
            is_active: true,
            in_customization: false,
            custom_state: WidgetConfig::default(),
            draft_state: None,
            customization_requested_at: None,
        };

        // Step 1: OnCustomizationRequested triggers
        widget.in_customization = true;
        widget.customization_requested_at = Some(Instant::now());
        widget.draft_state = Some(WidgetConfig::default());

        // Step 2: Windows immediately calls Deactivate (opening transition)
        let is_opening = widget
            .customization_requested_at
            .map(|t| t.elapsed() < Duration::from_millis(1500))
            .unwrap_or(false);
        assert!(is_opening, "Should detect flyout opening transition");

        if !is_opening {
            widget.in_customization = false;
        }
        assert!(
            widget.in_customization,
            "in_customization must be preserved during transition"
        );

        // Step 3: Windows activates the flyout host
        // Activate marks transition complete by clearing customization_requested_at
        widget.customization_requested_at = None;
        assert!(
            widget.in_customization,
            "Flyout must receive in_customization = true"
        );

        // Step 4: User dismisses the flyout (clicks X or outside) -> Deactivate fires
        let is_opening_after_dismiss = widget
            .customization_requested_at
            .map(|t| t.elapsed() < Duration::from_millis(1500))
            .unwrap_or(false);
        assert!(
            !is_opening_after_dismiss,
            "Dismissal is not an opening transition"
        );

        if !is_opening_after_dismiss {
            widget.in_customization = false;
            widget.draft_state = None;
            widget.customization_requested_at = None;
        }
        assert!(
            !widget.in_customization,
            "in_customization must be reset to false on dismissal"
        );
        assert!(widget.draft_state.is_none());
    }

    #[test]
    fn test_expired_transition_resets_safely() {
        let mut widget = InternalWidgetInfo {
            id: "test-widget-2".to_string(),
            size: WidgetSize::Medium,
            is_active: true,
            in_customization: true,
            custom_state: WidgetConfig::default(),
            draft_state: Some(WidgetConfig::default()),
            // Simulated stale transition older than 1.5s
            customization_requested_at: Some(Instant::now() - Duration::from_secs(5)),
        };

        let is_opening = widget
            .customization_requested_at
            .map(|t| t.elapsed() < Duration::from_millis(1500))
            .unwrap_or(false);
        assert!(
            !is_opening,
            "Stale transition must not be treated as opening"
        );

        if !is_opening {
            widget.in_customization = false;
            widget.draft_state = None;
            widget.customization_requested_at = None;
        }
        assert!(!widget.in_customization);
        assert!(widget.draft_state.is_none());
        assert!(widget.customization_requested_at.is_none());
    }
}
