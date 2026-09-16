use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
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
use net_flow_core::card::{WidgetConfig, build_adaptive_card, build_settings_card};
use net_flow_core::format::SpeedUnit;
use net_flow_core::{SAMPLING_INTERVAL_MS, UPDATE_INTERVAL_MS};

pub fn widget_size_to_str(size: WidgetSize) -> &'static str {
    match size {
        WidgetSize::Small => "Small",
        WidgetSize::Large => "Large",
        _ => "Medium",
    }
}

/// Commands sent from COM callbacks to the worker thread.
pub enum WorkerCommand {
    ResetSession,
    Shutdown,
}

pub struct InternalWidgetInfo {
    pub id: String,
    pub size: WidgetSize,
    pub is_active: bool,
    pub in_customization: bool,
    pub custom_state: WidgetConfig,
    pub draft_state: Option<WidgetConfig>,
}

pub struct ProviderState {
    pub widgets: HashMap<String, InternalWidgetInfo>,
    pub active_count: usize,
    pub worker: Option<WorkerHandle>,
    pub cmd_tx: Option<mpsc::Sender<WorkerCommand>>,
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
            let mut b = backend.lock().unwrap();
            b.sample().unwrap_or_default()
        };
        Self {
            widgets: HashMap::new(),
            active_count: 0,
            worker: None,
            cmd_tx: None,
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
        *lock.lock().unwrap() = true;
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
        let mut state = self.state.lock().unwrap();
        if state.worker.is_none() {
            let shutdown = Arc::new((Mutex::new(false), Condvar::new()));
            let shutdown_clone = Arc::clone(&shutdown);
            let state_clone = Arc::clone(&self.state);
            let backend_clone = Arc::clone(&state.backend);
            let snapshot_ref = Arc::clone(&state.latest_snapshot);
            let ui_dirty = Arc::clone(&state.ui_dirty);

            let (cmd_tx, cmd_rx) = mpsc::channel();
            state.cmd_tx = Some(cmd_tx);

            let handle = std::thread::spawn(move || {
                worker_loop(
                    shutdown_clone,
                    state_clone,
                    backend_clone,
                    snapshot_ref,
                    ui_dirty,
                    cmd_rx,
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

pub fn log_widget(msg: &str) {
    use std::io::Write;
    if let Ok(temp) = std::env::var("TEMP") {
        let path = std::path::Path::new(&temp).join("netflow_widget.log");
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
        {
            let _ = writeln!(f, "[{:?}] {}", std::time::SystemTime::now(), msg);
        }
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
        let settings_card = build_settings_card(config);
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
            log_widget(&format!(
                "UpdateWidget id={} in_custom={}: {:?}",
                widget_id, in_customization, res
            ));
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
            let state = self.state.lock().unwrap();
            state
                .widgets
                .get(&id)
                .map(|w| w.custom_state.clone())
                .unwrap_or_default()
        };

        {
            let mut state = self.state.lock().unwrap();
            state.widgets.insert(
                id.clone(),
                InternalWidgetInfo {
                    id: id.clone(),
                    size,
                    is_active: false,
                    in_customization: false,
                    custom_state: config.clone(),
                    draft_state: None,
                },
            );
        }

        // Push initial card from latest snapshot
        let snapshot = {
            let state = self.state.lock().unwrap();
            state.latest_snapshot.read().unwrap().clone()
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
            let mut state = self.state.lock().unwrap();
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
                    let state = self.state.lock().unwrap();
                    let current = state
                        .widgets
                        .iter()
                        .find(|(id, _)| matches_widget_id(id, &widget_id))
                        .map(|(id, w)| {
                            target_id = id.clone();
                            w.custom_state.clone()
                        })
                        .unwrap_or_default();
                    parse_settings_form(&data_json, &current)
                };
                {
                    let mut state = self.state.lock().unwrap();
                    let mut found = false;
                    for (id, w) in state.widgets.iter_mut() {
                        if matches_widget_id(id, &target_id) {
                            w.custom_state = new_config.clone();
                            w.in_customization = false;
                            w.draft_state = None;
                            target_id = id.clone();
                            found = true;
                        }
                    }
                    if !found {
                        for (id, w) in state.widgets.iter_mut() {
                            w.custom_state = new_config.clone();
                            w.in_customization = false;
                            w.draft_state = None;
                            target_id = id.clone();
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
                {
                    let mut state = self.state.lock().unwrap();
                    for (id, w) in state.widgets.iter_mut() {
                        if matches_widget_id(id, &widget_id) {
                            w.custom_state.apps_expanded = !w.custom_state.apps_expanded;
                            w.custom_state.apps_page = 0;
                            target_id = id.clone();
                        }
                    }
                }
                self.push_current_card(&target_id);
            }
            "next_apps_page" => {
                let mut target_id = widget_id.clone();
                {
                    let mut state = self.state.lock().unwrap();
                    for (id, w) in state.widgets.iter_mut() {
                        if matches_widget_id(id, &widget_id) {
                            w.custom_state.apps_page = w.custom_state.apps_page.saturating_add(1);
                            target_id = id.clone();
                        }
                    }
                }
                self.push_current_card(&target_id);
            }
            "prev_apps_page" => {
                let mut target_id = widget_id.clone();
                {
                    let mut state = self.state.lock().unwrap();
                    for (id, w) in state.widgets.iter_mut() {
                        if matches_widget_id(id, &widget_id) {
                            w.custom_state.apps_page = w.custom_state.apps_page.saturating_sub(1);
                            target_id = id.clone();
                        }
                    }
                }
                self.push_current_card(&target_id);
            }
            "reset_session" => {
                let mut target_id = widget_id.clone();
                {
                    let state = self.state.lock().unwrap();
                    if let Some((id, _)) = state
                        .widgets
                        .iter()
                        .find(|(id, _)| matches_widget_id(id, &widget_id))
                    {
                        target_id = id.clone();
                    }
                    {
                        let mut backend = state.backend.lock().unwrap();
                        backend.reset_session();
                        let s = backend.sample().unwrap_or_default();
                        let mut snap_write = state.latest_snapshot.write().unwrap();
                        *snap_write = s;
                    }
                    state.ui_dirty.store(true, Ordering::SeqCst);
                    if let Some(worker) = &state.worker {
                        worker.shutdown.1.notify_all();
                    }
                }
                self.push_current_card(&target_id);
            }
            "open_settings" => {
                let mut target_id = widget_id.clone();
                {
                    let mut state = self.state.lock().unwrap();
                    for (id, w) in state.widgets.iter_mut() {
                        if matches_widget_id(id, &widget_id) {
                            w.in_customization = true;
                            target_id = id.clone();
                        }
                    }
                }
                self.push_current_card(&target_id);
            }
            "cancel_settings" | "cancel" | "exit" | "exitCustomization" | "dismiss" => {
                log_widget(&format!("Handling cancel_settings for id={widget_id}"));
                let mut target_id = widget_id.clone();
                {
                    let mut state = self.state.lock().unwrap();
                    let mut found = false;
                    for (id, w) in state.widgets.iter_mut() {
                        if matches_widget_id(id, &widget_id) {
                            w.in_customization = false;
                            w.draft_state = None;
                            target_id = id.clone();
                            found = true;
                        }
                    }
                    if !found {
                        for (id, w) in state.widgets.iter_mut() {
                            w.in_customization = false;
                            w.draft_state = None;
                            target_id = id.clone();
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
            let mut state = self.state.lock().unwrap();
            for (wid, w) in state.widgets.iter_mut() {
                if matches_widget_id(wid, &id) {
                    w.size = new_size;
                    target_id = wid.clone();
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
            let mut state = self.state.lock().unwrap();
            let mut became_active = false;
            let mut found = false;
            for (wid, w) in state.widgets.iter_mut() {
                if matches_widget_id(wid, &id) {
                    if !w.is_active {
                        w.is_active = true;
                        became_active = true;
                    }
                    w.size = size;
                    target_id = wid.clone();
                    found = true;
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
            let mut state = self.state.lock().unwrap();
            let mut became_inactive = false;
            for (wid, w) in state.widgets.iter_mut() {
                if matches_widget_id(wid, &id) {
                    if w.is_active {
                        w.is_active = false;
                        became_inactive = true;
                    }
                    // When the widget is deactivated (e.g. board closed or widget hidden),
                    // reset customization mode so reopening never stays stuck in settings!
                    w.in_customization = false;
                    w.draft_state = None;
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
            let state = self.state.lock().unwrap();
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

        {
            let mut state = self.state.lock().unwrap();
            let mut found = false;
            for (wid, w) in state.widgets.iter_mut() {
                if matches_widget_id(wid, &id) {
                    w.in_customization = true;
                    w.draft_state = Some(parsed_config.clone());
                    w.custom_state = parsed_config.clone();
                    target_id = wid.clone();
                    found = true;
                }
            }
            if !found {
                state.widgets.insert(
                    id.clone(),
                    InternalWidgetInfo {
                        id: id.clone(),
                        size: ctx.Size().unwrap_or(WidgetSize::Medium),
                        is_active: true,
                        in_customization: true,
                        custom_state: parsed_config.clone(),
                        draft_state: Some(parsed_config),
                    },
                );
            }
        }

        self.push_current_card(&target_id);
        if target_id != id {
            self.push_current_card(&id);
        }
        Ok(())
    }
}

impl NetFlowWidgetProvider_Impl {
    /// Push the correct card for a widget based on its current state.
    fn push_current_card(&self, widget_id: &str) {
        let (actual_id, size, config, in_customization) = {
            let state = self.state.lock().unwrap();
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
                    if let Some((id, w)) = state.widgets.iter().next() {
                        (
                            id.clone(),
                            w.size,
                            w.custom_state.clone(),
                            w.in_customization,
                        )
                    } else {
                        (
                            widget_id.to_string(),
                            WidgetSize::Medium,
                            WidgetConfig::default(),
                            false,
                        )
                    }
                }
            }
        };

        let snapshot = {
            let state = self.state.lock().unwrap();
            state.latest_snapshot.read().unwrap().clone()
        };

        log_widget(&format!(
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
    cmd_rx: mpsc::Receiver<WorkerCommand>,
) {
    let sample_period = Duration::from_millis(SAMPLING_INTERVAL_MS);
    let ui_period = Duration::from_millis(UPDATE_INTERVAL_MS);
    let persist_period = Duration::from_secs(5);
    let mut next_sample = Instant::now() + sample_period;
    let mut last_ui_update = Instant::now();
    let mut last_persist = Instant::now();

    loop {
        while let Ok(cmd) = cmd_rx.try_recv() {
            match cmd {
                WorkerCommand::ResetSession => {
                    let mut b = backend.lock().unwrap();
                    b.reset_session();
                    let s = b.sample().unwrap_or_default();
                    let mut snap_write = snapshot_ref.write().unwrap();
                    *snap_write = s;
                    ui_dirty.store(true, Ordering::SeqCst);
                }
                WorkerCommand::Shutdown => {
                    if let Ok(b) = backend.lock() {
                        b.persist_session();
                    }
                    return;
                }
            }
        }

        let wait = next_sample.saturating_duration_since(Instant::now());
        let (lock, cvar) = &*shutdown;
        let guard = lock.lock().unwrap();
        let (guard, _) = cvar.wait_timeout(guard, wait).unwrap();
        if *guard {
            break;
        }
        drop(guard);

        let started = Instant::now();
        {
            let mut b = backend.lock().unwrap();
            match b.sample() {
                Ok(s) => {
                    let mut snap_write = snapshot_ref.write().unwrap();
                    *snap_write = s;
                }
                Err(_) => {}
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

        let snapshot = snapshot_ref.read().unwrap().clone();

        let targets: Vec<WidgetTarget> = {
            let s = state.lock().unwrap();
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

    if let Ok(b) = backend.lock() {
        b.persist_session();
    }
}
