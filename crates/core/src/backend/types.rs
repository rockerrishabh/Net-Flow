use serde::{Deserialize, Serialize};
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

impl std::fmt::Display for LatencyTargetMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str_value())
    }
}

impl std::str::FromStr for LatencyTargetMode {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_str_value(s))
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

/// Wi-Fi generation / standard mapped from 802.11 PHY types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WifiGeneration {
    #[default]
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "wifi_4")]
    Wifi4,
    #[serde(rename = "wifi_5")]
    Wifi5,
    #[serde(rename = "wifi_6")]
    Wifi6,
    #[serde(rename = "wifi_6e")]
    Wifi6E,
    #[serde(rename = "wifi_7")]
    Wifi7,
}

impl WifiGeneration {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Legacy => "Wi-Fi (Legacy)",
            Self::Wifi4 => "Wi-Fi 4",
            Self::Wifi5 => "Wi-Fi 5",
            Self::Wifi6 => "Wi-Fi 6",
            Self::Wifi6E => "Wi-Fi 6E",
            Self::Wifi7 => "Wi-Fi 7",
            Self::Unknown => "Wi-Fi",
        }
    }

    pub fn as_snake_case(&self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Legacy => "legacy",
            Self::Wifi4 => "wifi_4",
            Self::Wifi5 => "wifi_5",
            Self::Wifi6 => "wifi_6",
            Self::Wifi6E => "wifi_6e",
            Self::Wifi7 => "wifi_7",
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

/// Identifies Wi-Fi generation considering both 802.11 PHY type and operating frequency band.
/// Wi-Fi 6E is 802.11ax operating in the 6 GHz band (5925–7125 MHz).
pub fn wifi_generation_with_band(phy: u32, band: WifiBand) -> WifiGeneration {
    let generation = wifi_generation(phy);
    if generation == WifiGeneration::Wifi6 && band == WifiBand::Band6Ghz {
        WifiGeneration::Wifi6E
    } else {
        generation
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
