use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
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
use net_flow_core::card::{
    WidgetConfig, build_adaptive_card_data_string, build_adaptive_card_template,
    build_settings_card_for_size,
};
use net_flow_core::format::SpeedUnit;
use net_flow_core::{
    BandwidthAlertConfig, BandwidthAlertEngine, GraphStyle, SAMPLING_INTERVAL_MS, ThemeMode,
    compute_adaptive_ui_interval, load_alert_config, save_alert_config,
};

/// Maximum data-update suppression interval (application-level empirical safety policy).
/// Periodically republishes unchanged data to avoid indefinitely suppressing provider updates.
/// Note: This is an internal Net Flow policy for empirical safety, NOT a Windows Widgets platform requirement.
pub const REDUNDANT_UPDATE_HEARTBEAT: Duration = Duration::from_secs(15);

/// Computes a fast 64-bit hash of a serialized data JSON payload.
pub fn compute_payload_hash(data: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

/// Determines whether dynamic telemetry data should be published to the widget host.
///
/// Collision-free identity:
/// 1. If `force` is true, always publish.
/// 2. Fast path: If `last_hash` differs from `current_hash`, payload definitely changed -> publish.
/// 3. If `last_hash` matches, verify `last_json == current_json` (exact byte equality) to eliminate hash collisions.
/// 4. If payload is truly identical:
///    - If `last_published_at.elapsed() >= REDUNDANT_UPDATE_HEARTBEAT` (15s empirical safety interval), publish.
///    - Otherwise, skip publication entirely (0 IPC, 0 host layout work).
pub fn should_publish_data(
    force: bool,
    current_hash: u64,
    current_json: &str,
    last_hash: Option<u64>,
    last_json: Option<&str>,
    last_published_at: Option<Instant>,
) -> bool {
    if force {
        return true;
    }

    match (last_hash, last_json) {
        (Some(lh), Some(lj)) if lh == current_hash && lj == current_json => match last_published_at
        {
            Some(published_at) => published_at.elapsed() >= REDUNDANT_UPDATE_HEARTBEAT,
            None => true,
        },
        _ => true,
    }
}

/// Poison-safe lock acquisition helper to keep the widget server resilient if a thread panics.
pub trait LockExt<T> {
    fn lock_safe(&self) -> std::sync::MutexGuard<'_, T>;
}

impl<T> LockExt<T> for Mutex<T> {
    fn lock_safe(&self) -> std::sync::MutexGuard<'_, T> {
        self.lock().unwrap_or_else(|e| e.into_inner())
    }
}

/// Poison-safe RwLock helper.
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

/// Maps Windows WidgetSize enum variants to string keys expected by Adaptive Card builders.
pub fn widget_size_to_str(size: WidgetSize) -> &'static str {
    match size {
        WidgetSize::Small => "Small",
        WidgetSize::Large => "Large",
        _ => "Medium",
    }
}

/// The installed visual template kind on the widget board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TemplateKind {
    #[default]
    Live,
    Settings,
}

/// Internal tracking info for each widget instance registered on the board.
pub struct InternalWidgetInfo {
    pub id: String,
    pub size: WidgetSize,
    pub is_active: bool,
    pub in_customization: bool,
    pub template_kind: TemplateKind,
    pub custom_state: WidgetConfig,
    pub draft_state: Option<WidgetConfig>,
    pub customization_requested_at: Option<Instant>,
    // Phase B: collision-free payload diffing and forced publication
    pub last_data_hash: Option<u64>,
    pub last_data_json: Option<String>,
    pub last_data_published_at: Option<Instant>,
    pub force_data_publish: bool,
}

/// Shared state synchronized across the COM provider and the background sampling worker thread.
pub struct ProviderState {
    pub widgets: HashMap<String, InternalWidgetInfo>,
    pub active_count: usize,
    pub has_had_widgets: bool,
    pub last_empty_at: Option<Instant>,
    pub worker: Option<WorkerHandle>,
    pub backend: Arc<Mutex<NetworkBackend>>,
    pub latest_snapshot: Arc<RwLock<NetworkSnapshot>>,
    /// Flagged when a user interaction (like resetting session totals) requires an immediate card update.
    pub ui_dirty: Arc<AtomicBool>,
    /// Global alert preferences survive widget recreation in `%LOCALAPPDATA%\\NetFlow\\alerts.json`.
    pub alert_config: Arc<Mutex<BandwidthAlertConfig>>,
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
            alert_config: Arc::new(Mutex::new(load_alert_config())),
        }
    }
}

/// Thread join handle and shutdown condition variable for the sampling worker.
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

/// Lightweight snapshot of a widget target passed to the worker thread for card pushing.
pub struct WidgetTarget {
    pub id: String,
    pub size: WidgetSize,
    pub config: WidgetConfig,
    pub template_kind: TemplateKind,
    pub in_customization: bool,
    pub last_data_hash: Option<u64>,
    pub last_data_json: Option<String>,
    pub last_data_published_at: Option<Instant>,
    pub force_data_publish: bool,
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
            let alert_config = Arc::clone(&state.alert_config);

            let handle = std::thread::spawn(move || {
                worker_loop(
                    shutdown_clone,
                    state_clone,
                    backend_clone,
                    snapshot_ref,
                    ui_dirty,
                    alert_config,
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
            && meta.len() >= 1024 * 1024
        {
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

/// Constructs WidgetUpdateRequestOptions containing dynamic telemetry data only (leaving Template unset).
pub fn build_data_update_options(
    widget_id: &str,
    data_json: &str,
) -> windows_core::Result<WidgetUpdateRequestOptions> {
    let opts = WidgetUpdateRequestOptions::CreateInstance(&HSTRING::from(widget_id))?;
    opts.SetData(&HSTRING::from(data_json))?;
    Ok(opts)
}

/// Constructs WidgetUpdateRequestOptions containing visual template, data context, and custom state.
pub fn build_template_and_data_options(
    widget_id: &str,
    template: &str,
    data_json: &str,
    custom_state_json: &str,
) -> windows_core::Result<WidgetUpdateRequestOptions> {
    let opts = WidgetUpdateRequestOptions::CreateInstance(&HSTRING::from(widget_id))?;
    opts.SetTemplate(&HSTRING::from(template))?;
    opts.SetData(&HSTRING::from(data_json))?;
    opts.SetCustomState(&HSTRING::from(custom_state_json))?;
    Ok(opts)
}

/// Structural lifecycle update: pushes visual template, initial data payload, and custom state.
pub fn update_widget_template_and_data(
    manager: &WidgetManager,
    widget_id: &str,
    template: &str,
    data_json: &str,
    custom_state_json: &str,
) {
    match build_template_and_data_options(widget_id, template, data_json, custom_state_json) {
        Ok(opts) => {
            let res = manager.UpdateWidget(&opts);
            match res {
                Ok(()) => {
                    log_widget_verbose(&format!(
                        "UpdateWidget (Template+Data) id={}: Ok",
                        widget_id
                    ));
                }
                Err(ref e) => {
                    log_widget(&format!(
                        "UpdateWidget (Template+Data) id={} failed: {:?}",
                        widget_id, e
                    ));
                }
            }
        }
        Err(e) => {
            log_widget(&format!(
                "build_template_and_data_options failed id={}: {:?}",
                widget_id, e
            ));
        }
    }
}

/// Telemetry update: pushes dynamic data payload ONLY, leaving the existing visual template untouched.
pub fn update_widget_data_only(manager: &WidgetManager, widget_id: &str, data_json: &str) {
    match build_data_update_options(widget_id, data_json) {
        Ok(opts) => {
            let res = manager.UpdateWidget(&opts);
            match res {
                Ok(()) => {
                    log_widget_verbose(&format!("UpdateWidget (Data only) id={}: Ok", widget_id));
                }
                Err(ref e) => {
                    log_widget(&format!(
                        "UpdateWidget (Data only) id={} failed: {:?}",
                        widget_id, e
                    ));
                }
            }
        }
        Err(e) => {
            log_widget(&format!(
                "build_data_update_options failed id={}: {:?}",
                widget_id, e
            ));
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
                    template_kind: TemplateKind::Live,
                    custom_state: config.clone(),
                    draft_state: None,
                    customization_requested_at: None,
                    last_data_hash: None,
                    last_data_json: None,
                    last_data_published_at: None,
                    force_data_publish: true,
                },
            );
        }

        // Push initial card from latest snapshot
        let snapshot = {
            let state = self.state.lock_safe();
            state.latest_snapshot.read_safe().clone()
        };

        if let Ok(manager) = WidgetManager::GetDefault() {
            let size_str = widget_size_to_str(size);
            let template = build_adaptive_card_template(size_str);
            let data = build_adaptive_card_data_string(&snapshot, &config, size_str);
            let state_json = serde_json::to_string(&config).unwrap_or_else(|_| "{}".to_string());
            update_widget_template_and_data(&manager, &id, &template, &data, &state_json);

            let h = compute_payload_hash(&data);
            let mut state = self.state.lock_safe();
            if let Some(w) = state.widgets.get_mut(&id) {
                w.last_data_hash = Some(h);
                w.last_data_json = Some(data);
                w.last_data_published_at = Some(Instant::now());
                w.force_data_publish = false;
            }
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
                            w.template_kind = TemplateKind::Live;
                            w.draft_state = None;
                            w.customization_requested_at = None;
                            w.force_data_publish = true;
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
                                w.template_kind = TemplateKind::Live;
                                w.draft_state = None;
                                w.customization_requested_at = None;
                                w.force_data_publish = true;
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
                    // Mirror the saved alert preferences into the global engine and
                    // persist them so they survive widget recreation and restarts.
                    let alert_prefs = new_config.alerts.clone();
                    *state.alert_config.lock_safe() = alert_prefs.clone();
                    save_alert_config(&alert_prefs);
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
                            w.force_data_publish = true;
                            target_id = id.clone();
                            found = true;
                            break;
                        }
                    }
                    if !found
                        && state.widgets.len() == 1
                        && let Some((id, w)) = state.widgets.iter_mut().next()
                    {
                        w.custom_state.apps_expanded = !w.custom_state.apps_expanded;
                        w.custom_state.apps_page = 0;
                        w.force_data_publish = true;
                        target_id = id.clone();
                        found = true;
                    }
                }
                if found {
                    self.push_current_data_only(&target_id);
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
                            w.force_data_publish = true;
                            target_id = id.clone();
                            found = true;
                            break;
                        }
                    }
                    if !found
                        && state.widgets.len() == 1
                        && let Some((id, w)) = state.widgets.iter_mut().next()
                    {
                        w.custom_state.apps_page = w.custom_state.apps_page.saturating_add(1);
                        w.force_data_publish = true;
                        target_id = id.clone();
                        found = true;
                    }
                }
                if found {
                    self.push_current_data_only(&target_id);
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
                            w.force_data_publish = true;
                            target_id = id.clone();
                            found = true;
                            break;
                        }
                    }
                    if !found
                        && state.widgets.len() == 1
                        && let Some((id, w)) = state.widgets.iter_mut().next()
                    {
                        w.custom_state.apps_page = w.custom_state.apps_page.saturating_sub(1);
                        w.force_data_publish = true;
                        target_id = id.clone();
                        found = true;
                    }
                }
                if found {
                    self.push_current_data_only(&target_id);
                }
            }
            "reset_session" => {
                let mut target_id = widget_id.clone();
                let mut found = false;
                {
                    let mut state = self.state.lock_safe();
                    if let Some((id, w)) = state
                        .widgets
                        .iter_mut()
                        .find(|(id, _)| matches_widget_id(id, &widget_id))
                    {
                        w.force_data_publish = true;
                        target_id = id.clone();
                        found = true;
                    } else if state.widgets.len() == 1
                        && let Some((id, w)) = state.widgets.iter_mut().next()
                    {
                        w.force_data_publish = true;
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
                    self.push_current_data_only(&target_id);
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
                            w.template_kind = TemplateKind::Settings;
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
                                w.template_kind = TemplateKind::Settings;
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
                            w.template_kind = TemplateKind::Live;
                            w.draft_state = None;
                            w.customization_requested_at = None;
                            w.force_data_publish = true;
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
                                w.template_kind = TemplateKind::Live;
                                w.draft_state = None;
                                w.customization_requested_at = None;
                                w.force_data_publish = true;
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
                    w.force_data_publish = true;
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
                    w.force_data_publish = true;
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
                        template_kind: TemplateKind::Live,
                        custom_state: WidgetConfig::default(),
                        draft_state: None,
                        customization_requested_at: None,
                        last_data_hash: None,
                        last_data_json: None,
                        last_data_published_at: None,
                        force_data_publish: true,
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
                        w.template_kind = TemplateKind::Live;
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
                    w.template_kind = TemplateKind::Settings;
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
                        template_kind: TemplateKind::Settings,
                        custom_state: parsed_config.clone(),
                        draft_state: Some(parsed_config.clone()),
                        customization_requested_at: Some(Instant::now()),
                        last_data_hash: None,
                        last_data_json: None,
                        last_data_published_at: None,
                        force_data_publish: true,
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
            let size_str = widget_size_to_str(board_size);
            let template = build_adaptive_card_template(size_str);
            let data = build_adaptive_card_data_string(&snapshot, &board_config, size_str);
            let state_json =
                serde_json::to_string(&board_config).unwrap_or_else(|_| "{}".to_string());
            update_widget_template_and_data(&manager, &target_id, &template, &data, &state_json);
            if target_id != id {
                update_widget_template_and_data(&manager, &id, &template, &data, &state_json);
            }
        }
        Ok(())
    }
}

impl NetFlowWidgetProvider_Impl {
    /// Push the correct card (template + data) for a widget based on its current state.
    fn push_current_card(&self, widget_id: &str) {
        let (actual_id, size, config, in_customization, template_kind) = {
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
                        w.template_kind,
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
                                w.template_kind,
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
            "push_current_card req_id={} -> actual_id={} size={:?} in_custom={} kind={:?}",
            widget_id, actual_id, size, in_customization, template_kind
        ));

        let size_str = widget_size_to_str(size);
        let (template, data_json, state_json) = if in_customization
            || template_kind == TemplateKind::Settings
        {
            let settings_card = build_settings_card_for_size(
                &config,
                size_str,
                snapshot.session_duration_secs,
                &snapshot,
            );
            let state_json = serde_json::to_string(&config).unwrap_or_else(|_| "{}".to_string());
            (settings_card, "{}".to_string(), state_json)
        } else {
            let live_template = build_adaptive_card_template(size_str);
            let live_data = build_adaptive_card_data_string(&snapshot, &config, size_str);
            let state_json = serde_json::to_string(&config).unwrap_or_else(|_| "{}".to_string());
            (live_template, live_data, state_json)
        };

        if let Ok(manager) = WidgetManager::GetDefault() {
            update_widget_template_and_data(
                &manager,
                &actual_id,
                &template,
                &data_json,
                &state_json,
            );
            if actual_id != widget_id && !widget_id.is_empty() {
                update_widget_template_and_data(
                    &manager,
                    widget_id,
                    &template,
                    &data_json,
                    &state_json,
                );
            }
            let h = compute_payload_hash(&data_json);
            let mut state = self.state.lock_safe();
            if let Some(w) = state.widgets.get_mut(&actual_id) {
                w.last_data_hash = Some(h);
                w.last_data_json = Some(data_json.clone());
                w.last_data_published_at = Some(Instant::now());
                w.force_data_publish = false;
            }
            if actual_id != widget_id
                && !widget_id.is_empty()
                && let Some(w) = state.widgets.get_mut(widget_id)
            {
                w.last_data_hash = Some(h);
                w.last_data_json = Some(data_json.clone());
                w.last_data_published_at = Some(Instant::now());
                w.force_data_publish = false;
            }
        }
    }

    /// Push dynamic telemetry data ONLY for a live widget without replacing its visual template.
    fn push_current_data_only(&self, widget_id: &str) {
        let (actual_id, size, config) = {
            let state = self.state.lock_safe();
            let found = state
                .widgets
                .iter()
                .find(|(id, _)| matches_widget_id(id, widget_id))
                .map(|(id, w)| (id.clone(), w.size, w.custom_state.clone()));

            match found {
                Some(data) => data,
                None => {
                    if state.widgets.len() == 1 {
                        if let Some((id, w)) = state.widgets.iter().next() {
                            (id.clone(), w.size, w.custom_state.clone())
                        } else {
                            return;
                        }
                    } else {
                        return;
                    }
                }
            }
        };

        let snapshot = {
            let state = self.state.lock_safe();
            state.latest_snapshot.read_safe().clone()
        };

        let size_str = widget_size_to_str(size);
        let data_json = build_adaptive_card_data_string(&snapshot, &config, size_str);

        if let Ok(manager) = WidgetManager::GetDefault() {
            update_widget_data_only(&manager, &actual_id, &data_json);
            if actual_id != widget_id && !widget_id.is_empty() {
                update_widget_data_only(&manager, widget_id, &data_json);
            }
            let h = compute_payload_hash(&data_json);
            let mut state = self.state.lock_safe();
            if let Some(w) = state.widgets.get_mut(&actual_id) {
                w.last_data_hash = Some(h);
                w.last_data_json = Some(data_json.clone());
                w.last_data_published_at = Some(Instant::now());
                w.force_data_publish = false;
            }
            if actual_id != widget_id
                && !widget_id.is_empty()
                && let Some(w) = state.widgets.get_mut(widget_id)
            {
                w.last_data_hash = Some(h);
                w.last_data_json = Some(data_json);
                w.last_data_published_at = Some(Instant::now());
                w.force_data_publish = false;
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

    let theme = parsed
        .get("theme")
        .and_then(|v| v.as_str())
        .map(ThemeMode::from_str_value)
        .unwrap_or(current_config.theme);

    let graph_style = parsed
        .get("graph_style")
        .and_then(|v| v.as_str())
        .map(GraphStyle::from_str_value)
        .unwrap_or(current_config.graph_style);

    // Adapter selector submits "auto" for aggregate traffic or a decimal LUID string.
    let selected_adapter_luid = match parsed.get("adapter_luid").and_then(|v| v.as_str()) {
        None => current_config.selected_adapter_luid,
        Some(raw) => {
            let trimmed = raw.trim();
            if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("auto") {
                None
            } else {
                trimmed
                    .parse::<u64>()
                    .ok()
                    .or(current_config.selected_adapter_luid)
            }
        }
    };

    // Alert preferences are mirrored into the global engine when settings are saved.
    let mut alerts = current_config.alerts.clone();
    if let Some(enabled) = parse_bool_field(parsed.get("alerts_enabled")) {
        alerts.enabled = enabled;
    }
    if let Some(mbps) = parse_number_field(parsed.get("alert_threshold_mbps")) {
        alerts.threshold_bps = (mbps.max(1) as u64) * 1024 * 1024;
    }
    if let Some(secs) = parse_number_field(parsed.get("alert_sustain_secs")) {
        alerts.sustain_secs = secs.max(1) as u32;
    }
    let alerts = alerts.normalized();

    WidgetConfig {
        speed_unit,
        chart_window,
        apps_expanded,
        apps_page: 0,
        theme,
        graph_style,
        selected_adapter_luid,
        alerts,
    }
}

/// Interprets an Adaptive Card toggle/boolean input that may arrive as a bool or string.
fn parse_bool_field(value: Option<&serde_json::Value>) -> Option<bool> {
    let v = value?;
    if let Some(b) = v.as_bool() {
        Some(b)
    } else {
        v.as_str()
            .map(|s| s.eq_ignore_ascii_case("true") || s == "1")
    }
}

/// Interprets an Adaptive Card numeric input that may arrive as a number or string.
fn parse_number_field(value: Option<&serde_json::Value>) -> Option<i64> {
    match value {
        Some(v) => {
            if let Some(n) = v.as_i64() {
                Some(n)
            } else if let Some(n) = v.as_f64() {
                Some(n as i64)
            } else if let Some(s) = v.as_str() {
                s.trim().parse::<f64>().ok().map(|n| n as i64)
            } else {
                None
            }
        }
        None => None,
    }
}

/// Determines whether the adaptive data publication cadence has elapsed or should be bypassed.
///
/// Priority:
/// 1. `force_ui` (user interaction) -> immediate bypass.
/// 2. Traffic rate acceleration (e.g. sudden burst while previously idle) -> immediate trigger.
/// 3. Normal cadence interval (`last_generation_at.elapsed() >= adaptive_interval`).
pub fn is_publication_cadence_due(
    force_ui: bool,
    adaptive_interval: Duration,
    last_generation_at: Instant,
    last_data_published_at: Instant,
) -> bool {
    if force_ui {
        return true;
    }
    let traffic_spiked = adaptive_interval < Duration::from_millis(1500)
        && last_data_published_at.elapsed() >= adaptive_interval;
    last_generation_at.elapsed() >= adaptive_interval || traffic_spiked
}

/// Background worker loop that samples network telemetry every 250ms
/// and pushes updated Adaptive Cards to active board widgets using an
/// adaptive publication cadence and collision-free payload diffing.
fn worker_loop(
    shutdown: Arc<(Mutex<bool>, Condvar)>,
    state: Arc<Mutex<ProviderState>>,
    backend: Arc<Mutex<NetworkBackend>>,
    snapshot_ref: Arc<RwLock<NetworkSnapshot>>,
    ui_dirty: Arc<AtomicBool>,
    alert_config: Arc<Mutex<BandwidthAlertConfig>>,
) {
    let sample_period = Duration::from_millis(SAMPLING_INTERVAL_MS);
    let persist_period = Duration::from_secs(5);
    let mut next_sample = Instant::now() + sample_period;
    let mut _last_sampled_at = Instant::now();
    let mut last_data_published_at = Instant::now();
    let mut last_generation_at = Instant::now();
    let mut last_persist = Instant::now();
    let mut alert_engine = BandwidthAlertEngine::default();

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
        _last_sampled_at = started;
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
        let snapshot = snapshot_ref.read_safe().clone();
        let alert_preferences = alert_config.lock_safe().clone();
        for event in alert_engine.evaluate(
            &alert_preferences,
            snapshot.rx_bps,
            snapshot.tx_bps,
            Instant::now(),
        ) {
            if let Err(error) = crate::toast::show_bandwidth_alert(event) {
                log_widget(&format!("Bandwidth toast failed: {error}"));
            }
        }
        let adaptive_interval = compute_adaptive_ui_interval(snapshot.rx_bps, snapshot.tx_bps);

        if !is_publication_cadence_due(
            force_ui,
            adaptive_interval,
            last_generation_at,
            last_data_published_at,
        ) {
            continue;
        }
        last_generation_at = Instant::now();

        let targets: Vec<WidgetTarget> = {
            let s = state.lock_safe();
            s.widgets
                .values()
                .filter(|w| w.is_active)
                .map(|w| WidgetTarget {
                    id: w.id.clone(),
                    size: w.size,
                    config: w.custom_state.clone(),
                    template_kind: w.template_kind,
                    in_customization: w.in_customization,
                    last_data_hash: w.last_data_hash,
                    last_data_json: w.last_data_json.clone(),
                    last_data_published_at: w.last_data_published_at,
                    force_data_publish: w.force_data_publish,
                })
                .collect()
        };

        if targets.is_empty() {
            continue;
        }

        if let Ok(manager) = WidgetManager::GetDefault() {
            let mut published_any = false;
            for target in &targets {
                if target.in_customization || target.template_kind != TemplateKind::Live {
                    continue;
                }
                let data_json = build_adaptive_card_data_string(
                    &snapshot,
                    &target.config,
                    widget_size_to_str(target.size),
                );
                let current_hash = compute_payload_hash(&data_json);
                let must_force = force_ui || target.force_data_publish;
                if !should_publish_data(
                    must_force,
                    current_hash,
                    &data_json,
                    target.last_data_hash,
                    target.last_data_json.as_deref(),
                    target.last_data_published_at,
                ) {
                    log_widget_verbose(&format!(
                        "UpdateWidget (Data only) id={}: skipped (payload identical, heartbeat not due)",
                        target.id
                    ));
                    continue;
                }

                update_widget_data_only(&manager, &target.id, &data_json);
                published_any = true;

                let mut s = state.lock_safe();
                if let Some(w) = s.widgets.get_mut(&target.id) {
                    w.last_data_hash = Some(current_hash);
                    w.last_data_json = Some(data_json);
                    w.last_data_published_at = Some(Instant::now());
                    w.force_data_publish = false;
                }
            }
            if published_any {
                last_data_published_at = Instant::now();
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
            template_kind: TemplateKind::Live,
            in_customization: false,
            custom_state: WidgetConfig::default(),
            draft_state: None,
            customization_requested_at: None,
            last_data_hash: None,
            last_data_json: None,
            last_data_published_at: None,
            force_data_publish: false,
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
            template_kind: TemplateKind::Live,
            in_customization: true,
            custom_state: WidgetConfig::default(),
            draft_state: Some(WidgetConfig::default()),
            // Simulated stale transition older than 1.5s
            customization_requested_at: Some(Instant::now() - Duration::from_secs(5)),
            last_data_hash: None,
            last_data_json: None,
            last_data_published_at: None,
            force_data_publish: false,
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

    #[test]
    fn test_telemetry_update_does_not_set_template() {
        unsafe {
            #[link(name = "ole32")]
            unsafe extern "system" {
                fn CoInitializeEx(
                    pv_reserved: *const core::ffi::c_void,
                    dw_co_init: u32,
                ) -> windows_core::HRESULT;
            }
            let _ = CoInitializeEx(core::ptr::null(), 0);
        }

        match build_data_update_options("test-widget-telemetry", r#"{"downloadRate":"12.5 MB/s"}"#)
        {
            Ok(opts) => {
                assert_eq!(
                    opts.WidgetId()
                        .map(|s| s.to_string_lossy())
                        .unwrap_or_default(),
                    "test-widget-telemetry"
                );
                assert_eq!(
                    opts.Data().map(|s| s.to_string_lossy()).unwrap_or_default(),
                    r#"{"downloadRate":"12.5 MB/s"}"#
                );
                // Invariant: Template must NOT be set on telemetry data updates
                let template = opts
                    .Template()
                    .map(|s| s.to_string_lossy())
                    .unwrap_or_default();
                assert!(
                    template.is_empty(),
                    "Template must remain unset in data-only updates"
                );
                // Invariant: CustomState must NOT be set on telemetry data updates
                let custom_state = opts
                    .CustomState()
                    .map(|s| s.to_string_lossy())
                    .unwrap_or_default();
                assert!(
                    custom_state.is_empty(),
                    "CustomState must remain unset in data-only updates"
                );
            }
            Err(e) => {
                // If running in an uncontained unit test environment where WinRT activation
                // for Microsoft.Windows.Widgets.Providers is not registered in the test runner,
                // verify that ClassNotRegistered is the only error code received.
                assert_eq!(
                    e.code().0 as u32,
                    0x80040154,
                    "Expected ClassNotRegistered in bare test runner: {:?}",
                    e
                );
            }
        }
    }

    #[test]
    fn test_payload_hash_consistency() {
        let p1 = r#"{"downloadRate":"10 KB/s","uploadRate":"2 KB/s"}"#;
        let p2 = r#"{"downloadRate":"10 KB/s","uploadRate":"2 KB/s"}"#;
        let p3 = r#"{"downloadRate":"11 KB/s","uploadRate":"2 KB/s"}"#;

        assert_eq!(compute_payload_hash(p1), compute_payload_hash(p2));
        assert_ne!(compute_payload_hash(p1), compute_payload_hash(p3));
    }

    #[test]
    fn test_should_publish_data_forced() {
        let json = r#"{"downloadRate":"0 B/s"}"#;
        let hash = compute_payload_hash(json);

        // Even with matching hash, matching string, and fresh publication timestamp,
        // force = true MUST trigger publication (e.g. user paged apps or clicked reset session).
        let publish = should_publish_data(
            true,
            hash,
            json,
            Some(hash),
            Some(json),
            Some(Instant::now()),
        );
        assert!(publish, "force = true must always publish");
    }

    #[test]
    fn test_should_publish_data_skips_identical_payload() {
        let json = r#"{"downloadRate":"0 B/s","sessionText":"Session (1m)"}"#;
        let hash = compute_payload_hash(json);
        let recent = Some(Instant::now());

        let publish = should_publish_data(false, hash, json, Some(hash), Some(json), recent);
        assert!(
            !publish,
            "Identical payload within heartbeat window must be skipped"
        );
    }

    #[test]
    fn test_should_publish_data_detects_hash_or_string_change() {
        let json1 = r#"{"downloadRate":"10 KB/s"}"#;
        let json2 = r#"{"downloadRate":"20 KB/s"}"#;
        let hash1 = compute_payload_hash(json1);
        let hash2 = compute_payload_hash(json2);
        let recent = Some(Instant::now());

        // Hash differs
        assert!(should_publish_data(
            false,
            hash2,
            json2,
            Some(hash1),
            Some(json1),
            recent
        ));

        // Theoretical hash collision simulation: hashes match but strings differ
        assert!(
            should_publish_data(false, hash1, json2, Some(hash1), Some(json1), recent),
            "String mismatch must publish even if hashes were to collide"
        );
    }

    #[test]
    fn test_should_publish_data_heartbeat() {
        let json = r#"{"downloadRate":"0 B/s"}"#;
        let hash = compute_payload_hash(json);
        // Stale publication timestamp beyond REDUNDANT_UPDATE_HEARTBEAT (15s)
        let expired = Some(Instant::now() - Duration::from_secs(20));

        let publish = should_publish_data(false, hash, json, Some(hash), Some(json), expired);
        assert!(
            publish,
            "Expired heartbeat safety interval must trigger publication even if payload is identical"
        );
    }

    #[test]
    fn test_is_publication_cadence_due() {
        let now = Instant::now();
        let interval_500ms = Duration::from_millis(500);
        let interval_1500ms = Duration::from_millis(1500);

        // 1. force_ui always triggers immediately regardless of elapsed duration
        assert!(
            is_publication_cadence_due(true, interval_500ms, now, now),
            "force_ui must bypass cadence immediately"
        );

        // 2. Normal wait: interval has not elapsed
        let recent_gen = now - Duration::from_millis(200);
        let recent_pub = now - Duration::from_millis(200);
        assert!(
            !is_publication_cadence_due(false, interval_500ms, recent_gen, recent_pub),
            "Must wait when elapsed time is less than adaptive interval"
        );

        // 3. Normal elapsed: interval has elapsed
        let past_gen = now - Duration::from_millis(505);
        assert!(
            is_publication_cadence_due(false, interval_500ms, past_gen, recent_pub),
            "Must trigger once adaptive interval has elapsed"
        );

        // 4. Traffic spike from idle:
        // When traffic accelerates to 500ms cadence and widget hasn't published in >= 500ms,
        // it triggers immediately even if the last generation check was 250ms ago.
        let idle_last_pub = now - Duration::from_millis(1200);
        let recent_idle_gen = now - Duration::from_millis(250);
        assert!(
            is_publication_cadence_due(false, interval_500ms, recent_idle_gen, idle_last_pub),
            "Traffic spike from idle must trigger immediate publication"
        );

        // 5. In steady-state idle (1500ms interval), waiting continues until 1500ms elapses
        assert!(
            !is_publication_cadence_due(false, interval_1500ms, recent_idle_gen, idle_last_pub),
            "Steady-state idle must wait for full 1500ms cadence"
        );
    }
}
