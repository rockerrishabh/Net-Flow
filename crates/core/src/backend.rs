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
    /// Round-trip latency in milliseconds.
    #[serde(default)]
    pub latency_ms: Option<u32>,
    /// Jitter (latency variance from previous probe) in milliseconds.
    #[serde(default)]
    pub jitter_ms: Option<u32>,
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
            latency_ms: None,
            jitter_ms: None,
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
            latency_ms: None,
            jitter_ms: None,
        }
    }
}

/// Persistent telemetry metrics saved across widget restarts and session resets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionState {
    #[serde(default = "default_session_generation")]
    pub generation: u64,
    #[serde(alias = "download_bytes")]
    pub session_rx: u64,
    #[serde(alias = "upload_bytes")]
    pub session_tx: u64,
    #[serde(alias = "started_at")]
    pub session_start_unix: u64,
    #[serde(default)]
    pub all_time_peak_rx: f64,
    #[serde(default)]
    pub all_time_peak_tx: f64,
    #[serde(default)]
    pub updated_at_unix: u64,
    #[serde(default)]
    pub latency: LatencySnapshot,
    #[serde(default)]
    pub physical_link: Option<PhysicalLinkInfo>,
}

const fn default_session_generation() -> u64 {
    1
}

impl Default for SessionState {
    fn default() -> Self {
        let now_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            generation: 1,
            session_rx: 0,
            session_tx: 0,
            session_start_unix: now_unix,
            all_time_peak_rx: 0.0,
            all_time_peak_tx: 0.0,
            updated_at_unix: now_unix,
            latency: LatencySnapshot::default(),
            physical_link: None,
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

/// Atomically writes content to the target file by first writing to a process-unique temporary
/// file in the same directory, syncing to disk, and renaming over the target path with retry backoff.
pub fn write_atomic(path: &std::path::Path, content: &[u8]) -> std::io::Result<()> {
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    std::fs::create_dir_all(parent)?;
    let file_name = path
        .file_name()
        .map(|s| s.to_string_lossy())
        .unwrap_or_else(|| "atomic".into());
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let tmp_path = parent.join(format!(
        "{}.tmp.{}.{}",
        file_name,
        std::process::id(),
        nanos
    ));

    {
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&tmp_path)?;
        file.write_all(content)?;
        file.flush()?;
        file.sync_all()?;
    }

    let mut last_err = None;
    for attempt in 0..5 {
        match std::fs::rename(&tmp_path, path) {
            Ok(()) => return Ok(()),
            Err(err) => {
                last_err = Some(err);
                if attempt < 4 {
                    std::thread::sleep(std::time::Duration::from_millis(5 * (attempt + 1) as u64));
                }
            }
        }
    }

    let _ = std::fs::remove_file(&tmp_path);
    Err(last_err
        .unwrap_or_else(|| std::io::Error::other("Failed to rename temporary file atomically")))
}

pub fn save_persisted_session_state(state: &SessionState) {
    let path = get_session_state_path();
    let mut updated = state.clone();
    if updated.updated_at_unix == 0 {
        updated.updated_at_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
    }
    if let Ok(json) = serde_json::to_string_pretty(&updated) {
        let _ = write_atomic(&path, json.as_bytes());
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

/// User-configurable mode selecting the target endpoint for network latency.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LatencyTargetMode {
    /// Probe public internet (1.1.1.1) when route is available; fallback to Gateway.
    #[default]
    Auto,
    /// Probe public internet resolver (1.1.1.1).
    Internet,
    /// Probe local network default gateway / router on LAN.
    Gateway,
}

impl LatencyTargetMode {
    pub fn to_str_value(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Internet => "internet",
            Self::Gateway => "gateway",
        }
    }

    pub fn from_str_value(s: &str) -> Self {
        match s {
            "internet" => Self::Internet,
            "gateway" => Self::Gateway,
            _ => Self::Auto,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::Internet => "Internet",
            Self::Gateway => "Gateway",
        }
    }
}

/// Target endpoint being probed for network round-trip latency.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LatencyTarget {
    /// Local default gateway / router on the LAN.
    #[default]
    Gateway,
    /// Public internet resolver probe (e.g. 1.1.1.1).
    Internet,
}

impl LatencyTarget {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Gateway => "Gateway",
            Self::Internet => "Internet",
        }
    }
}

/// Operational IP protocol utilized for network latency probing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum IpProtocol {
    #[default]
    Ipv4,
    Ipv6,
}

impl IpProtocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ipv4 => "IPv4",
            Self::Ipv6 => "IPv6",
        }
    }
}

/// Point-in-time result of an individual network ICMP latency probe attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProbeResult {
    /// Echo reply successfully received with round-trip latency.
    Success { latency: Duration },
    /// Echo probe timed out waiting for response.
    Timeout,
    /// Probe could not be executed (e.g. no route or network down).
    Unavailable,
}

/// Overall latency connectivity health state derived from probe history and packet loss.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LatencyHealth {
    /// Latency not yet probed, disabled, or network unreachable.
    #[default]
    Unavailable,
    /// Probe succeeded with 0% rolling packet loss.
    Healthy,
    /// Probe succeeded but experiencing packet loss (> 0% and < 100%).
    Degraded,
    /// All probes in rolling window timed out (100% loss).
    Timeout,
}

impl LatencyHealth {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unavailable => "Unavailable",
            Self::Healthy => "Healthy",
            Self::Degraded => "Degraded",
            Self::Timeout => "Timeout",
        }
    }
}

/// Circular sliding-window buffer tracking recent probe results to compute rolling packet loss.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PacketLossTracker {
    pub samples: [Option<ProbeResult>; 20],
    pub head: usize,
    pub count: usize,
}

impl PacketLossTracker {
    pub fn record(&mut self, result: ProbeResult) {
        self.samples[self.head] = Some(result);
        self.head = (self.head + 1) % 20;
        self.count = (self.count + 1).min(20);
    }

    /// Calculates rolling packet loss percentage and packet counts.
    /// Unavailable attempts are excluded from packet loss accounting.
    pub fn evaluate(&self) -> (Option<u8>, u32, u32, LatencyHealth) {
        let mut total_attempts = 0u32;
        let mut lost_packets = 0u32;

        for i in 0..self.count {
            if let Some(res) = self.samples[i] {
                match res {
                    ProbeResult::Success { .. } => {
                        total_attempts += 1;
                    }
                    ProbeResult::Timeout => {
                        total_attempts += 1;
                        lost_packets += 1;
                    }
                    ProbeResult::Unavailable => {}
                }
            }
        }

        if total_attempts == 0 {
            return (None, 0, 0, LatencyHealth::Unavailable);
        }

        let pct = ((lost_packets as f64 / total_attempts as f64) * 100.0).round() as u8;
        let health = if lost_packets == 0 {
            LatencyHealth::Healthy
        } else if lost_packets == total_attempts {
            LatencyHealth::Timeout
        } else {
            LatencyHealth::Degraded
        };

        (Some(pct), lost_packets, total_attempts, health)
    }

    /// Convenience getter for rolling packet loss percentage.
    pub fn loss_pct(&self) -> Option<u8> {
        self.evaluate().0
    }

    /// Convenience getter for (lost_packets, total_attempts).
    pub fn counts(&self) -> (u32, u32) {
        let (_, lost, total, _) = self.evaluate();
        (lost, total)
    }

    /// Convenience getter for latency health assessment.
    pub fn health(&self) -> LatencyHealth {
        self.evaluate().3
    }
}

/// Wi-Fi generation / standard mapped from 802.11 PHY types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WifiGeneration {
    #[default]
    Unknown,
    Legacy,
    Wifi4,
    Wifi5,
    Wifi6,
    Wifi7,
}

impl WifiGeneration {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Legacy => "Wi-Fi (Legacy)",
            Self::Wifi4 => "Wi-Fi 4",
            Self::Wifi5 => "Wi-Fi 5",
            Self::Wifi6 => "Wi-Fi 6",
            Self::Wifi7 => "Wi-Fi 7",
            Self::Unknown => "Wi-Fi",
        }
    }
}

/// Pure deterministic mapping from Windows DOT11_PHY_TYPE to WifiGeneration.
pub fn wifi_generation(phy: u32) -> WifiGeneration {
    match phy {
        7 => WifiGeneration::Wifi4,          // dot11_phy_type_ht (802.11n)
        8 => WifiGeneration::Wifi5,          // dot11_phy_type_vht (802.11ac)
        10 => WifiGeneration::Wifi6,         // dot11_phy_type_he (802.11ax)
        11 => WifiGeneration::Wifi7,         // dot11_phy_type_eht (802.11be)
        1..=6 | 9 => WifiGeneration::Legacy, // fhss, dsss, ir, ofdm, hrdsss, erp, dmg
        _ => WifiGeneration::Unknown,
    }
}

/// Operating frequency band of a Wi-Fi connection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WifiBand {
    #[default]
    Unknown,
    Band24Ghz,
    Band5Ghz,
    Band6Ghz,
}

impl WifiBand {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Band24Ghz => "2.4 GHz",
            Self::Band5Ghz => "5 GHz",
            Self::Band6Ghz => "6 GHz",
            Self::Unknown => "",
        }
    }
}

/// Pure deterministic deduction of Wi-Fi band from channel frequency (MHz) or channel number.
pub fn wifi_band(freq_mhz: u32, channel: Option<u32>) -> WifiBand {
    if (5925..=7125).contains(&freq_mhz) {
        WifiBand::Band6Ghz
    } else if (4900..=5895).contains(&freq_mhz) {
        WifiBand::Band5Ghz
    } else if (2400..=2500).contains(&freq_mhz) {
        WifiBand::Band24Ghz
    } else if let Some(ch) = channel {
        if (1..=14).contains(&ch) {
            WifiBand::Band24Ghz
        } else if (32..=177).contains(&ch) {
            WifiBand::Band5Ghz
        } else if ch > 177 {
            WifiBand::Band6Ghz
        } else {
            WifiBand::Unknown
        }
    } else {
        WifiBand::Unknown
    }
}

/// Pure deterministic conversion from 0..=100 link quality percentage to estimated RSSI in dBm.
/// Microsoft documentation: 0 -> -100 dBm, 100 -> -50 dBm with linear interpolation.
pub fn signal_quality_to_rssi_dbm(quality: u8) -> i16 {
    let clamped = quality.min(100);
    (clamped as i16 / 2) - 100
}

/// Physical layer link attributes for an active Wi-Fi connection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WifiPhyMetrics {
    pub ssid: String,
    pub generation: WifiGeneration,
    pub band: WifiBand,
    pub channel: Option<u32>,
    pub signal_quality_pct: u8,
    pub rssi_dbm: Option<i16>,
    pub tx_rate_mbps: Option<u32>,
    pub rx_rate_mbps: Option<u32>,
    pub is_mlo: bool,
    pub link_count: u8,
}

/// Physical link metrics for a wired Ethernet connection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthernetLinkMetrics {
    pub adapter_name: String,
    pub tx_speed_bps: u64,
    pub rx_speed_bps: u64,
}

/// Unified physical link model representing the active physical transmission medium.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhysicalLinkInfo {
    Wifi(WifiPhyMetrics),
    Ethernet(EthernetLinkMetrics),
}

impl PhysicalLinkInfo {
    /// Formats a concise physical link summary string for Adaptive Cards and tooltips.
    pub fn display_summary(&self) -> String {
        match self {
            Self::Wifi(w) => {
                let mut parts = Vec::new();
                let gen_str = w.generation.as_str();
                if !gen_str.is_empty() {
                    parts.push(gen_str.to_string());
                }
                let band_str = w.band.as_str();
                if !band_str.is_empty() {
                    parts.push(band_str.to_string());
                }
                if let Some(rssi) = w.rssi_dbm {
                    parts.push(format!("{} dBm", rssi));
                } else {
                    parts.push(format!("{}%", w.signal_quality_pct));
                }
                if let Some(tx) = w.tx_rate_mbps {
                    parts.push(format!("{} Mbps", tx));
                }
                parts.join(" · ")
            }
            Self::Ethernet(e) => {
                let max_speed = e.tx_speed_bps.max(e.rx_speed_bps);
                if max_speed >= 1_000_000_000 {
                    let gbps = max_speed as f64 / 1_000_000_000.0;
                    if gbps.fract() == 0.0 {
                        format!("Ethernet · {:.0} Gbps", gbps)
                    } else {
                        format!("Ethernet · {:.1} Gbps", gbps)
                    }
                } else if max_speed >= 1_000_000 {
                    format!("Ethernet · {} Mbps", max_speed / 1_000_000)
                } else {
                    "Ethernet".to_string()
                }
            }
        }
    }
}

/// Discovered IPv6 routing parameters from `GetBestRoute2`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ipv6Route {
    pub interface_luid: u64,
    pub interface_index: u32,
    pub source: std::net::Ipv6Addr,
    pub gateway: Option<std::net::Ipv6Addr>,
}

/// Operational state of network latency telemetry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LatencyState {
    /// Ping not yet sampled, disabled, or no adapter is connected.
    #[default]
    Unavailable,
    /// Successfully received ICMP echo reply within timeout.
    Healthy,
    /// Partial packet loss observed across recent probes.
    Degraded,
    /// ICMP echo request timed out without response.
    Timeout,
}

/// Point-in-time network round-trip latency measurement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatencySnapshot {
    /// Round-trip time in milliseconds if Healthy or Degraded.
    pub latency_ms: Option<u32>,
    /// State of the probe.
    pub state: LatencyState,
    /// Whether measuring LAN gateway or public internet.
    pub target: LatencyTarget,
    /// Monotonically increasing sequence number.
    pub sequence: u64,
    /// Unix timestamp in seconds when the probe was completed.
    pub sampled_at_unix: u64,
    /// Jitter (latency variance from previous probe) in milliseconds.
    pub jitter_ms: Option<u32>,
    /// IP Protocol utilized for the probe (IPv4 or IPv6).
    #[serde(default)]
    pub protocol: Option<IpProtocol>,
    /// Rolling packet loss percentage (0..=100). None if no valid probe attempts.
    #[serde(default)]
    pub packet_loss_pct: Option<u8>,
    /// Number of lost pings in the rolling window.
    #[serde(default)]
    pub pings_lost: u32,
    /// Total number of valid probe attempts in the rolling window.
    #[serde(default)]
    pub pings_total: u32,
    /// Evaluated latency health status.
    #[serde(default)]
    pub health: LatencyHealth,
}

impl Default for LatencySnapshot {
    fn default() -> Self {
        Self {
            latency_ms: None,
            state: LatencyState::Unavailable,
            target: LatencyTarget::Gateway,
            sequence: 0,
            sampled_at_unix: 0,
            jitter_ms: None,
            protocol: None,
            packet_loss_pct: None,
            pings_lost: 0,
            pings_total: 0,
            health: LatencyHealth::Unavailable,
        }
    }
}

impl LatencySnapshot {
    pub fn is_fresh(&self, now_unix: u64, max_age_secs: u64) -> bool {
        self.sampled_at_unix > 0 && now_unix.saturating_sub(self.sampled_at_unix) <= max_age_secs
    }

    /// User-facing short string for header status pill: "18 ms", "18 ms (5% loss)", "Timeout", "-- ms".
    pub fn display_text(&self) -> String {
        match self.state {
            LatencyState::Healthy => {
                if let Some(ms) = self.latency_ms {
                    if let Some(loss) = self.packet_loss_pct {
                        format!("{} ms · {}% loss", ms, loss)
                    } else {
                        format!("{} ms", ms)
                    }
                } else {
                    "-- ms".to_string()
                }
            }
            LatencyState::Degraded => {
                if let (Some(ms), Some(loss)) = (self.latency_ms, self.packet_loss_pct) {
                    format!("{} ms ({}% loss)", ms, loss)
                } else if let Some(ms) = self.latency_ms {
                    format!("{} ms", ms)
                } else {
                    "Degraded".to_string()
                }
            }
            LatencyState::Timeout => {
                if let Some(loss) = self.packet_loss_pct {
                    format!("Timeout · {}% loss", loss)
                } else {
                    "Timeout".to_string()
                }
            }
            LatencyState::Unavailable => "-- ms".to_string(),
        }
    }

    /// Detailed display string: "18 ms (±2 ms) · 0% loss · Internet", "Timeout · Gateway", "-- ms".
    pub fn detailed_display_text(&self) -> String {
        match self.state {
            LatencyState::Healthy => {
                if let Some(ms) = self.latency_ms {
                    let mut s = if let Some(jitter) = self.jitter_ms {
                        if jitter > 0 {
                            format!("{} ms (±{} ms)", ms, jitter)
                        } else {
                            format!("{} ms", ms)
                        }
                    } else {
                        format!("{} ms", ms)
                    };
                    if let Some(loss) = self.packet_loss_pct {
                        s.push_str(&format!(" · {}% loss", loss));
                    }
                    s.push_str(&format!(" · {}", self.target.label()));
                    if let Some(proto) = self.protocol {
                        s.push_str(&format!(" ({})", proto.as_str()));
                    }
                    s
                } else {
                    format!("-- ms · {}", self.target.label())
                }
            }
            LatencyState::Degraded => {
                let mut s = if let Some(ms) = self.latency_ms {
                    if let Some(jitter) = self.jitter_ms {
                        if jitter > 0 {
                            format!("{} ms (±{} ms)", ms, jitter)
                        } else {
                            format!("{} ms", ms)
                        }
                    } else {
                        format!("{} ms", ms)
                    }
                } else {
                    "Degraded".to_string()
                };
                if let Some(loss) = self.packet_loss_pct {
                    s.push_str(&format!(" · {}% loss", loss));
                }
                s.push_str(&format!(" · {}", self.target.label()));
                if let Some(proto) = self.protocol {
                    s.push_str(&format!(" ({})", proto.as_str()));
                }
                s
            }
            LatencyState::Timeout => {
                let mut s = if let Some(loss) = self.packet_loss_pct {
                    format!("Timeout · {}% loss · {}", loss, self.target.label())
                } else {
                    format!("Timeout · {}", self.target.label())
                };
                if let Some(proto) = self.protocol {
                    s.push_str(&format!(" ({})", proto.as_str()));
                }
                s
            }
            LatencyState::Unavailable => "-- ms".to_string(),
        }
    }
}

/// Telemetry snapshot containing current rates, totals, active apps, and chart history.
#[derive(Debug, Clone)]
pub struct NetworkSnapshot {
    pub generation: u64,
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
    pub latency: LatencySnapshot,
    pub physical_link: Option<PhysicalLinkInfo>,
    pub budget: Option<crate::budget::BudgetSnapshot>,
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
            generation: 1,
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
            latency: LatencySnapshot::default(),
            physical_link: None,
            budget: None,
        }
    }
}

pub fn compute_delta(prev: u64, curr: u64) -> u64 {
    curr.saturating_sub(prev)
}

/// Discontinuity-aware counter delta evaluation.
///
/// Returns None when `curr < prev` (indicating a counter reset, interface restart,
/// driver reload, machine resume, or counter wrap). When None is returned, 0 bytes
/// should be attributed to this tick rather than erroneously adding the current counter value.
pub fn counter_delta(prev: u64, curr: u64) -> Option<u64> {
    if curr < prev { None } else { Some(curr - prev) }
}

/// Tracking byte counts for a specific network interface.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct InterfaceCounterState {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

/// Core telemetry backend that queries Windows network adapters, computes bandwidth, and tracks apps.
pub struct NetworkBackend {
    pub mode: AggregateMode,
    /// Baseline byte counts for all known interfaces to avoid spikes when an adapter comes online.
    prev_counters: HashMap<InterfaceLuid, InterfaceCounterState>,
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
            chart_peak_rx: 0,
            chart_peak_tx: 0,
            process_tracker: crate::process::ProcessTracker::new(),
            last_process_sample: None,
            cached_raw_apps: (Vec::new(), 0),
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
            chart_peak_rx: 0,
            chart_peak_tx: 0,
            process_tracker: crate::process::ProcessTracker::new(),
            last_process_sample: None,
            cached_raw_apps: (Vec::new(), 0),
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
        self.chart_peak_rx = 0;
        self.chart_peak_tx = 0;
        self.accumulator.reset();
        self.rolling_window.reset();
        self.process_tracker.reset();
        self.last_process_sample = None;
        self.cached_raw_apps = (Vec::new(), 0);
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
            self.chart_peak_rx = 0;
            self.chart_peak_tx = 0;
            self.accumulator.reset();
            self.rolling_window.reset();
            self.process_tracker.reset();
            self.last_process_sample = None;
            self.cached_raw_apps = (Vec::new(), 0);
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
        self.physical_link = query_physical_link_info(snapshot.primary_medium);
        snapshot.physical_link = self.physical_link.clone();

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

        let budget_snap = crate::budget::calculate_budget_snapshot(
            &self.budget_config,
            &mut self.daily_usage,
            today_ymd,
        );

        NetworkSnapshot {
            generation: self.generation,
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
            latency: self.latency,
            physical_link: self.physical_link.clone(),
            budget: Some(budget_snap),
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
struct WLAN_RATE_SET {
    uRateSetLength: u32,
    usRateSet: [u16; 126],
}

#[allow(non_snake_case, dead_code, clippy::upper_case_acronyms)]
#[repr(C)]
struct WLAN_REALTIME_CONNECTION_QUALITY_LINK_INFO {
    ucLinkID: u8,
    ulChannelCenterFrequencyMhz: u32,
    ulBandwidth: u32,
    lRssi: i32,
    wlanRateSet: WLAN_RATE_SET,
}

#[allow(non_snake_case, dead_code, clippy::upper_case_acronyms)]
#[repr(C)]
struct WLAN_REALTIME_CONNECTION_QUALITY {
    dot11PhyType: u32,
    ulLinkQuality: u32,
    ulRxRate: u32,
    ulTxRate: u32,
    bIsMLOConnection: i32,
    ulNumLinks: u32,
    linksInfo: [WLAN_REALTIME_CONNECTION_QUALITY_LINK_INFO; 1],
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

const WLAN_INTF_OPCODE_CURRENT_CONNECTION: u32 = 7;
const WLAN_INTF_OPCODE_CHANNEL_NUMBER: u32 = 8;
const WLAN_INTF_OPCODE_REALTIME_CONNECTION_QUALITY: u32 = 19;

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

/// Queries deep Wi-Fi physical layer (PHY) telemetry using Windows Native Wi-Fi API.
/// Uses `WLAN_REALTIME_CONNECTION_QUALITY` as the primary rate, quality, and MLO source
/// without requiring Windows location permissions, with graceful fallback to connection attributes.
pub fn query_active_wifi_metrics() -> Option<WifiPhyMetrics> {
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
            let _ = WlanCloseHandle(handle, std::ptr::null_mut());
            return None;
        }

        let mut found_metrics: Option<WifiPhyMetrics> = None;
        let count = (*list_ptr).dwNumberOfItems;
        if count > 0 {
            let interfaces =
                std::slice::from_raw_parts((*list_ptr).InterfaceInfo.as_ptr(), count as usize);
            for iface in interfaces {
                // wlan_interface_state_connected = 1
                if iface.isState == 1 {
                    let mut data_size = 0u32;
                    let mut data_ptr: *mut core::ffi::c_void = std::ptr::null_mut();

                    // 1. Query current connection to extract SSID
                    let mut ssid = String::new();
                    let query_conn = WlanQueryInterface(
                        handle,
                        &iface.InterfaceGuid,
                        WLAN_INTF_OPCODE_CURRENT_CONNECTION,
                        std::ptr::null_mut(),
                        &mut data_size,
                        &mut data_ptr,
                        std::ptr::null_mut(),
                    );
                    let mut fallback_phy = 0u32;
                    let mut fallback_quality = 0u32;
                    let mut fallback_rx_rate = 0u32;
                    let mut fallback_tx_rate = 0u32;

                    if query_conn == 0 && !data_ptr.is_null() {
                        let conn_attrs = &*(data_ptr as *const WLAN_CONNECTION_ATTRIBUTES);
                        let ssid_len =
                            conn_attrs.wlanAssociationAttributes.dot11Ssid.uSSIDLength as usize;
                        if ssid_len > 0 && ssid_len <= 32 {
                            let bytes =
                                &conn_attrs.wlanAssociationAttributes.dot11Ssid.ucSSID[..ssid_len];
                            let ssid_lossy = String::from_utf8_lossy(bytes);
                            let trimmed = ssid_lossy.trim_matches(['\0', ' ']);
                            if !trimmed.is_empty() {
                                ssid = trimmed.to_string();
                            }
                        }
                        if ssid.is_empty() {
                            let prof = wchar_to_string(&conn_attrs.strProfileName);
                            let trimmed = prof.trim();
                            if !trimmed.is_empty() {
                                ssid = trimmed.to_string();
                            }
                        }
                        fallback_phy = conn_attrs.wlanAssociationAttributes.dot11PhyType;
                        fallback_quality = conn_attrs.wlanAssociationAttributes.wlanSignalQuality;
                        fallback_rx_rate = conn_attrs.wlanAssociationAttributes.ulRxRate;
                        fallback_tx_rate = conn_attrs.wlanAssociationAttributes.ulTxRate;
                        WlanFreeMemory(data_ptr);
                    }

                    if ssid.is_empty() {
                        ssid = "Wi-Fi".to_string();
                    }

                    // 2. Query WLAN_REALTIME_CONNECTION_QUALITY (Opcode 19)
                    let mut rt_size = 0u32;
                    let mut rt_ptr: *mut core::ffi::c_void = std::ptr::null_mut();
                    let query_rt = WlanQueryInterface(
                        handle,
                        &iface.InterfaceGuid,
                        WLAN_INTF_OPCODE_REALTIME_CONNECTION_QUALITY,
                        std::ptr::null_mut(),
                        &mut rt_size,
                        &mut rt_ptr,
                        std::ptr::null_mut(),
                    );

                    if query_rt == 0 && !rt_ptr.is_null() {
                        let rt = &*(rt_ptr as *const WLAN_REALTIME_CONNECTION_QUALITY);
                        let wifi_gen = wifi_generation(rt.dot11PhyType);
                        let quality_pct = rt.ulLinkQuality.min(100) as u8;
                        let is_mlo = rt.bIsMLOConnection != 0;
                        let link_count = rt.ulNumLinks.clamp(1, 255) as u8;

                        let (freq_mhz, rssi_dbm) = if rt.ulNumLinks > 0 {
                            let link = &rt.linksInfo[0];
                            let r = if link.lRssi != 0 {
                                Some(link.lRssi as i16)
                            } else {
                                Some(signal_quality_to_rssi_dbm(quality_pct))
                            };
                            (link.ulChannelCenterFrequencyMhz, r)
                        } else {
                            (0, Some(signal_quality_to_rssi_dbm(quality_pct)))
                        };

                        let band = wifi_band(freq_mhz, None);
                        let tx_mbps = if rt.ulTxRate > 0 {
                            Some(rt.ulTxRate / 1000)
                        } else {
                            None
                        };
                        let rx_mbps = if rt.ulRxRate > 0 {
                            Some(rt.ulRxRate / 1000)
                        } else {
                            None
                        };

                        WlanFreeMemory(rt_ptr);

                        found_metrics = Some(WifiPhyMetrics {
                            ssid,
                            generation: wifi_gen,
                            band,
                            channel: if freq_mhz > 0 { Some(freq_mhz) } else { None },
                            signal_quality_pct: quality_pct,
                            rssi_dbm,
                            tx_rate_mbps: tx_mbps,
                            rx_rate_mbps: rx_mbps,
                            is_mlo,
                            link_count,
                        });
                    } else {
                        // 3. Fallback to association attributes + channel query
                        let wifi_gen = wifi_generation(fallback_phy);
                        let quality_pct = fallback_quality.min(100) as u8;
                        let rssi = Some(signal_quality_to_rssi_dbm(quality_pct));
                        let tx_mbps = if fallback_tx_rate > 0 {
                            Some(fallback_tx_rate / 1000)
                        } else {
                            None
                        };
                        let rx_mbps = if fallback_rx_rate > 0 {
                            Some(fallback_rx_rate / 1000)
                        } else {
                            None
                        };

                        // Query channel number (Opcode 8)
                        let mut ch_size = 0u32;
                        let mut ch_ptr: *mut core::ffi::c_void = std::ptr::null_mut();
                        let mut channel_opt: Option<u32> = None;
                        if WlanQueryInterface(
                            handle,
                            &iface.InterfaceGuid,
                            WLAN_INTF_OPCODE_CHANNEL_NUMBER,
                            std::ptr::null_mut(),
                            &mut ch_size,
                            &mut ch_ptr,
                            std::ptr::null_mut(),
                        ) == 0
                            && !ch_ptr.is_null()
                        {
                            let ch = *(ch_ptr as *const u32);
                            if ch > 0 {
                                channel_opt = Some(ch);
                            }
                            WlanFreeMemory(ch_ptr);
                        }

                        let band = wifi_band(0, channel_opt);

                        found_metrics = Some(WifiPhyMetrics {
                            ssid,
                            generation: wifi_gen,
                            band,
                            channel: channel_opt,
                            signal_quality_pct: quality_pct,
                            rssi_dbm: rssi,
                            tx_rate_mbps: tx_mbps,
                            rx_rate_mbps: rx_mbps,
                            is_mlo: false,
                            link_count: 1,
                        });
                    }

                    if found_metrics.is_some() {
                        break;
                    }
                }
            }
        }

        WlanFreeMemory(list_ptr as *mut core::ffi::c_void);
        let _ = WlanCloseHandle(handle, std::ptr::null_mut());
        found_metrics
    }
}

static WIFI_PHY_CACHE: std::sync::Mutex<(Option<WifiPhyMetrics>, Option<Instant>)> =
    std::sync::Mutex::new((None, None));

/// Cached Wi-Fi PHY metrics lookup with a 2-second TTL to avoid spamming WlanAPI on every 500ms tick.
pub fn query_cached_wifi_phy() -> Option<WifiPhyMetrics> {
    const CACHE_TTL: std::time::Duration = std::time::Duration::from_secs(2);
    let now = Instant::now();
    if let Ok(mut cache) = WIFI_PHY_CACHE.lock() {
        if let (Some(phy), Some(last_query)) = &*cache
            && now.duration_since(*last_query) < CACHE_TTL
        {
            return Some(phy.clone());
        }
        let fresh = query_active_wifi_metrics();
        *cache = (fresh.clone(), Some(now));
        fresh
    } else {
        query_active_wifi_metrics()
    }
}

/// Cached Wi-Fi SSID lookup utilizing the unified Wi-Fi cache.
pub fn query_cached_wifi_ssid() -> Option<String> {
    query_cached_wifi_phy().map(|w| w.ssid)
}

/// Query active Ethernet connection link speed using IP Helper MIB_IF_ROW2.
pub fn query_active_ethernet_metrics() -> Option<EthernetLinkMetrics> {
    use windows::Win32::NetworkManagement::IpHelper::{FreeMibTable, GetIfTable2, MIB_IF_TABLE2};

    unsafe {
        let mut table: *mut MIB_IF_TABLE2 = std::ptr::null_mut();
        if GetIfTable2(&mut table).0 != 0 || table.is_null() {
            return None;
        }

        let num_entries = (*table).NumEntries as usize;
        let rows = std::slice::from_raw_parts((*table).Table.as_ptr(), num_entries);
        let mut best: Option<EthernetLinkMetrics> = None;

        for row in rows {
            // IF_TYPE_ETHERNET_CSMACD = 6, IF_TYPE_GIGABITETHERNET = 117, IF_TYPE_FASTETHER = 62, IfOperStatusUp = 1
            if (row.Type == 6 || row.Type == 117 || row.Type == 62) && row.OperStatus.0 == 1 {
                let tx_bps = row.TransmitLinkSpeed;
                let rx_bps = row.ReceiveLinkSpeed;
                let desc = wchar_to_string(&row.Description);
                let alias = wchar_to_string(&row.Alias);
                let name = if !alias.is_empty() {
                    alias
                } else if !desc.is_empty() {
                    desc
                } else {
                    "Ethernet".to_string()
                };

                best = Some(EthernetLinkMetrics {
                    adapter_name: name,
                    tx_speed_bps: tx_bps,
                    rx_speed_bps: rx_bps,
                });
                break;
            }
        }

        FreeMibTable(table as *const core::ffi::c_void);
        best
    }
}

/// Resolves physical layer link information according to active medium.
pub fn query_physical_link_info(primary_medium: InterfaceMedium) -> Option<PhysicalLinkInfo> {
    match primary_medium {
        InterfaceMedium::Wifi => query_cached_wifi_phy().map(PhysicalLinkInfo::Wifi),
        InterfaceMedium::Ethernet => {
            query_active_ethernet_metrics().map(PhysicalLinkInfo::Ethernet)
        }
        _ => None,
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

/// Query the active default IPv4 gateway address.
///
/// Uses Windows `GetBestRoute2` towards public internet (`1.1.1.1`) to determine
/// the exact next-hop gateway Windows uses on the active network route.
/// Falls back to scanning default route entries (`0.0.0.0/0`) via `GetIpForwardTable2`.
pub fn query_ipv4_gateway_address(interface_luid: Option<u64>) -> Option<std::net::Ipv4Addr> {
    use windows::Win32::NetworkManagement::IpHelper::{
        FreeMibTable, GetBestRoute2, GetIpForwardTable2, MIB_IPFORWARD_ROW2, MIB_IPFORWARD_TABLE2,
    };
    use windows::Win32::NetworkManagement::Ndis::NET_LUID_LH;
    use windows::Win32::Networking::WinSock::{AF_INET, SOCKADDR_INET};

    unsafe {
        // Destination address: 1.1.1.1 (public internet probe route)
        let mut dest: SOCKADDR_INET = std::mem::zeroed();
        dest.si_family = AF_INET;
        dest.Ipv4.sin_family = AF_INET;
        dest.Ipv4.sin_addr.S_un.S_addr = u32::from_ne_bytes([1, 1, 1, 1]);

        let mut best_route: MIB_IPFORWARD_ROW2 = std::mem::zeroed();
        let mut best_source: SOCKADDR_INET = std::mem::zeroed();

        let luid_val = interface_luid.map(|l| NET_LUID_LH { Value: l });
        let luid_ptr = luid_val.as_ref().map(|l| l as *const _);

        let status = GetBestRoute2(
            luid_ptr,
            0,
            None,
            &dest,
            0,
            &mut best_route,
            &mut best_source,
        );

        if status.0 == 0 && best_route.NextHop.si_family == AF_INET {
            let s_addr = best_route.NextHop.Ipv4.sin_addr.S_un.S_addr;
            if s_addr != 0 {
                return Some(std::net::Ipv4Addr::from(s_addr.to_ne_bytes()));
            }
        }

        // Fallback: enumerate IPv4 forwarding table for default 0.0.0.0/0 route
        let mut table: *mut MIB_IPFORWARD_TABLE2 = std::ptr::null_mut();
        if GetIpForwardTable2(AF_INET, &mut table).0 == 0 && !table.is_null() {
            let entries = (*table).NumEntries as usize;
            let rows = std::slice::from_raw_parts((*table).Table.as_ptr(), entries);
            let mut best_gateway = None;
            let mut lowest_metric = u32::MAX;

            for row in rows {
                if let Some(target_luid) = interface_luid
                    && row.InterfaceLuid.Value != target_luid
                {
                    continue;
                }
                if row.DestinationPrefix.PrefixLength == 0 && row.NextHop.si_family == AF_INET {
                    let gw = row.NextHop.Ipv4.sin_addr.S_un.S_addr;
                    if gw != 0 && row.Metric < lowest_metric {
                        lowest_metric = row.Metric;
                        best_gateway = Some(std::net::Ipv4Addr::from(gw.to_ne_bytes()));
                    }
                }
            }

            FreeMibTable(table as *const core::ffi::c_void);
            if best_gateway.is_some() {
                return best_gateway;
            }
        }
    }

    None
}

/// Query the active default IPv6 route and gateway using `GetBestRoute2`.
///
/// Uses Windows `GetBestRoute2` towards Cloudflare IPv6 DNS (`2606:4700:4700::1111`) to determine
/// the preferred interface, best source address, and next-hop IPv6 gateway.
/// Falls back to scanning default route entries (`::/0`) via `GetIpForwardTable2`.
pub fn query_ipv6_route(interface_luid: Option<u64>) -> Option<Ipv6Route> {
    use windows::Win32::NetworkManagement::IpHelper::{
        FreeMibTable, GetBestRoute2, GetIpForwardTable2, MIB_IPFORWARD_ROW2, MIB_IPFORWARD_TABLE2,
    };
    use windows::Win32::NetworkManagement::Ndis::NET_LUID_LH;
    use windows::Win32::Networking::WinSock::{AF_INET6, IN6_ADDR, SOCKADDR_INET};

    unsafe {
        let mut dest: SOCKADDR_INET = std::mem::zeroed();
        dest.si_family = AF_INET6;
        dest.Ipv6.sin6_family = AF_INET6;
        dest.Ipv6.sin6_addr = IN6_ADDR {
            u: windows::Win32::Networking::WinSock::IN6_ADDR_0 {
                Byte: [
                    0x26, 0x06, 0x47, 0x00, 0x47, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x11, 0x11,
                ],
            },
        };

        let mut best_route: MIB_IPFORWARD_ROW2 = std::mem::zeroed();
        let mut best_source: SOCKADDR_INET = std::mem::zeroed();

        let luid_val = interface_luid.map(|l| NET_LUID_LH { Value: l });
        let luid_ptr = luid_val.as_ref().map(|l| l as *const _);

        let status = GetBestRoute2(
            luid_ptr,
            0,
            None,
            &dest,
            0,
            &mut best_route,
            &mut best_source,
        );

        if status.0 == 0 {
            let source_ip = std::net::Ipv6Addr::from(best_source.Ipv6.sin6_addr.u.Byte);
            let gateway_ip = if best_route.NextHop.si_family == AF_INET6 {
                let bytes = best_route.NextHop.Ipv6.sin6_addr.u.Byte;
                let addr = std::net::Ipv6Addr::from(bytes);
                if !addr.is_unspecified() {
                    Some(addr)
                } else {
                    None
                }
            } else {
                None
            };

            return Some(Ipv6Route {
                interface_luid: best_route.InterfaceLuid.Value,
                interface_index: best_route.InterfaceIndex,
                source: source_ip,
                gateway: gateway_ip,
            });
        }

        // Fallback: check IPv6 forward table for default ::/0 route
        let mut table: *mut MIB_IPFORWARD_TABLE2 = std::ptr::null_mut();
        if GetIpForwardTable2(AF_INET6, &mut table).0 == 0 && !table.is_null() {
            let entries = (*table).NumEntries as usize;
            let rows = std::slice::from_raw_parts((*table).Table.as_ptr(), entries);
            let mut best_route_match = None;
            let mut lowest_metric = u32::MAX;

            for row in rows {
                if let Some(target_luid) = interface_luid
                    && row.InterfaceLuid.Value != target_luid
                {
                    continue;
                }
                if row.DestinationPrefix.PrefixLength == 0 && row.NextHop.si_family == AF_INET6 {
                    let bytes = row.NextHop.Ipv6.sin6_addr.u.Byte;
                    let addr = std::net::Ipv6Addr::from(bytes);
                    if !addr.is_unspecified() && row.Metric < lowest_metric {
                        lowest_metric = row.Metric;
                        best_route_match = Some(Ipv6Route {
                            interface_luid: row.InterfaceLuid.Value,
                            interface_index: row.InterfaceIndex,
                            source: std::net::Ipv6Addr::UNSPECIFIED,
                            gateway: Some(addr),
                        });
                    }
                }
            }

            FreeMibTable(table as *const core::ffi::c_void);
            if best_route_match.is_some() {
                return best_route_match;
            }
        }
    }

    None
}

/// Probes round-trip latency to the given IPv4 target using asynchronous Win32 `IcmpSendEcho2`.
pub fn probe_latency_ipv4(target_ip: std::net::Ipv4Addr, timeout_ms: u32) -> ProbeResult {
    use windows::Win32::Foundation::{CloseHandle, WAIT_OBJECT_0, WAIT_TIMEOUT};
    use windows::Win32::NetworkManagement::IpHelper::{
        ICMP_ECHO_REPLY, IcmpCloseHandle, IcmpCreateFile, IcmpParseReplies, IcmpSendEcho2,
    };
    use windows::Win32::System::Threading::{CreateEventW, WaitForSingleObject};

    unsafe {
        let icmp_handle = match IcmpCreateFile() {
            Ok(h) if !h.is_invalid() => h,
            _ => return ProbeResult::Unavailable,
        };

        let event = match CreateEventW(None, false, false, None) {
            Ok(h) => h,
            Err(_) => {
                let _ = IcmpCloseHandle(icmp_handle);
                return ProbeResult::Unavailable;
            }
        };

        let reply_buf_len = size_of::<ICMP_ECHO_REPLY>() + 64;
        let mut reply_buf = vec![0u8; reply_buf_len];
        let send_data = *b"NetFlow";
        let dest_addr = u32::from_ne_bytes(target_ip.octets());

        let _ = IcmpSendEcho2(
            icmp_handle,
            Some(event),
            None,
            None,
            dest_addr,
            send_data.as_ptr() as *const _,
            send_data.len() as u16,
            None,
            reply_buf.as_mut_ptr() as *mut _,
            reply_buf_len as u32,
            timeout_ms,
        );

        let wait_result = WaitForSingleObject(event, timeout_ms);
        let _ = CloseHandle(event);

        if wait_result == WAIT_OBJECT_0 {
            let parse_count =
                IcmpParseReplies(reply_buf.as_mut_ptr() as *mut _, reply_buf_len as u32);
            let _ = IcmpCloseHandle(icmp_handle);

            if parse_count > 0 {
                let reply = &*(reply_buf.as_ptr() as *const ICMP_ECHO_REPLY);
                if reply.Status == 0 {
                    ProbeResult::Success {
                        latency: Duration::from_millis(reply.RoundTripTime as u64),
                    }
                } else {
                    ProbeResult::Timeout
                }
            } else {
                ProbeResult::Timeout
            }
        } else if wait_result == WAIT_TIMEOUT {
            let _ = IcmpCloseHandle(icmp_handle);
            ProbeResult::Timeout
        } else {
            let _ = IcmpCloseHandle(icmp_handle);
            ProbeResult::Unavailable
        }
    }
}

/// Probes round-trip latency to the given IPv6 target using asynchronous Win32 `Icmp6SendEcho2`.
pub fn probe_latency_ipv6(target_ip: std::net::Ipv6Addr, timeout_ms: u32) -> ProbeResult {
    use windows::Win32::Foundation::{CloseHandle, WAIT_OBJECT_0, WAIT_TIMEOUT};
    use windows::Win32::NetworkManagement::IpHelper::{
        ICMPV6_ECHO_REPLY_LH, Icmp6CreateFile, Icmp6ParseReplies, Icmp6SendEcho2, IcmpCloseHandle,
    };
    use windows::Win32::Networking::WinSock::{AF_INET6, IN6_ADDR, SOCKADDR_IN6};
    use windows::Win32::System::Threading::{CreateEventW, WaitForSingleObject};

    unsafe {
        let icmp_handle = match Icmp6CreateFile() {
            Ok(h) if !h.is_invalid() => h,
            _ => return ProbeResult::Unavailable,
        };

        let event = match CreateEventW(None, false, false, None) {
            Ok(h) => h,
            Err(_) => {
                let _ = IcmpCloseHandle(icmp_handle);
                return ProbeResult::Unavailable;
            }
        };

        let reply_buf_len = size_of::<ICMPV6_ECHO_REPLY_LH>() + 128;
        let mut reply_buf = vec![0u8; reply_buf_len];
        let send_data = *b"NetFlow";

        let mut dest: SOCKADDR_IN6 = std::mem::zeroed();
        dest.sin6_family = AF_INET6;
        dest.sin6_addr = IN6_ADDR {
            u: windows::Win32::Networking::WinSock::IN6_ADDR_0 {
                Byte: target_ip.octets(),
            },
        };

        let mut source: SOCKADDR_IN6 = std::mem::zeroed();
        source.sin6_family = AF_INET6;

        let _ = Icmp6SendEcho2(
            icmp_handle,
            Some(event),
            None,
            None,
            &source,
            &dest,
            send_data.as_ptr() as *const _,
            send_data.len() as u16,
            None,
            reply_buf.as_mut_ptr() as *mut _,
            reply_buf_len as u32,
            timeout_ms,
        );

        let wait_result = WaitForSingleObject(event, timeout_ms);
        let _ = CloseHandle(event);

        if wait_result == WAIT_OBJECT_0 {
            let parse_count =
                Icmp6ParseReplies(reply_buf.as_mut_ptr() as *mut _, reply_buf_len as u32);
            let _ = IcmpCloseHandle(icmp_handle);

            if parse_count > 0 {
                let reply = &*(reply_buf.as_ptr() as *const ICMPV6_ECHO_REPLY_LH);
                if reply.Status == 0 {
                    ProbeResult::Success {
                        latency: Duration::from_millis(reply.RoundTripTime as u64),
                    }
                } else {
                    ProbeResult::Timeout
                }
            } else {
                ProbeResult::Timeout
            }
        } else if wait_result == WAIT_TIMEOUT {
            let _ = IcmpCloseHandle(icmp_handle);
            ProbeResult::Timeout
        } else {
            let _ = IcmpCloseHandle(icmp_handle);
            ProbeResult::Unavailable
        }
    }
}

/// Platform-neutral internal probe function dispatching between IPv4 and IPv6 ICMP echo APIs.
pub fn probe_latency(target: std::net::IpAddr, timeout_ms: u32) -> ProbeResult {
    match target {
        std::net::IpAddr::V4(ipv4) => probe_latency_ipv4(ipv4, timeout_ms),
        std::net::IpAddr::V6(ipv6) => probe_latency_ipv6(ipv6, timeout_ms),
    }
}

/// Probes round-trip latency using dual-stack IPv6-first routing with seamless IPv4 fallback.
/// Returns the constructed `LatencySnapshot` along with the individual `ProbeResult`.
pub fn sample_latency_snapshot_dual_stack(
    mode: LatencyTargetMode,
    sequence: u64,
    timeout_ms: u32,
    prev_ms: Option<u32>,
) -> (LatencySnapshot, ProbeResult) {
    let now_unix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let ipv6_route = query_ipv6_route(None);
    let ipv4_gateway = query_ipv4_gateway_address(None);

    let (target, primary_ipv6, fallback_ipv4) = match mode {
        LatencyTargetMode::Internet => (
            LatencyTarget::Internet,
            Some(std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1111,
            ))),
            Some(std::net::IpAddr::V4(std::net::Ipv4Addr::new(1, 1, 1, 1))),
        ),
        LatencyTargetMode::Gateway => {
            let gw_v6 = ipv6_route
                .as_ref()
                .and_then(|r| r.gateway)
                .map(std::net::IpAddr::V6);
            let gw_v4 = ipv4_gateway.map(std::net::IpAddr::V4);
            (LatencyTarget::Gateway, gw_v6, gw_v4)
        }
        LatencyTargetMode::Auto => {
            if let Some(gw_v6) = ipv6_route.as_ref().and_then(|r| r.gateway) {
                (
                    LatencyTarget::Gateway,
                    Some(std::net::IpAddr::V6(gw_v6)),
                    ipv4_gateway.map(std::net::IpAddr::V4),
                )
            } else if let Some(gw_v4) = ipv4_gateway {
                (
                    LatencyTarget::Gateway,
                    None,
                    Some(std::net::IpAddr::V4(gw_v4)),
                )
            } else {
                (
                    LatencyTarget::Internet,
                    Some(std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                        0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1111,
                    ))),
                    Some(std::net::IpAddr::V4(std::net::Ipv4Addr::new(1, 1, 1, 1))),
                )
            }
        }
    };

    // 1. Try IPv6 first if route / target is available
    if let Some(ip6) = primary_ipv6 {
        let res = probe_latency(ip6, timeout_ms);
        match res {
            ProbeResult::Success { latency } => {
                let ms = latency.as_millis() as u32;
                let jitter = prev_ms.map(|p| ms.abs_diff(p));
                let snap = LatencySnapshot {
                    latency_ms: Some(ms),
                    state: LatencyState::Healthy,
                    target,
                    sequence,
                    sampled_at_unix: now_unix,
                    jitter_ms: jitter,
                    protocol: Some(IpProtocol::Ipv6),
                    packet_loss_pct: None,
                    pings_lost: 0,
                    pings_total: 0,
                    health: LatencyHealth::Healthy,
                };
                return (snap, res);
            }
            ProbeResult::Unavailable => {
                // IPv6 route unavailable -> fall back to IPv4
            }
            ProbeResult::Timeout => {
                // If IPv6 timed out, test if IPv4 has connectivity
                if let Some(ip4) = fallback_ipv4 {
                    let res4 = probe_latency(ip4, timeout_ms);
                    if let ProbeResult::Success { latency } = res4 {
                        let ms = latency.as_millis() as u32;
                        let jitter = prev_ms.map(|p| ms.abs_diff(p));
                        let snap = LatencySnapshot {
                            latency_ms: Some(ms),
                            state: LatencyState::Healthy,
                            target,
                            sequence,
                            sampled_at_unix: now_unix,
                            jitter_ms: jitter,
                            protocol: Some(IpProtocol::Ipv4),
                            packet_loss_pct: None,
                            pings_lost: 0,
                            pings_total: 0,
                            health: LatencyHealth::Healthy,
                        };
                        return (snap, res4);
                    }
                }
                let snap = LatencySnapshot {
                    latency_ms: None,
                    state: LatencyState::Timeout,
                    target,
                    sequence,
                    sampled_at_unix: now_unix,
                    jitter_ms: None,
                    protocol: Some(IpProtocol::Ipv6),
                    packet_loss_pct: None,
                    pings_lost: 0,
                    pings_total: 0,
                    health: LatencyHealth::Timeout,
                };
                return (snap, ProbeResult::Timeout);
            }
        }
    }

    // 2. IPv4 probe fallback
    if let Some(ip4) = fallback_ipv4 {
        let res = probe_latency(ip4, timeout_ms);
        let (state, ms) = match res {
            ProbeResult::Success { latency } => {
                (LatencyState::Healthy, Some(latency.as_millis() as u32))
            }
            ProbeResult::Timeout => (LatencyState::Timeout, None),
            ProbeResult::Unavailable => (LatencyState::Unavailable, None),
        };
        let jitter = ms.and_then(|m| prev_ms.map(|p| m.abs_diff(p)));
        let health = match state {
            LatencyState::Healthy => LatencyHealth::Healthy,
            LatencyState::Timeout => LatencyHealth::Timeout,
            _ => LatencyHealth::Unavailable,
        };
        let snap = LatencySnapshot {
            latency_ms: ms,
            state,
            target,
            sequence,
            sampled_at_unix: now_unix,
            jitter_ms: jitter,
            protocol: Some(IpProtocol::Ipv4),
            packet_loss_pct: None,
            pings_lost: 0,
            pings_total: 0,
            health,
        };
        (snap, res)
    } else {
        let snap = LatencySnapshot {
            latency_ms: None,
            state: LatencyState::Unavailable,
            target,
            sequence,
            sampled_at_unix: now_unix,
            jitter_ms: None,
            protocol: None,
            packet_loss_pct: None,
            pings_lost: 0,
            pings_total: 0,
            health: LatencyHealth::Unavailable,
        };
        (snap, ProbeResult::Unavailable)
    }
}

/// Convenience method that resolves the active probe target according to `LatencyTargetMode`
/// and queries round-trip latency, returning a complete `LatencySnapshot`.
pub fn sample_latency_snapshot(
    mode: LatencyTargetMode,
    sequence: u64,
    timeout_ms: u32,
) -> LatencySnapshot {
    sample_latency_snapshot_dual_stack(mode, sequence, timeout_ms, None).0
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
    fn test_counter_delta_discontinuity() {
        assert_eq!(counter_delta(100, 250), Some(150));
        assert_eq!(counter_delta(250, 250), Some(0));
        assert_eq!(counter_delta(8_000_000, 100_000), None);
    }

    #[test]
    fn test_backend_daily_usage_accumulation() {
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
        // Sample 1 establishes baseline
        backend.sample_from_interfaces(&ifaces, t0);

        // Sample 2 adds 500 bytes RX and 300 bytes TX
        let t1 = t0 + Duration::from_secs(1);
        let ifaces2 = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Wifi,
            1,
            1500,
            2300,
        )];
        let snap = backend.sample_from_interfaces(&ifaces2, t1);
        assert_eq!(snap.session_rx, 500);
        assert_eq!(snap.session_tx, 300);
        assert!(snap.budget.is_some());

        let today_ymd = crate::budget::current_local_ymd();
        let today_str = crate::budget::format_ymd(today_ymd.0, today_ymd.1, today_ymd.2);
        let entry = backend.daily_usage.get_entry(&today_str).unwrap();
        assert_eq!(entry.rx_bytes, 500);
        assert_eq!(entry.tx_bytes, 300);
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
        let _ = query_cached_wifi_phy();
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
            generation: 1,
            session_rx: 1234567,
            session_tx: 987654,
            session_start_unix: 1700000000,
            all_time_peak_rx: 4500000.0,
            all_time_peak_tx: 1200000.0,
            updated_at_unix: 1700000500,
            latency: LatencySnapshot::default(),
            physical_link: None,
        };
        let serialized = serde_json::to_string(&state).expect("serialize");
        let deserialized: SessionState = serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(state, deserialized);

        // Verify backward-compatible aliases
        let json_with_aliases = r#"{
            "generation": 42,
            "download_bytes": 1000,
            "upload_bytes": 2000,
            "started_at": 1700000000
        }"#;
        let from_aliases: SessionState =
            serde_json::from_str(json_with_aliases).expect("deserialize aliases");
        assert_eq!(from_aliases.generation, 42);
        assert_eq!(from_aliases.session_rx, 1000);
        assert_eq!(from_aliases.session_tx, 2000);
        assert_eq!(from_aliases.session_start_unix, 1700000000);
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

    #[test]
    fn test_atomic_write_simulation() {
        let dir = std::env::temp_dir().join(format!("netflow_test_atomic_{}", std::process::id()));
        let file_path = dir.join("nested").join("test_atomic.json");

        // 1. Initial write creates directories and writes content
        let initial_payload = b"{\"generation\": 1, \"data\": \"init\"}";
        write_atomic(&file_path, initial_payload).expect("initial write should succeed");
        assert_eq!(std::fs::read(&file_path).unwrap(), initial_payload);

        // 2. Overwrite replaces content atomically
        let updated_payload = b"{\"generation\": 2, \"data\": \"updated\"}";
        write_atomic(&file_path, updated_payload).expect("replacement should succeed");
        assert_eq!(std::fs::read(&file_path).unwrap(), updated_payload);

        // Clean up
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_session_state_generation_increments() {
        let mut backend = NetworkBackend::new();
        assert_eq!(backend.generation, 1);
        backend.reset_session();
        assert_eq!(backend.generation, 2);
        backend.reset_session();
        assert_eq!(backend.generation, 3);
    }

    #[test]
    fn test_latency_snapshot_semantics_and_formatting() {
        let healthy = LatencySnapshot {
            latency_ms: Some(18),
            state: LatencyState::Healthy,
            target: LatencyTarget::Internet,
            sequence: 1,
            sampled_at_unix: 1700000000,
            jitter_ms: None,
            protocol: None,
            packet_loss_pct: None,
            pings_lost: 0,
            pings_total: 1,
            health: LatencyHealth::Healthy,
        };
        assert_eq!(healthy.display_text(), "18 ms");
        assert_eq!(healthy.detailed_display_text(), "18 ms · Internet");
        assert!(healthy.is_fresh(1700000005, 10));
        assert!(!healthy.is_fresh(1700000020, 10));

        let with_jitter = LatencySnapshot {
            latency_ms: Some(18),
            state: LatencyState::Healthy,
            target: LatencyTarget::Internet,
            sequence: 2,
            sampled_at_unix: 1700000001,
            jitter_ms: Some(2),
            protocol: None,
            packet_loss_pct: None,
            pings_lost: 0,
            pings_total: 2,
            health: LatencyHealth::Healthy,
        };
        assert_eq!(
            with_jitter.detailed_display_text(),
            "18 ms (±2 ms) · Internet"
        );

        let with_dual_stack_and_loss = LatencySnapshot {
            latency_ms: Some(18),
            state: LatencyState::Healthy,
            target: LatencyTarget::Internet,
            sequence: 3,
            sampled_at_unix: 1700000002,
            jitter_ms: Some(2),
            protocol: Some(IpProtocol::Ipv6),
            packet_loss_pct: Some(5),
            pings_lost: 1,
            pings_total: 20,
            health: LatencyHealth::Degraded,
        };
        assert_eq!(
            with_dual_stack_and_loss.detailed_display_text(),
            "18 ms (±2 ms) · 5% loss · Internet (IPv6)"
        );

        let timeout = LatencySnapshot {
            latency_ms: None,
            state: LatencyState::Timeout,
            target: LatencyTarget::Gateway,
            sequence: 4,
            sampled_at_unix: 1700000003,
            jitter_ms: None,
            protocol: Some(IpProtocol::Ipv4),
            packet_loss_pct: Some(100),
            pings_lost: 20,
            pings_total: 20,
            health: LatencyHealth::Timeout,
        };
        assert_eq!(timeout.display_text(), "Timeout · 100% loss");
        assert_eq!(
            timeout.detailed_display_text(),
            "Timeout · 100% loss · Gateway (IPv4)"
        );

        let unavailable = LatencySnapshot::default();
        assert_eq!(unavailable.display_text(), "-- ms");
        assert_eq!(unavailable.detailed_display_text(), "-- ms");
    }

    #[test]
    fn test_wifi_generation_pure_mapping() {
        assert_eq!(wifi_generation(7), WifiGeneration::Wifi4);
        assert_eq!(wifi_generation(8), WifiGeneration::Wifi5);
        assert_eq!(wifi_generation(10), WifiGeneration::Wifi6);
        assert_eq!(wifi_generation(11), WifiGeneration::Wifi7);
        assert_eq!(wifi_generation(4), WifiGeneration::Legacy);
        assert_eq!(wifi_generation(9), WifiGeneration::Legacy);
        assert_eq!(wifi_generation(99), WifiGeneration::Unknown);
        assert_eq!(WifiGeneration::Wifi7.as_str(), "Wi-Fi 7");
        assert_eq!(WifiGeneration::Wifi6.as_str(), "Wi-Fi 6");
    }

    #[test]
    fn test_signal_quality_to_rssi_dbm() {
        assert_eq!(signal_quality_to_rssi_dbm(0), -100);
        assert_eq!(signal_quality_to_rssi_dbm(50), -75);
        assert_eq!(signal_quality_to_rssi_dbm(100), -50);
        assert_eq!(signal_quality_to_rssi_dbm(80), -60);
    }

    #[test]
    fn test_wifi_band_resolution() {
        assert_eq!(wifi_band(2412, None), WifiBand::Band24Ghz);
        assert_eq!(wifi_band(5180, None), WifiBand::Band5Ghz);
        assert_eq!(wifi_band(6100, None), WifiBand::Band6Ghz);
        assert_eq!(wifi_band(0, Some(6)), WifiBand::Band24Ghz);
        assert_eq!(wifi_band(0, Some(36)), WifiBand::Band5Ghz);
        assert_eq!(wifi_band(0, None), WifiBand::Unknown);
        assert_eq!(WifiBand::Band24Ghz.as_str(), "2.4 GHz");
        assert_eq!(WifiBand::Band5Ghz.as_str(), "5 GHz");
        assert_eq!(WifiBand::Band6Ghz.as_str(), "6 GHz");
    }

    #[test]
    fn test_packet_loss_tracker_semantics_and_rollover() {
        let mut tracker = PacketLossTracker::default();
        assert_eq!(tracker.loss_pct(), None);
        assert_eq!(tracker.health(), LatencyHealth::Unavailable);

        // 1. Successes only
        for _ in 0..10 {
            tracker.record(ProbeResult::Success {
                latency: Duration::from_millis(15),
            });
        }
        assert_eq!(tracker.loss_pct(), Some(0));
        assert_eq!(tracker.health(), LatencyHealth::Healthy);
        assert_eq!(tracker.counts(), (0, 10));

        // 2. Unavailable probes MUST NOT count towards packet loss
        for _ in 0..5 {
            tracker.record(ProbeResult::Unavailable);
        }
        // Total valid pings remains 10, lost remains 0, so 0% loss
        assert_eq!(tracker.loss_pct(), Some(0));
        assert_eq!(tracker.health(), LatencyHealth::Healthy);
        assert_eq!(tracker.counts(), (0, 10));

        // 3. One Timeout -> degraded
        tracker.record(ProbeResult::Timeout);
        assert_eq!(tracker.loss_pct(), Some(9)); // 1/11 = 9%
        assert_eq!(tracker.health(), LatencyHealth::Degraded);
        assert_eq!(tracker.counts(), (1, 11));

        // 4. Overwrite earlier samples with 19 successes so buffer is 19 success + 1 timeout
        for _ in 0..19 {
            tracker.record(ProbeResult::Success {
                latency: Duration::from_millis(18),
            });
        }
        assert_eq!(tracker.counts(), (1, 20));
        assert_eq!(tracker.loss_pct(), Some(5)); // 1/20 = 5%
        assert_eq!(tracker.health(), LatencyHealth::Degraded);

        // 5. Test rollover: push 20 timeouts
        for _ in 0..20 {
            tracker.record(ProbeResult::Timeout);
        }
        assert_eq!(tracker.counts(), (20, 20));
        assert_eq!(tracker.loss_pct(), Some(100));
        assert_eq!(tracker.health(), LatencyHealth::Timeout);
    }

    #[test]
    fn test_physical_link_summary_formatting() {
        let wifi = PhysicalLinkInfo::Wifi(WifiPhyMetrics {
            ssid: "HomeMesh".to_string(),
            generation: WifiGeneration::Wifi6,
            band: WifiBand::Band5Ghz,
            channel: Some(36),
            signal_quality_pct: 84,
            rssi_dbm: Some(-58),
            tx_rate_mbps: Some(866),
            rx_rate_mbps: Some(866),
            is_mlo: false,
            link_count: 1,
        });
        assert_eq!(
            wifi.display_summary(),
            "Wi-Fi 6 · 5 GHz · -58 dBm · 866 Mbps"
        );

        let eth_1g = PhysicalLinkInfo::Ethernet(EthernetLinkMetrics {
            adapter_name: "Ethernet".to_string(),
            tx_speed_bps: 1_000_000_000,
            rx_speed_bps: 1_000_000_000,
        });
        assert_eq!(eth_1g.display_summary(), "Ethernet · 1 Gbps");

        let eth_2_5g = PhysicalLinkInfo::Ethernet(EthernetLinkMetrics {
            adapter_name: "Ethernet 2".to_string(),
            tx_speed_bps: 2_500_000_000,
            rx_speed_bps: 2_500_000_000,
        });
        assert_eq!(eth_2_5g.display_summary(), "Ethernet · 2.5 Gbps");

        let eth_100m = PhysicalLinkInfo::Ethernet(EthernetLinkMetrics {
            adapter_name: "Ethernet".to_string(),
            tx_speed_bps: 100_000_000,
            rx_speed_bps: 100_000_000,
        });
        assert_eq!(eth_100m.display_summary(), "Ethernet · 100 Mbps");
    }
}
