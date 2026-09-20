use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};

pub type InterfaceLuid = u64;

/// Maximum number of history samples to retain (60s at the sampling cadence).
pub const HISTORY_CAPACITY: usize =
    (crate::MAX_CHART_WINDOW_SECS as u64 * 1000 / crate::SAMPLING_INTERVAL_MS) as usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterfaceCategory {
    Physical,
    Loopback,
    Tunnel,
    Virtual,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum InterfaceMedium {
    #[default]
    Wifi,
    Ethernet,
    Cellular,
    Virtual,
    Loopback,
    Other,
}

impl InterfaceMedium {
    pub fn icon(&self) -> &'static str {
        match self {
            InterfaceMedium::Wifi => "🛜",
            InterfaceMedium::Ethernet => "🖧",
            InterfaceMedium::Cellular => "📱",
            InterfaceMedium::Virtual => "🌐",
            InterfaceMedium::Loopback => "🔄",
            InterfaceMedium::Other => "🌐",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            InterfaceMedium::Wifi => "Wi-Fi",
            InterfaceMedium::Ethernet => "Ethernet",
            InterfaceMedium::Cellular => "Cellular",
            InterfaceMedium::Virtual => "Virtual",
            InterfaceMedium::Loopback => "Loopback",
            InterfaceMedium::Other => "Network",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AggregateMode {
    /// Ethernet + Wi-Fi physical adapters only.
    /// Excludes VPN, tunnel, loopback, virtual.
    #[default]
    PhysicalTransport,

    /// All interfaces except loopback.
    AllNonLoopback,

    /// Explicit category filters.
    Filter {
        include_physical: bool,
        include_virtual: bool,
        include_tunnel: bool,
        include_loopback: bool,
    },
}

impl AggregateMode {
    pub fn matches(&self, category: InterfaceCategory) -> bool {
        match self {
            AggregateMode::PhysicalTransport => category == InterfaceCategory::Physical,
            AggregateMode::AllNonLoopback => category != InterfaceCategory::Loopback,
            AggregateMode::Filter {
                include_physical,
                include_virtual,
                include_tunnel,
                include_loopback,
            } => match category {
                InterfaceCategory::Physical => *include_physical,
                InterfaceCategory::Virtual => *include_virtual,
                InterfaceCategory::Tunnel => *include_tunnel,
                InterfaceCategory::Loopback => *include_loopback,
                InterfaceCategory::Unknown => false,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceInfo {
    pub luid: InterfaceLuid,
    pub index: u32,
    pub name: String,
    pub description: String,
    pub category: InterfaceCategory,
    pub medium: InterfaceMedium,
    pub oper_status: i32, // 1 = Up
    pub in_octets: u64,
    pub out_octets: u64,
    pub speed: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceSample {
    pub luid: InterfaceLuid,
    pub name: String,
    pub category: InterfaceCategory,
    pub medium: InterfaceMedium,
    pub rx_bps: f64,
    pub tx_bps: f64,
}

/// A fixed-duration telemetry sample representing bandwidth across one bucket (e.g. 500ms).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistorySample {
    /// Bytes received during this sample bucket.
    #[serde(default)]
    pub rx_bytes: u64,
    /// Bytes transmitted during this sample bucket.
    #[serde(default)]
    pub tx_bytes: u64,
    /// Duration of this bucket in nanoseconds (e.g. 500_000_000 ns for 500ms).
    #[serde(default)]
    pub duration_ns: u64,
    /// Average download speed in B/s over this interval.
    pub rx_bps: u64,
    /// Average upload speed in B/s over this interval.
    pub tx_bps: u64,
}

impl HistorySample {
    pub fn new(rx_bytes: u64, tx_bytes: u64, duration_ns: u64) -> Self {
        let rx_bps = if duration_ns > 0 {
            ((rx_bytes as u128 * 1_000_000_000) / duration_ns as u128) as u64
        } else {
            0
        };
        let tx_bps = if duration_ns > 0 {
            ((tx_bytes as u128 * 1_000_000_000) / duration_ns as u128) as u64
        } else {
            0
        };
        Self {
            rx_bytes,
            tx_bytes,
            duration_ns,
            rx_bps,
            tx_bps,
        }
    }

    pub fn from_bps(rx_bps: u64, tx_bps: u64, duration_ns: u64) -> Self {
        let rx_bytes = ((rx_bps as u128 * duration_ns as u128) / 1_000_000_000) as u64;
        let tx_bytes = ((tx_bps as u128 * duration_ns as u128) / 1_000_000_000) as u64;
        Self {
            rx_bytes,
            tx_bytes,
            duration_ns,
            rx_bps,
            tx_bps,
        }
    }
}

/// Persistent telemetry metrics saved across widget restarts and session resets.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SessionState {
    pub session_rx: u64,
    pub session_tx: u64,
    pub session_start_unix: u64,
    pub all_time_peak_rx: f64,
    pub all_time_peak_tx: f64,
}

impl Default for SessionState {
    fn default() -> Self {
        let now_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            session_rx: 0,
            session_tx: 0,
            session_start_unix: now_unix,
            all_time_peak_rx: 0.0,
            all_time_peak_tx: 0.0,
        }
    }
}

pub fn get_session_state_path() -> std::path::PathBuf {
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        std::path::PathBuf::from(local_app_data)
            .join("NetFlow")
            .join("session_state.json")
    } else {
        std::env::temp_dir()
            .join("NetFlow")
            .join("session_state.json")
    }
}

pub fn load_persisted_session_state() -> SessionState {
    let path = get_session_state_path();
    if let Ok(data) = std::fs::read_to_string(&path)
        && let Ok(state) = serde_json::from_str::<SessionState>(&data)
    {
        return state;
    }
    SessionState::default()
}

pub fn save_persisted_session_state(state: &SessionState) {
    let path = get_session_state_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(state) {
        let _ = std::fs::write(&path, json);
    }
}

/// Accumulates measurement intervals and slices them into discrete buckets (default 500ms).
///
/// Because timer ticks on Windows can have minor jitter (e.g. 485ms or 515ms),
/// this distributes bytes proportionally across bucket boundaries so chart points
/// represent uniform time intervals without dropping or fabricating bytes.
#[derive(Clone, Debug, Default)]
pub struct RateAccumulator {
    time_acc_ns: u64,
    bytes_rx_acc: u64,
    bytes_tx_acc: u64,
}

impl RateAccumulator {
    pub fn new() -> Self {
        Self {
            time_acc_ns: 0,
            bytes_rx_acc: 0,
            bytes_tx_acc: 0,
        }
    }

    pub fn reset(&mut self) {
        self.time_acc_ns = 0;
        self.bytes_rx_acc = 0;
        self.bytes_tx_acc = 0;
    }

    pub fn time_acc_ns(&self) -> u64 {
        self.time_acc_ns
    }

    pub fn bytes_rx_acc(&self) -> u64 {
        self.bytes_rx_acc
    }

    pub fn bytes_tx_acc(&self) -> u64 {
        self.bytes_tx_acc
    }

    /// Pushes an observed time slice (`elapsed_ns`, `delta_rx`, `delta_tx`) and returns
    /// any completed buckets of duration `slot_ns`.
    pub fn push_sample(
        &mut self,
        elapsed_ns: u64,
        delta_rx: u64,
        delta_tx: u64,
        slot_ns: u64,
    ) -> Vec<HistorySample> {
        if elapsed_ns == 0 || slot_ns == 0 {
            return Vec::new();
        }

        let needed_ns = slot_ns.saturating_sub(self.time_acc_ns);

        // If the sample interval does not cross the current bucket boundary,
        // simply accumulate and return.
        if elapsed_ns < needed_ns {
            self.time_acc_ns += elapsed_ns;
            self.bytes_rx_acc += delta_rx;
            self.bytes_tx_acc += delta_tx;
            return Vec::new();
        }

        let mut emitted = Vec::new();
        let mut consumed_time_ns = 0u64;
        let mut consumed_rx = 0u64;
        let mut consumed_tx = 0u64;

        // 1. Complete the currently in-progress bucket
        let cum_boundary_1 = needed_ns;
        let target_rx_1 = ((delta_rx as u128 * cum_boundary_1 as u128) / elapsed_ns as u128) as u64;
        let target_tx_1 = ((delta_tx as u128 * cum_boundary_1 as u128) / elapsed_ns as u128) as u64;

        let slice_rx_1 = target_rx_1 - consumed_rx;
        let slice_tx_1 = target_tx_1 - consumed_tx;

        let bucket_rx_1 = self.bytes_rx_acc + slice_rx_1;
        let bucket_tx_1 = self.bytes_tx_acc + slice_tx_1;
        emitted.push(HistorySample::new(bucket_rx_1, bucket_tx_1, slot_ns));

        consumed_rx += slice_rx_1;
        consumed_tx += slice_tx_1;
        consumed_time_ns += needed_ns;

        self.time_acc_ns = 0;
        self.bytes_rx_acc = 0;
        self.bytes_tx_acc = 0;

        // 2. Emit any full buckets contained within the remainder of this interval
        while (elapsed_ns - consumed_time_ns) >= slot_ns {
            let cum_boundary = consumed_time_ns + slot_ns;
            let target_rx = ((delta_rx as u128 * cum_boundary as u128) / elapsed_ns as u128) as u64;
            let target_tx = ((delta_tx as u128 * cum_boundary as u128) / elapsed_ns as u128) as u64;

            let slice_rx = target_rx - consumed_rx;
            let slice_tx = target_tx - consumed_tx;

            emitted.push(HistorySample::new(slice_rx, slice_tx, slot_ns));

            consumed_rx += slice_rx;
            consumed_tx += slice_tx;
            consumed_time_ns += slot_ns;
        }

        // 3. Stash remaining residual fraction in accumulator
        let rem_time_ns = elapsed_ns - consumed_time_ns;
        let rem_rx = delta_rx - consumed_rx;
        let rem_tx = delta_tx - consumed_tx;

        self.time_acc_ns = rem_time_ns;
        self.bytes_rx_acc = rem_rx;
        self.bytes_tx_acc = rem_tx;

        emitted
    }
}

/// Timestamped cumulative byte totals used for boundary interpolation in `RollingRateWindow`.
#[derive(Clone, Copy, Debug)]
pub struct CounterPoint {
    pub timestamp: Instant,
    pub total_rx: u64,
    pub total_tx: u64,
}

/// Calculates a smooth 1-second rolling transfer rate by interpolating byte counters at window boundaries.
#[derive(Clone, Debug)]
pub struct RollingRateWindow {
    points: VecDeque<CounterPoint>,
    window: Duration,
}

impl RollingRateWindow {
    pub fn new(window_ns: u64) -> Self {
        Self {
            points: VecDeque::new(),
            window: Duration::from_nanos(window_ns),
        }
    }

    pub fn reset(&mut self) {
        self.points.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    pub fn record_sample(&mut self, now: Instant, total_rx: u64, total_tx: u64) {
        // If there is a massive time gap (> 5s), reset the window
        if let Some(last) = self.points.back()
            && now.saturating_duration_since(last.timestamp) > Duration::from_secs(5)
        {
            self.points.clear();
        }
        self.points.push_back(CounterPoint {
            timestamp: now,
            total_rx,
            total_tx,
        });

        // Prune points that are older than window + extra buffer, but retain at least 1 point
        // older than (now - window) so we can always bracket the boundary.
        if let Some(target_time) = now.checked_sub(self.window) {
            while self.points.len() >= 2 {
                // If the second point is also <= target_time, the first point is unneeded
                if self.points[1].timestamp <= target_time {
                    self.points.pop_front();
                } else {
                    break;
                }
            }
        }
    }

    /// Calculate the rolling rate over the specified window (default 1s) ending at `now`.
    /// Reconstructs the counter at (now - window) using piecewise-constant interpolation.
    pub fn current_rate(&self, now: Instant) -> (f64, f64) {
        let latest = match self.points.back() {
            Some(p) => p,
            None => return (0.0, 0.0),
        };

        let target_time = match now.checked_sub(self.window) {
            Some(t) => t,
            None => return (0.0, 0.0),
        };

        // If we only have 1 point, or the oldest point is newer than target_time:
        let oldest = &self.points[0];
        if self.points.len() == 1 || oldest.timestamp >= target_time {
            let elapsed = now
                .saturating_duration_since(oldest.timestamp)
                .as_secs_f64();
            if elapsed > 0.0 {
                let rx = (latest.total_rx.saturating_sub(oldest.total_rx)) as f64 / elapsed;
                let tx = (latest.total_tx.saturating_sub(oldest.total_tx)) as f64 / elapsed;
                return (rx, tx);
            } else {
                return (0.0, 0.0);
            }
        }

        // Find the bracket: points[i].timestamp <= target_time <= points[i+1].timestamp
        let mut bracket = None;
        for i in 0..self.points.len() - 1 {
            if self.points[i].timestamp <= target_time
                && self.points[i + 1].timestamp >= target_time
            {
                bracket = Some((&self.points[i], &self.points[i + 1]));
                break;
            }
        }

        let (p_start, p_end) = match bracket {
            Some(b) => b,
            None => {
                // Fallback to earliest point
                let elapsed = now
                    .saturating_duration_since(oldest.timestamp)
                    .as_secs_f64();
                if elapsed > 0.0 {
                    let rx = (latest.total_rx.saturating_sub(oldest.total_rx)) as f64 / elapsed;
                    let tx = (latest.total_tx.saturating_sub(oldest.total_tx)) as f64 / elapsed;
                    return (rx, tx);
                } else {
                    return (0.0, 0.0);
                }
            }
        };

        let bracket_duration = p_end
            .timestamp
            .saturating_duration_since(p_start.timestamp)
            .as_secs_f64();
        let (interp_rx, interp_tx) = if bracket_duration > 0.0 {
            let fraction = (target_time
                .saturating_duration_since(p_start.timestamp)
                .as_secs_f64()
                / bracket_duration)
                .clamp(0.0, 1.0);
            let rx_delta = (p_end.total_rx.saturating_sub(p_start.total_rx)) as f64;
            let tx_delta = (p_end.total_tx.saturating_sub(p_start.total_tx)) as f64;
            (
                p_start.total_rx as f64 + fraction * rx_delta,
                p_start.total_tx as f64 + fraction * tx_delta,
            )
        } else {
            (p_start.total_rx as f64, p_start.total_tx as f64)
        };

        let window_secs = self.window.as_secs_f64();
        let rx_bps = ((latest.total_rx as f64 - interp_rx) / window_secs).max(0.0);
        let tx_bps = ((latest.total_tx as f64 - interp_tx) / window_secs).max(0.0);
        (rx_bps, tx_bps)
    }
}

/// Telemetry snapshot containing current rates, totals, active apps, and chart history.
#[derive(Debug, Clone)]
pub struct NetworkSnapshot {
    /// 1-second rolling download speed in bytes/sec.
    pub rx_bps: f64,
    /// 1-second rolling upload speed in bytes/sec.
    pub tx_bps: f64,
    /// Download speed of the most recently finished 500ms bucket.
    pub rx_bps_500ms: u64,
    /// Upload speed of the most recently finished 500ms bucket.
    pub tx_bps_500ms: u64,
    /// Instantaneous download rate over the last raw tick (used for app rate reconciliation).
    pub instant_rx_bps: f64,
    /// Instantaneous upload rate over the last raw tick (used for app rate reconciliation).
    pub instant_tx_bps: f64,
    pub session_rx: u64,
    pub session_tx: u64,
    pub session_duration_secs: u64,
    pub active_interfaces: usize,
    pub peak_rx_bps: f64,
    pub peak_tx_bps: f64,
    pub session_peak_rx_bps: f64,
    pub session_peak_tx_bps: f64,
    pub timestamp: Instant,
    pub per_interface: Vec<InterfaceSample>,
    pub history: Vec<HistorySample>,
    pub primary_medium: InterfaceMedium,
    pub primary_name: String,
    pub active_apps: Vec<crate::process::ActiveAppInfo>,
    pub active_connections_count: usize,
}

impl NetworkSnapshot {
    /// Returns the maximum download and upload rate observed across the last `sample_count` history samples.
    pub fn chart_window_peak(&self, sample_count: usize) -> (f64, f64) {
        let count = sample_count.min(self.history.len());
        let mut max_rx = self.rx_bps;
        let mut max_tx = self.tx_bps;
        if count > 0 {
            let take_from = self.history.len().saturating_sub(count);
            for s in &self.history[take_from..] {
                let rx = s.rx_bps as f64;
                let tx = s.tx_bps as f64;
                if rx > max_rx {
                    max_rx = rx;
                }
                if tx > max_tx {
                    max_tx = tx;
                }
            }
        }
        (max_rx, max_tx)
    }
}

impl Default for NetworkSnapshot {
    fn default() -> Self {
        Self {
            rx_bps: 0.0,
            tx_bps: 0.0,
            rx_bps_500ms: 0,
            tx_bps_500ms: 0,
            instant_rx_bps: 0.0,
            instant_tx_bps: 0.0,
            session_rx: 0,
            session_tx: 0,
            session_duration_secs: 0,
            active_interfaces: 0,
            peak_rx_bps: 0.0,
            peak_tx_bps: 0.0,
            session_peak_rx_bps: 0.0,
            session_peak_tx_bps: 0.0,
            timestamp: Instant::now(),
            per_interface: Vec::new(),
            history: Vec::new(),
            primary_medium: InterfaceMedium::Other,
            primary_name: "Network".to_string(),
            active_apps: Vec::new(),
            active_connections_count: 0,
        }
    }
}

pub fn compute_delta(prev: u64, curr: u64) -> u64 {
    curr.saturating_sub(prev)
}

/// Core telemetry backend that queries Windows network adapters, computes bandwidth, and tracks apps.
pub struct NetworkBackend {
    pub mode: AggregateMode,
    /// Baseline byte counts for all known interfaces to avoid spikes when an adapter comes online.
    prev_counters: HashMap<InterfaceLuid, (u64, u64)>,
    prev_time: Option<Instant>,
    pub peak_rx: f64,
    pub peak_tx: f64,
    pub session_rx: u64,
    pub session_tx: u64,
    pub session_start_unix: u64,
    /// Rolling FIFO buffer of fixed-duration bandwidth samples.
    history: VecDeque<HistorySample>,
    /// Tracked chart peak download rate across current history buffer.
    chart_peak_rx: u64,
    /// Tracked chart peak upload rate across current history buffer.
    chart_peak_tx: u64,
    /// Tracks per-process network and disk I/O rates.
    pub process_tracker: crate::process::ProcessTracker,
    /// Last time the process table was refreshed (throttled to 1s).
    last_process_sample: Option<Instant>,
    /// Cached un-reconciled apps from the last process tracker pass.
    cached_raw_apps: (Vec<crate::process::ActiveAppInfo>, usize),
    /// Sub-sample bucket accumulator for fixed 500ms chart slices.
    pub accumulator: RateAccumulator,
    /// Rolling 1-second window for smooth UI headline rates.
    pub rolling_window: RollingRateWindow,
}

impl Default for NetworkBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkBackend {
    pub fn new() -> Self {
        Self::with_mode(AggregateMode::default())
    }

    pub fn with_mode(mode: AggregateMode) -> Self {
        let now_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            mode,
            prev_counters: HashMap::new(),
            prev_time: None,
            peak_rx: 0.0,
            peak_tx: 0.0,
            session_rx: 0,
            session_tx: 0,
            session_start_unix: now_unix,
            history: VecDeque::with_capacity(HISTORY_CAPACITY),
            chart_peak_rx: 0,
            chart_peak_tx: 0,
            process_tracker: crate::process::ProcessTracker::new(),
            last_process_sample: None,
            cached_raw_apps: (Vec::new(), 0),
            accumulator: RateAccumulator::new(),
            rolling_window: RollingRateWindow::new(1_000_000_000),
        }
    }

    /// Load or restore persistent session state from local storage.
    pub fn load_or_create(mode: AggregateMode) -> Self {
        let persisted = load_persisted_session_state();
        let now_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            mode,
            prev_counters: HashMap::new(),
            prev_time: None,
            peak_rx: persisted.all_time_peak_rx,
            peak_tx: persisted.all_time_peak_tx,
            session_rx: persisted.session_rx,
            session_tx: persisted.session_tx,
            session_start_unix: if persisted.session_start_unix > 0 {
                persisted.session_start_unix
            } else {
                now_unix
            },
            history: VecDeque::with_capacity(HISTORY_CAPACITY),
            chart_peak_rx: 0,
            chart_peak_tx: 0,
            process_tracker: crate::process::ProcessTracker::new(),
            last_process_sample: None,
            cached_raw_apps: (Vec::new(), 0),
            accumulator: RateAccumulator::new(),
            rolling_window: RollingRateWindow::new(1_000_000_000),
        }
    }

    /// Flush session state to disk.
    pub fn persist_session(&self) {
        save_persisted_session_state(&SessionState {
            session_rx: self.session_rx,
            session_tx: self.session_tx,
            session_start_unix: self.session_start_unix,
            all_time_peak_rx: self.peak_rx,
            all_time_peak_tx: self.peak_tx,
        });
    }

    /// Reset session: re-anchor all baselines, clear totals and history.
    pub fn reset_session(&mut self) {
        self.prev_counters.clear();
        self.prev_time = None;
        self.peak_rx = 0.0;
        self.peak_tx = 0.0;
        self.session_rx = 0;
        self.session_tx = 0;
        let now_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        self.session_start_unix = now_unix;
        self.history.clear();
        self.chart_peak_rx = 0;
        self.chart_peak_tx = 0;
        self.accumulator.reset();
        self.rolling_window.reset();
        self.process_tracker.reset();
        self.last_process_sample = None;
        self.cached_raw_apps = (Vec::new(), 0);
        save_persisted_session_state(&SessionState {
            session_rx: 0,
            session_tx: 0,
            session_start_unix: now_unix,
            all_time_peak_rx: 0.0,
            all_time_peak_tx: 0.0,
        });
    }

    /// Primary sampling method: queries IP Helper APIs, computes rates, updates state.
    pub fn sample(&mut self) -> Result<NetworkSnapshot, String> {
        let interfaces = query_interfaces()?;
        let now = Instant::now();
        let mut snapshot = self.sample_from_interfaces(&interfaces, now);

        let need_process_sample = match self.last_process_sample {
            Some(last) => now.duration_since(last) >= Duration::from_millis(1000),
            None => true,
        };

        if need_process_sample {
            self.cached_raw_apps = self.process_tracker.sample(now);
            self.last_process_sample = Some(now);
        }

        // Always clone the pre-reconciliation raw output and reconcile fresh
        // against this tick's network totals, avoiding directional-mismatch distortion!
        let (mut active_apps, active_conns) = self.cached_raw_apps.clone();
        crate::process::reconcile_app_bandwidth(&mut active_apps, snapshot.rx_bps, snapshot.tx_bps);
        snapshot.active_apps = active_apps;
        snapshot.active_connections_count = active_conns;
        Ok(snapshot)
    }

    /// Internal core sampling logic taking an explicit list of interfaces and timestamp.
    /// Used by `sample()` and deterministic unit tests.
    pub fn sample_from_interfaces(
        &mut self,
        interfaces: &[InterfaceInfo],
        now: Instant,
    ) -> NetworkSnapshot {
        let (elapsed_secs, elapsed_ns) = match self.prev_time {
            Some(prev) => match now.checked_duration_since(prev) {
                Some(duration) => (duration.as_secs_f64(), duration.as_nanos() as u64),
                None => (0.0, 0), // Clock jitter or backward step
            },
            None => (0.0, 0), // First sample establishes baseline
        };
        self.prev_time = Some(now);

        let mut seen_luids = HashSet::new();
        let mut total_delta_in = 0u64;
        let mut total_delta_out = 0u64;

        let mut per_interface = Vec::new();

        for iface in interfaces {
            seen_luids.insert(iface.luid);

            // **FIX**: Always update baseline counters for EVERY interface,
            // regardless of oper_status or mode. This prevents false spikes
            // when an interface cycles Down → Up.
            let (delta_in, delta_out) = match self.prev_counters.get(&iface.luid) {
                Some(&(prev_in, prev_out)) => {
                    let d_in = compute_delta(prev_in, iface.in_octets);
                    let d_out = compute_delta(prev_out, iface.out_octets);
                    (d_in, d_out)
                }
                None => {
                    // New interface: baseline established, report 0 delta
                    (0, 0)
                }
            };

            // Unconditionally update baseline to current values
            self.prev_counters
                .insert(iface.luid, (iface.in_octets, iface.out_octets));

            // Only active, mode-matching interfaces contribute to aggregation
            if iface.oper_status != 1 {
                continue;
            }
            if !self.mode.matches(iface.category) {
                continue;
            }

            total_delta_in += delta_in;
            total_delta_out += delta_out;

            let iface_rx_bps = if elapsed_secs > 0.0 {
                delta_in as f64 / elapsed_secs
            } else {
                0.0
            };
            let iface_tx_bps = if elapsed_secs > 0.0 {
                delta_out as f64 / elapsed_secs
            } else {
                0.0
            };

            per_interface.push(InterfaceSample {
                luid: iface.luid,
                name: iface.name.clone(),
                category: iface.category,
                medium: iface.medium,
                rx_bps: iface_rx_bps,
                tx_bps: iface_tx_bps,
            });
        }

        // Sort per_interface descending by total throughput (active traffic first)
        per_interface.sort_by(|a, b| {
            let sum_b = b.rx_bps + b.tx_bps;
            let sum_a = a.rx_bps + a.tx_bps;
            sum_b
                .partial_cmp(&sum_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Count active interfaces: those with non-zero traffic (> 10 B/s) OR if none, the connected physical adapters
        let active_traffic_count = per_interface
            .iter()
            .filter(|i| (i.rx_bps + i.tx_bps) > 10.0)
            .count();
        let reported_active_count = if active_traffic_count > 0 {
            active_traffic_count
        } else {
            let phys_count = per_interface
                .iter()
                .filter(|i| i.category == InterfaceCategory::Physical)
                .count();
            if phys_count > 0 {
                phys_count
            } else if !per_interface.is_empty() {
                1
            } else {
                0
            }
        };

        // Identify primary interface (highest traffic physical/wireless/ethernet, or first physical connected)
        let primary = per_interface
            .iter()
            .find(|i| (i.rx_bps + i.tx_bps) > 0.0 && i.category == InterfaceCategory::Physical)
            .or_else(|| {
                per_interface
                    .iter()
                    .find(|i| i.category == InterfaceCategory::Physical)
            })
            .or_else(|| per_interface.first());

        let (primary_medium, mut primary_name) = match primary {
            Some(p) => (p.medium, p.name.clone()),
            None => (InterfaceMedium::Other, "Network".to_string()),
        };

        if primary_medium == InterfaceMedium::Wifi
            && let Some(ssid) = query_cached_wifi_ssid()
        {
            primary_name = ssid;
        }

        // Prune interfaces that disappeared from the system
        self.prev_counters
            .retain(|luid, _| seen_luids.contains(luid));

        // Accumulate session totals
        self.session_rx += total_delta_in;
        self.session_tx += total_delta_out;

        let instant_rx_bps = if elapsed_secs > 0.0 {
            total_delta_in as f64 / elapsed_secs
        } else {
            0.0
        };
        let instant_tx_bps = if elapsed_secs > 0.0 {
            total_delta_out as f64 / elapsed_secs
        } else {
            0.0
        };

        // Time cadence synchronization:
        // Use integer-nanosecond RateAccumulator with cumulative proportional boundary allocation.
        // A gap > 5 seconds (e.g. PC suspended/slept) represents a new telemetry segment.
        let slot_ns = crate::SAMPLING_INTERVAL_MS * 1_000_000;
        if elapsed_ns > 5_000_000_000 {
            self.history.clear();
            self.chart_peak_rx = 0;
            self.chart_peak_tx = 0;
            self.accumulator.reset();
            self.rolling_window.reset();
            self.rolling_window
                .record_sample(now, self.session_rx, self.session_tx);
        } else if elapsed_ns > 0 {
            let emitted =
                self.accumulator
                    .push_sample(elapsed_ns, total_delta_in, total_delta_out, slot_ns);
            for bucket in emitted {
                if (bucket.rx_bps as f64) > self.peak_rx {
                    self.peak_rx = bucket.rx_bps as f64;
                }
                if (bucket.tx_bps as f64) > self.peak_tx {
                    self.peak_tx = bucket.tx_bps as f64;
                }
                if self.history.len() >= HISTORY_CAPACITY {
                    if let Some(evicted) = self.history.pop_front() {
                        if evicted.rx_bps == self.chart_peak_rx {
                            self.chart_peak_rx =
                                self.history.iter().map(|s| s.rx_bps).max().unwrap_or(0);
                        }
                        if evicted.tx_bps == self.chart_peak_tx {
                            self.chart_peak_tx =
                                self.history.iter().map(|s| s.tx_bps).max().unwrap_or(0);
                        }
                    }
                }
                self.history.push_back(bucket);
                self.chart_peak_rx = self.chart_peak_rx.max(bucket.rx_bps);
                self.chart_peak_tx = self.chart_peak_tx.max(bucket.tx_bps);
            }
            self.rolling_window
                .record_sample(now, self.session_rx, self.session_tx);
        } else {
            // First sample establishes initial baseline point
            self.rolling_window
                .record_sample(now, self.session_rx, self.session_tx);
        }

        let (rx_bps, tx_bps) = self.rolling_window.current_rate(now);
        let rx_bps_500ms = self.history.back().map(|s| s.rx_bps).unwrap_or(0);
        let tx_bps_500ms = self.history.back().map(|s| s.tx_bps).unwrap_or(0);

        let now_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let session_duration_secs = now_unix.saturating_sub(self.session_start_unix);

        NetworkSnapshot {
            rx_bps,
            tx_bps,
            rx_bps_500ms,
            tx_bps_500ms,
            instant_rx_bps,
            instant_tx_bps,
            session_rx: self.session_rx,
            session_tx: self.session_tx,
            session_duration_secs,
            active_interfaces: reported_active_count,
            peak_rx_bps: self.chart_peak_rx as f64,
            peak_tx_bps: self.chart_peak_tx as f64,
            session_peak_rx_bps: self.peak_rx,
            session_peak_tx_bps: self.peak_tx,
            timestamp: now,
            per_interface,
            history: self.history.iter().copied().collect(),
            primary_medium,
            primary_name,
            active_apps: Vec::new(),
            active_connections_count: 0,
        }
    }
}

/// Convert a null-terminated UTF-16 slice to Rust String.
fn wchar_to_string(slice: &[u16]) -> String {
    let len = slice.iter().position(|&c| c == 0).unwrap_or(slice.len());
    String::from_utf16_lossy(&slice[..len])
}

#[allow(non_snake_case, dead_code, clippy::upper_case_acronyms)]
#[repr(C)]
struct DOT11_SSID {
    uSSIDLength: u32,
    ucSSID: [u8; 32],
}

#[allow(non_snake_case, dead_code, clippy::upper_case_acronyms)]
#[repr(C)]
struct WLAN_ASSOCIATION_ATTRIBUTES {
    dot11Ssid: DOT11_SSID,
    dot11BssType: u32,
    dot11Bssid: [u8; 6],
    dot11PhyType: u32,
    uDot11PhyIndex: u32,
    wlanSignalQuality: u32,
    ulRxRate: u32,
    ulTxRate: u32,
}

#[allow(non_snake_case, dead_code, clippy::upper_case_acronyms)]
#[repr(C)]
struct WLAN_SECURITY_ATTRIBUTES {
    bSecurityEnabled: i32,
    bOneXEnabled: i32,
    dot11AuthAlgorithm: u32,
    dot11CipherAlgorithm: u32,
}

#[allow(non_snake_case, dead_code, clippy::upper_case_acronyms)]
#[repr(C)]
struct WLAN_CONNECTION_ATTRIBUTES {
    isState: u32,
    wlanConnectionMode: u32,
    strProfileName: [u16; 256],
    wlanAssociationAttributes: WLAN_ASSOCIATION_ATTRIBUTES,
    wlanSecurityAttributes: WLAN_SECURITY_ATTRIBUTES,
}

#[allow(non_snake_case, dead_code, clippy::upper_case_acronyms)]
#[repr(C)]
struct WLAN_INTERFACE_INFO {
    InterfaceGuid: [u8; 16],
    strInterfaceDescription: [u16; 256],
    isState: u32,
}

#[allow(non_snake_case, dead_code, clippy::upper_case_acronyms)]
#[repr(C)]
struct WLAN_INTERFACE_INFO_LIST {
    dwNumberOfItems: u32,
    dwIndex: u32,
    InterfaceInfo: [WLAN_INTERFACE_INFO; 1],
}

#[link(name = "wlanapi")]
unsafe extern "system" {
    fn WlanOpenHandle(
        dwClientVersion: u32,
        pReserved: *mut core::ffi::c_void,
        pdwNegotiatedVersion: *mut u32,
        phClientHandle: *mut isize,
    ) -> u32;
    fn WlanCloseHandle(hClientHandle: isize, pReserved: *mut core::ffi::c_void) -> u32;
    fn WlanEnumInterfaces(
        hClientHandle: isize,
        pReserved: *mut core::ffi::c_void,
        ppInterfaceList: *mut *mut WLAN_INTERFACE_INFO_LIST,
    ) -> u32;
    fn WlanQueryInterface(
        hClientHandle: isize,
        pInterfaceGuid: *const [u8; 16],
        OpCode: u32,
        pReserved: *mut core::ffi::c_void,
        pdwDataSize: *mut u32,
        ppData: *mut *mut core::ffi::c_void,
        pWlanOpcodeValueType: *mut u32,
    ) -> u32;
    fn WlanFreeMemory(pMemory: *mut core::ffi::c_void);
}

/// Queries the SSID of the currently connected Wi-Fi network using the native Windows WlanAPI.
pub fn query_active_wifi_ssid() -> Option<String> {
    unsafe {
        let mut negotiated = 0u32;
        let mut handle = 0isize;
        if WlanOpenHandle(2, std::ptr::null_mut(), &mut negotiated, &mut handle) != 0 || handle == 0
        {
            return None;
        }

        let mut list_ptr: *mut WLAN_INTERFACE_INFO_LIST = std::ptr::null_mut();
        let enum_res = WlanEnumInterfaces(handle, std::ptr::null_mut(), &mut list_ptr);
        if enum_res != 0 || list_ptr.is_null() {
            WlanCloseHandle(handle, std::ptr::null_mut());
            return None;
        }

        let mut found_ssid: Option<String> = None;
        let count = (*list_ptr).dwNumberOfItems;
        if count > 0 {
            let interfaces =
                std::slice::from_raw_parts((*list_ptr).InterfaceInfo.as_ptr(), count as usize);
            for iface in interfaces {
                // wlan_interface_state_connected = 1
                if iface.isState == 1 {
                    let mut data_size = 0u32;
                    let mut data_ptr: *mut core::ffi::c_void = std::ptr::null_mut();
                    // OpCode 7 = wlan_intf_opcode_current_connection
                    let query_res = WlanQueryInterface(
                        handle,
                        &iface.InterfaceGuid,
                        7,
                        std::ptr::null_mut(),
                        &mut data_size,
                        &mut data_ptr,
                        std::ptr::null_mut(),
                    );
                    if query_res == 0 && !data_ptr.is_null() {
                        let conn_attrs = &*(data_ptr as *const WLAN_CONNECTION_ATTRIBUTES);
                        let ssid_len =
                            conn_attrs.wlanAssociationAttributes.dot11Ssid.uSSIDLength as usize;
                        if ssid_len > 0 && ssid_len <= 32 {
                            let bytes =
                                &conn_attrs.wlanAssociationAttributes.dot11Ssid.ucSSID[..ssid_len];
                            let ssid_lossy = String::from_utf8_lossy(bytes);
                            let trimmed = ssid_lossy.trim_matches(['\0', ' ']);
                            if !trimmed.is_empty() {
                                found_ssid = Some(trimmed.to_string());
                            }
                        }
                        if found_ssid.is_none() {
                            let prof = wchar_to_string(&conn_attrs.strProfileName);
                            let trimmed = prof.trim();
                            if !trimmed.is_empty() {
                                found_ssid = Some(trimmed.to_string());
                            }
                        }
                        WlanFreeMemory(data_ptr);
                    }
                    if found_ssid.is_some() {
                        break;
                    }
                }
            }
        }

        WlanFreeMemory(list_ptr as *mut core::ffi::c_void);
        WlanCloseHandle(handle, std::ptr::null_mut());
        found_ssid
    }
}

static WIFI_SSID_CACHE: std::sync::Mutex<(Option<String>, Option<Instant>)> =
    std::sync::Mutex::new((None, None));

/// Cached Wi-Fi SSID lookup with a 2-second TTL to avoid spamming WlanAPI on every 500ms tick.
pub fn query_cached_wifi_ssid() -> Option<String> {
    const CACHE_TTL: std::time::Duration = std::time::Duration::from_secs(2);
    let now = Instant::now();
    if let Ok(mut cache) = WIFI_SSID_CACHE.lock() {
        if let (Some(ssid), Some(last_query)) = &*cache
            && now.duration_since(*last_query) < CACHE_TTL
        {
            return Some(ssid.clone());
        }
        let fresh_ssid = query_active_wifi_ssid();
        *cache = (fresh_ssid.clone(), Some(now));
        fresh_ssid
    } else {
        query_active_wifi_ssid()
    }
}

/// Identifies adapter category and hardware medium from Windows NDIS driver properties and name strings.
pub fn classify_interface(
    if_type: i32,
    tunnel_type: i32,
    description: &str,
    alias: &str,
) -> (InterfaceCategory, InterfaceMedium) {
    const IF_TYPE_SOFTWARE_LOOPBACK: i32 = 24;
    const IF_TYPE_ETHERNET_CSMACD: i32 = 6;
    const IF_TYPE_IEEE80211: i32 = 71;
    const IF_TYPE_GIGABITETHERNET: i32 = 117;
    const IF_TYPE_FASTETHER: i32 = 62;
    const IF_TYPE_TUNNEL: i32 = 131;
    const IF_TYPE_WWANPP: i32 = 243;
    const IF_TYPE_WWANPP2: i32 = 244;

    if if_type == IF_TYPE_SOFTWARE_LOOPBACK {
        return (InterfaceCategory::Loopback, InterfaceMedium::Loopback);
    }

    if tunnel_type != 0 || if_type == IF_TYPE_TUNNEL {
        return (InterfaceCategory::Tunnel, InterfaceMedium::Virtual);
    }

    let combined = format!("{} {}", description, alias).to_lowercase();
    if combined.contains("virtual")
        || combined.contains("hyper-v")
        || combined.contains("vethernet")
        || combined.contains("vpn")
        || combined.contains("tap-")
        || combined.contains("wsl")
        || combined.contains("docker")
        || combined.contains("vmware")
        || combined.contains("virtualbox")
        || combined.contains("npcap")
        || combined.contains("wan miniport")
        || combined.contains("pacer")
        || combined.contains("loopback")
        || combined.contains("bluetooth")
        || combined.contains("filter")
        || combined.contains("wfp")
        || combined.contains("lightweight")
        || combined.contains("packet scheduler")
        || combined.contains("qos")
        || combined.contains("ndiscap")
        || combined.contains("native mac layer")
    {
        return (InterfaceCategory::Virtual, InterfaceMedium::Virtual);
    }

    if if_type == IF_TYPE_IEEE80211
        || combined.contains("wi-fi")
        || combined.contains("wifi")
        || combined.contains("wireless")
        || combined.contains("802.11")
        || combined.contains("wlan")
    {
        return (InterfaceCategory::Physical, InterfaceMedium::Wifi);
    }

    if if_type == IF_TYPE_WWANPP
        || if_type == IF_TYPE_WWANPP2
        || combined.contains("cellular")
        || combined.contains("mobile broadband")
        || combined.contains("wwan")
        || combined.contains(" lte")
        || combined.contains("lte ")
        || combined.contains(" 5g")
        || combined.contains("5g ")
    {
        return (InterfaceCategory::Physical, InterfaceMedium::Cellular);
    }

    if if_type == IF_TYPE_ETHERNET_CSMACD
        || if_type == IF_TYPE_GIGABITETHERNET
        || if_type == IF_TYPE_FASTETHER
        || combined.contains("ethernet")
        || combined.contains("gigabit")
        || combined.contains("realtek")
        || combined.contains("intel(r)")
    {
        return (InterfaceCategory::Physical, InterfaceMedium::Ethernet);
    }

    (InterfaceCategory::Physical, InterfaceMedium::Other)
}

/// Polls all network adapters and current octet transfer counters via Windows `GetIfTable2`.
pub fn query_interfaces() -> Result<Vec<InterfaceInfo>, String> {
    use windows::Win32::NetworkManagement::IpHelper;

    let mut table: *mut IpHelper::MIB_IF_TABLE2 = std::ptr::null_mut();
    let status = unsafe { IpHelper::GetIfTable2(&mut table) };

    if status.is_err() || table.is_null() {
        return Err(format!("GetIfTable2 failed: {:?}", status));
    }

    let mut interfaces = Vec::new();
    unsafe {
        let entries = (*table).NumEntries as usize;
        let rows = std::slice::from_raw_parts((*table).Table.as_ptr(), entries);

        for row in rows {
            let luid = row.InterfaceLuid.Value;
            let index = row.InterfaceIndex;
            let desc = wchar_to_string(&row.Description);
            let alias = wchar_to_string(&row.Alias);
            let if_type = row.Type as i32;
            let tunnel_type = row.TunnelType.0;
            let (category, medium) = classify_interface(if_type, tunnel_type, &desc, &alias);
            let oper_status = row.OperStatus.0;

            let clean_name = if !alias.is_empty()
                && !alias.to_lowercase().contains("filter")
                && !alias.to_lowercase().contains("wfp")
            {
                alias
            } else if medium == InterfaceMedium::Wifi {
                "Wi-Fi".to_string()
            } else if medium == InterfaceMedium::Ethernet {
                "Ethernet".to_string()
            } else if medium == InterfaceMedium::Cellular {
                "Cellular".to_string()
            } else if !desc.is_empty() {
                desc.clone()
            } else {
                "Network".to_string()
            };

            interfaces.push(InterfaceInfo {
                luid,
                index,
                name: clean_name,
                description: desc,
                category,
                medium,
                oper_status,
                in_octets: row.InOctets,
                out_octets: row.OutOctets,
                speed: row.ReceiveLinkSpeed,
            });
        }

        IpHelper::FreeMibTable(table as *const core::ffi::c_void);
    }

    Ok(interfaces)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn mock_iface(
        luid: u64,
        category: InterfaceCategory,
        medium: InterfaceMedium,
        oper_status: i32,
        in_bytes: u64,
        out_bytes: u64,
    ) -> InterfaceInfo {
        InterfaceInfo {
            luid,
            index: luid as u32,
            name: format!("eth{}", luid),
            description: "Mock Adapter".to_string(),
            category,
            medium,
            oper_status,
            in_octets: in_bytes,
            out_octets: out_bytes,
            speed: 1_000_000_000,
        }
    }

    #[test]
    fn test_classify_interface() {
        assert_eq!(
            classify_interface(24, 0, "Loopback", ""),
            (InterfaceCategory::Loopback, InterfaceMedium::Loopback)
        );
        assert_eq!(
            classify_interface(6, 1, "Tunnel adapter", ""),
            (InterfaceCategory::Tunnel, InterfaceMedium::Virtual)
        );
        assert_eq!(
            classify_interface(6, 0, "Hyper-V Virtual Ethernet Adapter", ""),
            (InterfaceCategory::Virtual, InterfaceMedium::Virtual)
        );
        assert_eq!(
            classify_interface(6, 0, "Realtek PCIe GbE Family Controller", "Ethernet"),
            (InterfaceCategory::Physical, InterfaceMedium::Ethernet)
        );
        assert_eq!(
            classify_interface(71, 0, "Intel(R) Wi-Fi 6 AX200", "Wi-Fi"),
            (InterfaceCategory::Physical, InterfaceMedium::Wifi)
        );
        assert_eq!(
            classify_interface(999, 0, "Something Else", ""),
            (InterfaceCategory::Physical, InterfaceMedium::Other)
        );
    }

    #[test]
    fn test_compute_delta_normal_and_reset() {
        assert_eq!(compute_delta(100, 250), 150);
        assert_eq!(compute_delta(250, 250), 0);
        assert_eq!(compute_delta(500, 50), 0); // counter reset
    }

    #[test]
    fn test_first_sample_establishes_baseline() {
        let mut backend = NetworkBackend::new();
        let t0 = Instant::now();
        let ifaces = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Wifi,
            1,
            1000,
            2000,
        )];

        let snap = backend.sample_from_interfaces(&ifaces, t0);
        assert_eq!(snap.rx_bps, 0.0);
        assert_eq!(snap.tx_bps, 0.0);
        assert_eq!(snap.session_rx, 0);
        assert_eq!(snap.session_tx, 0);
        assert!(snap.history.is_empty());
        assert_eq!(snap.active_interfaces, 1);
        assert_eq!(snap.primary_medium, InterfaceMedium::Wifi);

        // Second sample 250ms later (1 sampling interval)
        let t1 = t0 + Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
        let ifaces2 = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Wifi,
            1,
            1000 + 1250,
            2000 + 500,
        )];
        let snap2 = backend.sample_from_interfaces(&ifaces2, t1);
        assert_eq!(snap2.rx_bps, 5000.0);
        assert_eq!(snap2.tx_bps, 2000.0);
        assert_eq!(snap2.session_rx, 1250);
        assert_eq!(snap2.session_tx, 500);
        assert_eq!(snap2.history.len(), 1);
        assert_eq!(snap2.history[0].rx_bps, 5000);
        assert_eq!(snap2.history[0].tx_bps, 2000);
    }

    #[test]
    fn test_counter_reset_updates_baseline_immediately() {
        let mut backend = NetworkBackend::new();
        let t0 = Instant::now();
        let ifaces = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            10_000,
            10_000,
        )];
        backend.sample_from_interfaces(&ifaces, t0);

        // Sample 2: normal delta
        let t1 = t0 + Duration::from_secs(1);
        let ifaces2 = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            15_000,
            15_000,
        )];
        let snap2 = backend.sample_from_interfaces(&ifaces2, t1);
        assert_eq!(snap2.rx_bps, 5000.0);

        // Sample 3: adapter counter resets to 100
        let t2 = t1 + Duration::from_secs(1);
        let ifaces3 = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            100,
            100,
        )];
        let snap3 = backend.sample_from_interfaces(&ifaces3, t2);
        assert_eq!(snap3.rx_bps, 0.0); // clamped to 0

        // Sample 4: delta from new baseline (100 -> 300 = 200)
        let t3 = t2 + Duration::from_secs(1);
        let ifaces4 = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            300,
            300,
        )];
        let snap4 = backend.sample_from_interfaces(&ifaces4, t3);
        assert_eq!(snap4.rx_bps, 200.0);
    }

    #[test]
    fn test_disappeared_interface_pruned() {
        let mut backend = NetworkBackend::new();
        let t0 = Instant::now();
        let ifaces = vec![
            mock_iface(
                1,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                1000,
                1000,
            ),
            mock_iface(
                2,
                InterfaceCategory::Physical,
                InterfaceMedium::Wifi,
                1,
                1000,
                1000,
            ),
        ];
        backend.sample_from_interfaces(&ifaces, t0);
        assert_eq!(backend.prev_counters.len(), 2);

        // Sample 2: interface 2 disappears
        let t1 = t0 + Duration::from_secs(1);
        let ifaces2 = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            2000,
            2000,
        )];
        backend.sample_from_interfaces(&ifaces2, t1);
        assert_eq!(backend.prev_counters.len(), 1);
        assert!(backend.prev_counters.contains_key(&1));
        assert!(!backend.prev_counters.contains_key(&2));
    }

    #[test]
    fn test_mode_filtering() {
        let mut backend = NetworkBackend::with_mode(AggregateMode::PhysicalTransport);
        let t0 = Instant::now();
        let ifaces = vec![
            mock_iface(
                1,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                1000,
                1000,
            ),
            mock_iface(
                2,
                InterfaceCategory::Virtual,
                InterfaceMedium::Virtual,
                1,
                1000,
                1000,
            ),
            mock_iface(
                3,
                InterfaceCategory::Loopback,
                InterfaceMedium::Loopback,
                1,
                1000,
                1000,
            ),
        ];
        let snap = backend.sample_from_interfaces(&ifaces, t0);
        assert_eq!(snap.active_interfaces, 1);

        let t1 = t0 + Duration::from_secs(1);
        let ifaces2 = vec![
            mock_iface(
                1,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                2000,
                2000,
            ),
            mock_iface(
                2,
                InterfaceCategory::Virtual,
                InterfaceMedium::Virtual,
                1,
                9999,
                9999,
            ),
            mock_iface(
                3,
                InterfaceCategory::Loopback,
                InterfaceMedium::Loopback,
                1,
                9999,
                9999,
            ),
        ];
        let snap2 = backend.sample_from_interfaces(&ifaces2, t1);
        // Only physical (iface 1) delta should be counted
        assert_eq!(snap2.rx_bps, 1000.0);
        assert_eq!(snap2.tx_bps, 1000.0);
    }

    #[test]
    fn test_history_rolling_fifo() {
        let mut backend = NetworkBackend::new();
        let t0 = Instant::now();
        let ifaces = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            0,
            0,
        )];
        backend.sample_from_interfaces(&ifaces, t0);

        // Generate 260 samples at 250ms intervals (65s total, exceeds capacity of 240 / 60s)
        for i in 1..=260u64 {
            let t = t0 + Duration::from_millis(i * crate::SAMPLING_INTERVAL_MS);
            let ifaces = vec![mock_iface(
                1,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                i * 1000,
                i * 500,
            )];
            backend.sample_from_interfaces(&ifaces, t);
        }

        assert_eq!(backend.history.len(), HISTORY_CAPACITY);
        assert_eq!(HISTORY_CAPACITY, crate::history_samples_for_secs(60));
        assert_eq!(HISTORY_CAPACITY, 240);
        // The oldest samples should have been dropped
        assert!(backend.history.len() <= HISTORY_CAPACITY);
    }

    #[test]
    fn test_reset_session() {
        let mut backend = NetworkBackend::new();
        let t0 = Instant::now();
        let ifaces = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Wifi,
            1,
            1000,
            1000,
        )];
        backend.sample_from_interfaces(&ifaces, t0);

        let t1 = t0 + Duration::from_secs(1);
        let ifaces2 = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Wifi,
            1,
            5000,
            5000,
        )];
        backend.sample_from_interfaces(&ifaces2, t1);
        assert!(backend.session_rx > 0);
        assert!(!backend.history.is_empty());

        backend.reset_session();
        assert_eq!(backend.session_rx, 0);
        assert_eq!(backend.session_tx, 0);
        assert_eq!(backend.peak_rx, 0.0);
        assert_eq!(backend.peak_tx, 0.0);
        assert!(backend.history.is_empty());
        assert!(backend.prev_counters.is_empty());

        // First sample after reset re-establishes baseline without recording history or spiking rates
        let t2 = t1 + Duration::from_secs(1);
        let ifaces3 = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Wifi,
            1,
            6000,
            6000,
        )];
        let snap_reset = backend.sample_from_interfaces(&ifaces3, t2);
        assert_eq!(snap_reset.rx_bps, 0.0);
        assert_eq!(snap_reset.tx_bps, 0.0);
        assert_eq!(snap_reset.session_rx, 0);
        assert_eq!(snap_reset.session_tx, 0);
        assert!(snap_reset.history.is_empty());

        // Second sample after reset (250ms cadence) computes real delta from new baseline
        let t3 = t2 + Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
        let ifaces4 = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Wifi,
            1,
            6000 + 250,
            6000 + 125,
        )];
        let snap_after = backend.sample_from_interfaces(&ifaces4, t3);
        assert_eq!(snap_after.rx_bps, 1000.0);
        assert_eq!(snap_after.tx_bps, 500.0);
        assert_eq!(snap_after.session_rx, 250);
        assert_eq!(snap_after.session_tx, 125);
        assert_eq!(snap_after.history.len(), 1);
        assert_eq!(snap_after.history[0].rx_bps, 1000);
        assert_eq!(snap_after.history[0].tx_bps, 500);
    }

    #[test]
    fn test_zero_or_backwards_duration_handled_safely() {
        let mut backend = NetworkBackend::new();
        let t0 = Instant::now();
        let ifaces = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            1000,
            1000,
        )];
        backend.sample_from_interfaces(&ifaces, t0);

        // Immediate second sample at exact same instant t0 (duration = 0)
        let snap_zero = backend.sample_from_interfaces(&ifaces, t0);
        assert_eq!(snap_zero.rx_bps, 0.0);
        assert_eq!(snap_zero.tx_bps, 0.0);
        assert_eq!(snap_zero.history.len(), 0);

        // Third sample with backwards time (clock jitter where now < prev)
        let t_before = t0 - Duration::from_millis(100);
        let snap_jitter = backend.sample_from_interfaces(&ifaces, t_before);
        assert_eq!(snap_jitter.rx_bps, 0.0);
        assert_eq!(snap_jitter.tx_bps, 0.0);
        assert_eq!(snap_jitter.history.len(), 0);
    }

    #[test]
    fn test_inactive_interface_baseline_fix() {
        // Scenario: interface starts Down, comes Up — should NOT spike.
        let mut backend = NetworkBackend::new();
        let t0 = Instant::now();

        // Sample 1: iface is Down with counters at 10000
        let ifaces = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            2,
            10_000,
            10_000,
        )]; // status=2 (Down)
        backend.sample_from_interfaces(&ifaces, t0);

        // Sample 2: iface is still Down, counters advanced
        let t1 = t0 + Duration::from_secs(1);
        let ifaces2 = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            2,
            50_000,
            50_000,
        )];
        backend.sample_from_interfaces(&ifaces2, t1);

        // Sample 3: iface comes Up — delta should be from 50000, NOT from 0
        let t2 = t1 + Duration::from_secs(1);
        let ifaces3 = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            51_000,
            51_000,
        )]; // Up
        let snap3 = backend.sample_from_interfaces(&ifaces3, t2);

        // Only 1000 bytes delta (51000-50000), not 51000
        assert_eq!(snap3.rx_bps, 1000.0);
        assert_eq!(snap3.tx_bps, 1000.0);
    }

    #[test]
    fn test_wifi_ssid_query_does_not_panic() {
        // Safe execution check: whether running on Wi-Fi, Ethernet, or headless VM,
        // this must never panic and must return cleanly.
        let _ = query_cached_wifi_ssid();
        let _ = query_active_wifi_ssid();
    }

    #[test]
    fn test_history_time_cadence_gap_expansion() {
        let mut backend = NetworkBackend::new();
        let t0 = Instant::now();
        let ifaces = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            1000,
            1000,
        )];
        backend.sample_from_interfaces(&ifaces, t0);

        // Advance 3s (12 intervals of 250ms)
        let t1 = t0 + Duration::from_secs(3);
        let ifaces2 = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            1000 + 15000,
            1000 + 6000,
        )];
        let snap = backend.sample_from_interfaces(&ifaces2, t1);
        assert_eq!(snap.rx_bps, 5000.0);
        assert_eq!(snap.tx_bps, 2000.0);
        // History should contain 12 samples reflecting the 3s elapsed span (3.0s / 0.25s = 12)
        assert_eq!(snap.history.len(), 12);
        for s in &snap.history {
            assert_eq!(s.rx_bps, 5000);
            assert_eq!(s.tx_bps, 2000);
        }
    }

    #[test]
    fn test_history_clears_on_extended_gap() {
        let mut backend = NetworkBackend::new();
        let t0 = Instant::now();
        let ifaces = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            1000,
            1000,
        )];
        backend.sample_from_interfaces(&ifaces, t0);

        // Push some initial samples at 250ms intervals
        for i in 1..=5u64 {
            let t = t0 + Duration::from_millis(i * crate::SAMPLING_INTERVAL_MS);
            let ifaces = vec![mock_iface(
                1,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                1000 + i * 1000,
                1000 + i * 500,
            )];
            backend.sample_from_interfaces(&ifaces, t);
        }
        assert_eq!(backend.history.len(), 5);

        // Simulate 120-second suspend/sleep gap (> 5s threshold)
        let t_sleep =
            t0 + Duration::from_millis(5 * crate::SAMPLING_INTERVAL_MS) + Duration::from_secs(120);
        let ifaces_wake = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            100_000,
            50_000,
        )];
        let snap_wake = backend.sample_from_interfaces(&ifaces_wake, t_sleep);
        // Stale samples from before sleep are cleared, and wake establishes a clean new baseline
        assert_eq!(snap_wake.history.len(), 0);
        assert_eq!(backend.accumulator.time_acc_ns(), 0);
        assert_eq!(backend.accumulator.bytes_rx_acc(), 0);

        // Next 250ms sample after wake starts fresh without incorporating pre-suspend data
        let t_after_wake = t_sleep + Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
        let ifaces_after_wake = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            102_500,
            51_250,
        )];
        let snap_after_wake = backend.sample_from_interfaces(&ifaces_after_wake, t_after_wake);
        assert_eq!(snap_after_wake.history.len(), 1);
        assert_eq!(snap_after_wake.history[0].rx_bytes, 2500);
        assert_eq!(snap_after_wake.history[0].tx_bytes, 1250);
        assert_eq!(snap_after_wake.history[0].rx_bps, 10_000);
        assert_eq!(snap_after_wake.history[0].tx_bps, 5_000);
    }

    #[test]
    fn test_adversarial_fractional_allocation() {
        let mut acc = RateAccumulator::new();
        // 10 bytes transferred across 3 ns with 1-ns boundaries
        let emitted = acc.push_sample(3, 10, 20, 1);
        assert_eq!(emitted.len(), 3);
        // Under cumulative proportional allocation:
        // boundary 1 (1/3 of 10 = 3) -> 3
        // boundary 2 (2/3 of 10 = 6 - 3 = 3) -> 3
        // boundary 3 (3/3 of 10 = 10 - 6 = 4) -> 4
        assert_eq!(emitted[0].rx_bytes, 3);
        assert_eq!(emitted[1].rx_bytes, 3);
        assert_eq!(emitted[2].rx_bytes, 4);

        // TX (20 bytes across 3 ns):
        // boundary 1 (1/3 of 20 = 6) -> 6
        // boundary 2 (2/3 of 20 = 13 - 6 = 7) -> 7
        // boundary 3 (3/3 of 20 = 20 - 13 = 7) -> 7
        assert_eq!(emitted[0].tx_bytes, 6);
        assert_eq!(emitted[1].tx_bytes, 7);
        assert_eq!(emitted[2].tx_bytes, 7);

        assert_eq!(acc.time_acc_ns(), 0);
        assert_eq!(acc.bytes_rx_acc(), 0);
        assert_eq!(acc.bytes_tx_acc(), 0);
    }

    #[test]
    fn test_exact_byte_conservation_under_jitter() {
        let mut acc = RateAccumulator::new();
        let slot_ns = 500_000_000; // 500ms

        // 4 jittered intervals summing to exactly 2000ms (4 completed 500ms slots)
        let intervals = [
            (450_000_000, 10_000, 5_000),
            (550_000_000, 20_000, 10_000),
            (480_000_000, 15_000, 7_500),
            (520_000_000, 25_000, 12_500),
        ];

        let mut total_input_rx = 0u64;
        let mut total_input_tx = 0u64;
        let mut emitted_all = Vec::new();

        for (dt, rx, tx) in intervals {
            total_input_rx += rx;
            total_input_tx += tx;
            let mut buckets = acc.push_sample(dt, rx, tx, slot_ns);
            emitted_all.append(&mut buckets);
        }

        assert_eq!(emitted_all.len(), 4);
        assert_eq!(acc.time_acc_ns(), 0);
        assert_eq!(acc.bytes_rx_acc(), 0);
        assert_eq!(acc.bytes_tx_acc(), 0);

        let sum_emitted_rx: u64 = emitted_all.iter().map(|b| b.rx_bytes).sum();
        let sum_emitted_tx: u64 = emitted_all.iter().map(|b| b.tx_bytes).sum();

        // Exact integral conservation invariant
        assert_eq!(sum_emitted_rx, total_input_rx);
        assert_eq!(sum_emitted_tx, total_input_tx);

        for b in &emitted_all {
            assert_eq!(b.duration_ns, slot_ns);
            assert_eq!(b.rx_bps, b.rx_bytes * 2);
            assert_eq!(b.tx_bps, b.tx_bytes * 2);
        }
    }

    #[test]
    fn test_long_running_phase_stability() {
        let mut acc = RateAccumulator::new();
        let slot_ns = 500_000_000;

        let mut total_input_time = 0u64;
        let mut total_input_rx = 0u64;
        let mut total_emitted_rx = 0u64;
        let mut total_emitted_slots = 0u64;

        // Run 10,000 jittered intervals alternating around 500ms
        for i in 0..10_000u64 {
            let jitter_offset = (i % 7) * 5_000_000; // 0ms to 30ms
            let dt = if i % 2 == 0 {
                485_000_000 + jitter_offset
            } else {
                515_000_000 - jitter_offset
            };
            let rx = 1000 + (i % 100);

            total_input_time += dt;
            total_input_rx += rx;

            let buckets = acc.push_sample(dt, rx, 0, slot_ns);
            total_emitted_slots += buckets.len() as u64;
            for b in buckets {
                total_emitted_rx += b.rx_bytes;
            }
        }

        // Timing phase invariant: emitted time + residual time == total input time
        let emitted_time = total_emitted_slots * slot_ns;
        assert_eq!(emitted_time + acc.time_acc_ns(), total_input_time);

        // Byte flux conservation invariant: emitted bytes + residual bytes == total input bytes
        assert_eq!(total_emitted_rx + acc.bytes_rx_acc(), total_input_rx);
    }

    #[test]
    fn test_rolling_rate_window_boundary_interpolation() {
        let mut window = RollingRateWindow::new(1_000_000_000); // 1-second window
        let t0 = Instant::now();

        // Feed points:
        // 0.0s -> 0 bytes
        // 0.4s -> 400 bytes
        // 0.9s -> 900 bytes
        // 1.4s -> 1400 bytes
        window.record_sample(t0, 0, 0);
        window.record_sample(t0 + Duration::from_millis(400), 400, 0);
        window.record_sample(t0 + Duration::from_millis(900), 900, 0);
        window.record_sample(t0 + Duration::from_millis(1400), 1400, 0);

        // At t = 1.4s, the 1-second window looks back to t = 0.4s.
        // Counter at 0.4s is exactly 400.
        // Rate = (1400 - 400) / 1.0s = 1000 B/s.
        let (rate_rx, _) = window.current_rate(t0 + Duration::from_millis(1400));
        assert!((rate_rx - 1000.0).abs() < 1e-4);

        // Now advance to t = 1.5s (1500 bytes)
        // 1-second window looks back to t = 0.5s.
        // t = 0.5s falls between 0.4s (400 bytes) and 0.9s (900 bytes).
        // dt = 0.5s, fraction = (0.5 - 0.4) / 0.5 = 0.2.
        // Interpolated counter = 400 + 0.2 * 500 = 500 bytes.
        // Rate = (1500 - 500) / 1.0s = 1000 B/s!
        window.record_sample(t0 + Duration::from_millis(1500), 1500, 0);
        let (rate_rx_interp, _) = window.current_rate(t0 + Duration::from_millis(1500));
        assert!((rate_rx_interp - 1000.0).abs() < 1e-4);
    }

    #[test]
    fn test_session_state_serde_roundtrip() {
        let state = SessionState {
            session_rx: 1234567,
            session_tx: 987654,
            session_start_unix: 1700000000,
            all_time_peak_rx: 4500000.0,
            all_time_peak_tx: 1200000.0,
        };
        let serialized = serde_json::to_string(&state).expect("serialize");
        let deserialized: SessionState = serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(state, deserialized);
    }
    #[test]
    fn test_incremental_chart_peak_tracking() {
        let mut backend = NetworkBackend::new();
        let t0 = Instant::now();
        let ifaces = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            0,
            0,
        )];
        backend.sample_from_interfaces(&ifaces, t0);

        let mut t = t0;
        let mut total_rx = 0u64;
        let mut total_tx = 0u64;

        // Sample 1: rate 20,000 rx, 10,000 tx (5,000 rx bytes, 2,500 tx bytes in 250ms)
        t += Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
        total_rx += 5000;
        total_tx += 2500;
        let snap = backend.sample_from_interfaces(
            &[mock_iface(
                1,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                total_rx,
                total_tx,
            )],
            t,
        );
        assert_eq!(snap.peak_rx_bps, 20_000.0);
        assert_eq!(snap.peak_tx_bps, 10_000.0);

        // Sample 2: burst! rate 100,000 rx, 50,000 tx (25,000 rx bytes, 12,500 tx bytes)
        t += Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
        total_rx += 25000;
        total_tx += 12500;
        let snap = backend.sample_from_interfaces(
            &[mock_iface(
                1,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                total_rx,
                total_tx,
            )],
            t,
        );
        assert_eq!(snap.peak_rx_bps, 100_000.0);
        assert_eq!(snap.peak_tx_bps, 50_000.0);

        // Sample 3: drop down to lower rate 4,000 rx, 2,000 tx (1,000 rx bytes, 500 tx bytes)
        t += Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
        total_rx += 1000;
        total_tx += 500;
        let snap = backend.sample_from_interfaces(
            &[mock_iface(
                1,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                total_rx,
                total_tx,
            )],
            t,
        );
        assert_eq!(snap.peak_rx_bps, 100_000.0);
        assert_eq!(snap.peak_tx_bps, 50_000.0);

        // Push 237 more samples at 4,000 rx, 2,000 tx (total 240 samples in history)
        for _ in 0..237 {
            t += Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
            total_rx += 1000;
            total_tx += 500;
            backend.sample_from_interfaces(
                &[mock_iface(
                    1,
                    InterfaceCategory::Physical,
                    InterfaceMedium::Ethernet,
                    1,
                    total_rx,
                    total_tx,
                )],
                t,
            );
        }
        assert_eq!(backend.history.len(), HISTORY_CAPACITY);
        assert_eq!(backend.chart_peak_rx, 100_000);

        // Push 1 more sample: evicts sample 1 (20,000 rx). Peak 100,000 is still in history!
        t += Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
        total_rx += 1000;
        total_tx += 500;
        let snap = backend.sample_from_interfaces(
            &[mock_iface(
                1,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                total_rx,
                total_tx,
            )],
            t,
        );
        assert_eq!(snap.peak_rx_bps, 100_000.0);

        // Push 1 more sample: evicts sample 2 (the 100,000 peak holder!). Lazy rescan occurs.
        // History now contains only 4,000 rx samples. Peak drops to 4,000!
        t += Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
        total_rx += 1000;
        total_tx += 500;
        let snap = backend.sample_from_interfaces(
            &[mock_iface(
                1,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                total_rx,
                total_tx,
            )],
            t,
        );
        assert_eq!(snap.peak_rx_bps, 4_000.0);
        assert_eq!(snap.peak_tx_bps, 2_000.0);

        // Reset session drops peaks to 0
        backend.reset_session();
        assert_eq!(backend.chart_peak_rx, 0);
        assert_eq!(backend.chart_peak_tx, 0);
    }
}
