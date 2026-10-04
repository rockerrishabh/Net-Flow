use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};

use super::adapter::{query_adapter_diagnostics, query_interfaces};
use super::diagnostics::build_diagnostics_snapshot;
use super::latency::{PacketLossTracker, sample_latency_snapshot_dual_stack};
use super::phy::{query_cached_wifi_phy_for_luid, query_physical_link_info_for_luid};
use super::rate::{RateAccumulator, RollingRateWindow, counter_delta};
use super::session::{SessionState, load_persisted_session_state, save_persisted_session_state};
use super::types::{
    AggregateMode, HISTORY_CAPACITY, HistorySample, InterfaceCategory, InterfaceInfo,
    InterfaceLuid, InterfaceMedium, InterfaceSample, LatencyHealth, LatencySnapshot, LatencyState,
    LatencyTargetMode, NetworkSnapshot, PhysicalLinkInfo, ProbeResult,
};

/// Tracking byte counts for a specific network interface.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct InterfaceCounterState {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

#[derive(Default)]
struct InterfaceChartHistory {
    accumulator: RateAccumulator,
    samples: VecDeque<HistorySample>,
}

/// Core telemetry backend that queries Windows network adapters, computes bandwidth, and tracks apps.
pub struct NetworkBackend {
    pub mode: AggregateMode,
    /// Baseline byte counts for all known interfaces to avoid spikes when an adapter comes online.
    pub(crate) prev_counters: HashMap<InterfaceLuid, InterfaceCounterState>,
    pub(crate) prev_time: Option<Instant>,
    pub peak_rx: f64,
    pub peak_tx: f64,
    pub session_rx: u64,
    pub session_tx: u64,
    pub session_start_unix: u64,
    /// Rolling FIFO buffer of fixed-duration bandwidth samples.
    pub(crate) history: VecDeque<HistorySample>,
    /// Fixed-duration histories used when the widget is filtered to one adapter.
    per_interface_chart_history: HashMap<InterfaceLuid, InterfaceChartHistory>,
    /// Tracked chart peak download rate across current history buffer.
    pub(crate) chart_peak_rx: u64,
    /// Tracked chart peak upload rate across current history buffer.
    pub(crate) chart_peak_tx: u64,
    /// Tracks processes that own active network sockets.
    pub process_tracker: crate::process::ProcessTracker,
    /// Last time the process table was refreshed (throttled to 1s).
    last_process_sample: Option<Instant>,
    /// Cached socket owners with their latest socket-count-based traffic estimates.
    pub(crate) cached_apps: (Vec<crate::process::ActiveAppInfo>, usize),
    /// Sub-sample bucket accumulator for fixed 250ms chart slices.
    pub accumulator: RateAccumulator,
    /// Rolling 1-second window for smooth UI headline rates.
    pub rolling_window: RollingRateWindow,
    /// Most recently probed network latency.
    pub latency: LatencySnapshot,
    /// Monotonically increasing session generation counter.
    pub generation: u64,
    /// Authoritative packet loss tracker maintaining rolling probe history.
    pub packet_loss_tracker: PacketLossTracker,
    /// Active physical link layer information (Wi-Fi PHY or Ethernet).
    pub physical_link: Option<PhysicalLinkInfo>,
    /// Bounded day-keyed daily usage store tracking 90-day bandwidth history.
    pub daily_usage: crate::daily_usage::DailyUsageStore,
    /// Whether daily usage has accumulated changes that need atomic flush to disk.
    pub daily_dirty: std::sync::atomic::AtomicBool,
    /// Active data budget configuration.
    pub budget_config: crate::budget::DataBudgetConfig,
    /// Last time daily usage was flushed to disk.
    last_daily_flush: Instant,
    /// Current calendar day string to detect local midnight transitions.
    last_local_day: String,
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
            per_interface_chart_history: HashMap::new(),
            chart_peak_rx: 0,
            chart_peak_tx: 0,
            process_tracker: crate::process::ProcessTracker::new(),
            last_process_sample: None,
            cached_apps: (Vec::new(), 0),
            accumulator: RateAccumulator::new(),
            rolling_window: RollingRateWindow::new(1_000_000_000),
            latency: LatencySnapshot::default(),
            generation: 1,
            packet_loss_tracker: PacketLossTracker::default(),
            physical_link: None,
            daily_usage: crate::daily_usage::DailyUsageStore::default(),
            daily_dirty: std::sync::atomic::AtomicBool::new(false),
            budget_config: crate::budget::DataBudgetConfig::default(),
            last_daily_flush: Instant::now(),
            last_local_day: String::new(),
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
            per_interface_chart_history: HashMap::new(),
            chart_peak_rx: 0,
            chart_peak_tx: 0,
            process_tracker: crate::process::ProcessTracker::new(),
            last_process_sample: None,
            cached_apps: (Vec::new(), 0),
            accumulator: RateAccumulator::new(),
            rolling_window: RollingRateWindow::new(1_000_000_000),
            latency: persisted.latency,
            generation: persisted.generation.max(1),
            packet_loss_tracker: PacketLossTracker::default(),
            physical_link: persisted.physical_link,
            daily_usage: crate::daily_usage::load_daily_usage(),
            daily_dirty: std::sync::atomic::AtomicBool::new(false),
            budget_config: crate::card::load_user_config().budget,
            last_daily_flush: Instant::now(),
            last_local_day: String::new(),
        }
    }

    pub fn set_latency(&mut self, mut latency: LatencySnapshot) {
        if let (Some(curr), Some(prev)) = (latency.latency_ms, self.latency.latency_ms) {
            latency.jitter_ms = Some(curr.abs_diff(prev));
        }
        self.latency = latency;
    }

    /// Records an ICMP probe outcome into the authoritative packet loss tracker
    /// and derives updated packet loss percentage and health on the latency snapshot.
    pub fn update_latency_probe(&mut self, mut snap: LatencySnapshot, result: ProbeResult) {
        if let (Some(curr), Some(prev)) = (snap.latency_ms, self.latency.latency_ms) {
            snap.jitter_ms = Some(curr.abs_diff(prev));
        }
        self.packet_loss_tracker.record(result);
        let (loss_pct, lost, total, health) = self.packet_loss_tracker.evaluate();
        snap.packet_loss_pct = loss_pct;
        snap.pings_lost = lost;
        snap.pings_total = total;
        snap.health = health;
        if health == LatencyHealth::Degraded {
            snap.state = LatencyState::Degraded;
        }
        self.latency = snap;
    }

    pub fn set_physical_link(&mut self, link: Option<PhysicalLinkInfo>) {
        self.physical_link = link;
    }

    /// Updates the active data budget configuration.
    pub fn update_budget_config(&mut self, config: crate::budget::DataBudgetConfig) {
        if self.budget_config != config {
            self.budget_config = config;
            self.persist_daily_usage();
        }
    }

    /// Flushes accumulated daily usage data atomically to disk if dirty.
    pub fn persist_daily_usage(&mut self) {
        if self.daily_dirty.load(std::sync::atomic::Ordering::SeqCst) {
            crate::daily_usage::save_daily_usage(&self.daily_usage);
            self.daily_dirty
                .store(false, std::sync::atomic::Ordering::SeqCst);
        }
    }

    /// Flush session state and daily usage to disk.
    pub fn persist_session(&self) {
        let now_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        save_persisted_session_state(&SessionState {
            generation: self.generation,
            session_rx: self.session_rx,
            session_tx: self.session_tx,
            session_start_unix: self.session_start_unix,
            all_time_peak_rx: self.peak_rx,
            all_time_peak_tx: self.peak_tx,
            updated_at_unix: now_unix,
            latency: self.latency,
            physical_link: self.physical_link.clone(),
        });
        if self.daily_dirty.load(std::sync::atomic::Ordering::SeqCst) {
            crate::daily_usage::save_daily_usage(&self.daily_usage);
            self.daily_dirty
                .store(false, std::sync::atomic::Ordering::SeqCst);
        }
    }

    /// Reset session: increment generation, re-anchor all baselines, clear totals and history.
    pub fn reset_session(&mut self) {
        self.generation = self.generation.saturating_add(1);
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
        self.per_interface_chart_history.clear();
        self.chart_peak_rx = 0;
        self.chart_peak_tx = 0;
        self.accumulator.reset();
        self.rolling_window.reset();
        self.last_process_sample = None;
        self.cached_apps = (Vec::new(), 0);
        save_persisted_session_state(&SessionState {
            generation: self.generation,
            session_rx: 0,
            session_tx: 0,
            session_start_unix: now_unix,
            all_time_peak_rx: 0.0,
            all_time_peak_tx: 0.0,
            updated_at_unix: now_unix,
            latency: self.latency,
            physical_link: self.physical_link.clone(),
        });
    }

    /// Synchronizes in-memory session totals and latency against authoritative persisted state.
    ///
    /// If another process (like the persistent Tray Host) incremented the session generation,
    /// local accumulators and history are reset immediately. Returns `true` if a generation
    /// change was detected.
    pub fn sync_from_persisted_session(&mut self) -> bool {
        let persisted = load_persisted_session_state();
        let generation_changed = persisted.generation != self.generation;
        if generation_changed {
            self.generation = persisted.generation;
            self.session_rx = persisted.session_rx;
            self.session_tx = persisted.session_tx;
            self.session_start_unix = persisted.session_start_unix;
            self.history.clear();
            self.per_interface_chart_history.clear();
            self.chart_peak_rx = 0;
            self.chart_peak_tx = 0;
            self.accumulator.reset();
            self.rolling_window.reset();
            self.last_process_sample = None;
            self.cached_apps = (Vec::new(), 0);
        } else {
            if persisted.session_rx > self.session_rx {
                self.session_rx = persisted.session_rx;
            }
            if persisted.session_tx > self.session_tx {
                self.session_tx = persisted.session_tx;
            }
            if persisted.session_start_unix != 0 {
                self.session_start_unix = persisted.session_start_unix;
            }
        }
        let now_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        if persisted.latency.is_fresh(now_unix, 10) {
            self.latency = persisted.latency;
        }
        self.physical_link = persisted.physical_link;
        generation_changed
    }

    /// Primary sampling method: queries IP Helper APIs, computes rates, updates state.
    pub fn sample(&mut self) -> Result<NetworkSnapshot, String> {
        let interfaces = query_interfaces()?;
        let now = Instant::now();
        let mut snapshot = self.sample_from_interfaces(&interfaces, now);
        self.physical_link = query_physical_link_info_for_luid(
            snapshot.primary_medium,
            snapshot.primary_interface_luid(),
        );
        snapshot.physical_link = self.physical_link.clone();

        let need_process_sample = match self.last_process_sample {
            Some(last) => now.duration_since(last) >= Duration::from_millis(1000),
            None => true,
        };

        if need_process_sample {
            self.cached_apps = self.process_tracker.sample(now);
            self.last_process_sample = Some(now);
        }

        // Always clone the pre-reconciliation raw output and reconcile fresh
        // against this tick's network totals, avoiding directional-mismatch distortion!
        let (mut active_apps, active_conns) = self.cached_apps.clone();
        crate::process::estimate_app_bandwidth_by_sockets(
            &mut active_apps,
            snapshot.rx_bps,
            snapshot.tx_bps,
        );
        self.cached_apps = (active_apps.clone(), active_conns);
        snapshot.active_apps = active_apps;
        snapshot.active_connections_count = active_conns;

        // Batched periodic flush of daily usage store (every 30 seconds if dirty)
        if self.daily_dirty.load(std::sync::atomic::Ordering::SeqCst)
            && self.last_daily_flush.elapsed() >= Duration::from_secs(30)
        {
            crate::daily_usage::save_daily_usage(&self.daily_usage);
            self.daily_dirty
                .store(false, std::sync::atomic::Ordering::SeqCst);
            self.last_daily_flush = now;
        }

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
        let mut per_interface_deltas = Vec::new();

        for iface in interfaces {
            seen_luids.insert(iface.luid);

            // **FIX**: Always update baseline counters for EVERY interface,
            // regardless of oper_status or mode. This prevents false spikes
            // when an interface cycles Down → Up.
            let (delta_in, delta_out) = match self.prev_counters.get(&iface.luid) {
                Some(prev) => {
                    let d_in = counter_delta(prev.rx_bytes, iface.in_octets).unwrap_or(0);
                    let d_out = counter_delta(prev.tx_bytes, iface.out_octets).unwrap_or(0);
                    (d_in, d_out)
                }
                None => {
                    // New interface: baseline established, report 0 delta
                    (0, 0)
                }
            };

            // Unconditionally update baseline to current values
            self.prev_counters.insert(
                iface.luid,
                InterfaceCounterState {
                    rx_bytes: iface.in_octets,
                    tx_bytes: iface.out_octets,
                },
            );

            // Only active, mode-matching interfaces contribute to aggregation
            if iface.oper_status != 1 {
                continue;
            }
            if !self.mode.matches(iface.category) {
                continue;
            }

            total_delta_in += delta_in;
            total_delta_out += delta_out;
            per_interface_deltas.push((iface.luid, delta_in, delta_out));

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
        let active_luids: HashSet<_> = per_interface.iter().map(|i| i.luid).collect();
        self.per_interface_chart_history
            .retain(|luid, _| active_luids.contains(luid));

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

        let (primary_medium, primary_luid, mut primary_name) = match primary {
            Some(p) => (p.medium, Some(p.luid), p.name.clone()),
            None => (InterfaceMedium::Other, None, "Network".to_string()),
        };

        if primary_medium == InterfaceMedium::Wifi
            && let Some(ssid) = query_cached_wifi_phy_for_luid(primary_luid).map(|w| w.ssid)
        {
            primary_name = ssid;
        }

        // Prune interfaces that disappeared from the system
        self.prev_counters
            .retain(|luid, _| seen_luids.contains(luid));

        // Accumulate session totals
        self.session_rx += total_delta_in;
        self.session_tx += total_delta_out;

        // Daily usage accumulation and date rollover check
        let today_ymd = crate::budget::current_local_ymd();
        let today_str = crate::budget::format_ymd(today_ymd.0, today_ymd.1, today_ymd.2);
        if !self.last_local_day.is_empty() && self.last_local_day != today_str {
            if self.daily_dirty.load(std::sync::atomic::Ordering::SeqCst) {
                crate::daily_usage::save_daily_usage(&self.daily_usage);
                self.daily_dirty
                    .store(false, std::sync::atomic::Ordering::SeqCst);
            }
            self.last_local_day = today_str.clone();
        } else if self.last_local_day.is_empty() {
            self.last_local_day = today_str.clone();
        }

        if total_delta_in > 0 || total_delta_out > 0 {
            self.daily_usage
                .record_usage_delta(&today_str, total_delta_in, total_delta_out);
            self.daily_dirty
                .store(true, std::sync::atomic::Ordering::SeqCst);
        }

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
            self.per_interface_chart_history.clear();
            self.chart_peak_rx = 0;
            self.chart_peak_tx = 0;
            self.accumulator.reset();
            self.rolling_window.reset();
            self.rolling_window
                .record_sample(now, self.session_rx, self.session_tx);
        } else if elapsed_ns > 0 {
            let mut emitted =
                self.accumulator
                    .push_sample(elapsed_ns, total_delta_in, total_delta_out, slot_ns);
            for bucket in &mut emitted {
                bucket.latency_ms = self.latency.latency_ms;
                bucket.jitter_ms = self.latency.jitter_ms;

                if (bucket.rx_bps as f64) > self.peak_rx {
                    self.peak_rx = bucket.rx_bps as f64;
                }
                if (bucket.tx_bps as f64) > self.peak_tx {
                    self.peak_tx = bucket.tx_bps as f64;
                }
                if self.history.len() >= HISTORY_CAPACITY
                    && let Some(evicted) = self.history.pop_front()
                {
                    if evicted.rx_bps == self.chart_peak_rx {
                        self.chart_peak_rx =
                            self.history.iter().map(|s| s.rx_bps).max().unwrap_or(0);
                    }
                    if evicted.tx_bps == self.chart_peak_tx {
                        self.chart_peak_tx =
                            self.history.iter().map(|s| s.tx_bps).max().unwrap_or(0);
                    }
                }
                self.history.push_back(*bucket);
                self.chart_peak_rx = self.chart_peak_rx.max(bucket.rx_bps);
                self.chart_peak_tx = self.chart_peak_tx.max(bucket.tx_bps);
            }
            for (luid, delta_rx, delta_tx) in per_interface_deltas {
                let interface_history = self.per_interface_chart_history.entry(luid).or_default();
                for bucket in interface_history
                    .accumulator
                    .push_sample(elapsed_ns, delta_rx, delta_tx, slot_ns)
                {
                    if interface_history.samples.len() >= HISTORY_CAPACITY {
                        interface_history.samples.pop_front();
                    }
                    interface_history.samples.push_back(bucket);
                }
            }
            self.rolling_window
                .record_sample(now, self.session_rx, self.session_tx);
        } else {
            // First sample establishes initial baseline point
            self.rolling_window
                .record_sample(now, self.session_rx, self.session_tx);
        }

        let (rx_bps, tx_bps) = self.rolling_window.current_rate(now);
        let rx_bps_250ms = self.history.back().map(|s| s.rx_bps).unwrap_or(0);
        let tx_bps_250ms = self.history.back().map(|s| s.tx_bps).unwrap_or(0);

        let now_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let session_duration_secs = now_unix.saturating_sub(self.session_start_unix);

        let milestones_before = self.daily_usage.notified_milestones.clone();
        let budget_snap = crate::budget::calculate_budget_snapshot(
            &self.budget_config,
            &mut self.daily_usage,
            today_ymd,
        );
        if self.daily_usage.notified_milestones != milestones_before {
            self.daily_dirty
                .store(true, std::sync::atomic::Ordering::SeqCst);
        }

        NetworkSnapshot {
            generation: self.generation,
            rx_bps,
            tx_bps,
            rx_bps_250ms,
            tx_bps_250ms,
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
            per_interface_history: self
                .per_interface_chart_history
                .iter()
                .map(|(luid, history)| (*luid, history.samples.iter().copied().collect()))
                .collect(),
            history: self.history.iter().copied().collect(),
            primary_medium,
            primary_name,
            active_apps: Vec::new(),
            active_connections_count: 0,
            latency: self.latency,
            physical_link: self.physical_link.clone(),
            budget: Some(budget_snap),
        }
    }

    pub fn collect_diagnostics_snapshot(&mut self) -> crate::export::DiagnosticsSnapshot {
        let _ = self.sample();

        if self.latency.latency_ms.is_none() && self.latency.state == LatencyState::Unavailable {
            let (snap, res) = sample_latency_snapshot_dual_stack(
                LatencyTargetMode::Internet,
                self.generation,
                500,
                None,
            );
            self.update_latency_probe(snap, res);
        }

        let (active, others) = query_adapter_diagnostics();
        build_diagnostics_snapshot(self, active, others)
    }
}
