use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

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

/// A single point in the rolling history buffer.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct HistorySample {
    pub rx_bps: u64,
    pub tx_bps: u64,
}

/// Persisted session state across widget restarts.
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

/// Complete snapshot returned by `NetworkBackend::sample()`.
/// Contains everything the presentation layer needs.
#[derive(Debug, Clone)]
pub struct NetworkSnapshot {
    pub rx_bps: f64,
    pub tx_bps: f64,
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
    /// Compute the peak rx and tx bandwidth across the most recent `sample_count` samples in history.
    /// Incorporates the current live `rx_bps` / `tx_bps` so the peak is strictly >= current rate.
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

pub struct NetworkBackend {
    pub mode: AggregateMode,
    /// Baseline counters for EVERY interface seen, regardless of mode/status.
    /// This prevents false rate spikes when an interface cycles Down → Up.
    prev_counters: HashMap<InterfaceLuid, (u64, u64)>,
    prev_time: Option<Instant>,
    pub peak_rx: f64,
    pub peak_tx: f64,
    pub session_rx: u64,
    pub session_tx: u64,
    pub session_start_unix: u64,
    /// Rolling FIFO history of aggregate rates.
    history: VecDeque<HistorySample>,
    /// State tracker for per-process realtime bandwidth and connection counts.
    pub process_tracker: crate::process::ProcessTracker,
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
            process_tracker: crate::process::ProcessTracker::new(),
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
            process_tracker: crate::process::ProcessTracker::new(),
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
        self.process_tracker.reset();
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
        let (mut active_apps, active_conns) = self.process_tracker.sample(now);
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
        let elapsed_secs = match self.prev_time {
            Some(prev) => {
                let duration = now.saturating_duration_since(prev).as_secs_f64();
                if duration > 0.0 { duration } else { 0.0 }
            }
            None => 0.0, // First sample establishes baseline
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

        let rx_bps = if elapsed_secs > 0.0 {
            total_delta_in as f64 / elapsed_secs
        } else {
            0.0
        };
        let tx_bps = if elapsed_secs > 0.0 {
            total_delta_out as f64 / elapsed_secs
        } else {
            0.0
        };

        if rx_bps > self.peak_rx {
            self.peak_rx = rx_bps;
        }
        if tx_bps > self.peak_tx {
            self.peak_tx = tx_bps;
        }

        // Time cadence synchronization:
        // Ensure each history slot strictly represents SAMPLING_INTERVAL_MS (500ms).
        // Only record timed intervals. The first sample after start/reset
        // is a baseline (elapsed = 0) and must not consume a chart slot.
        let sampling_sec = crate::SAMPLING_INTERVAL_MS as f64 / 1000.0;
        let intervals = if elapsed_secs > 0.0 {
            (elapsed_secs / sampling_sec).round() as usize
        } else {
            0
        };

        let sample_point = HistorySample {
            rx_bps: rx_bps as u64,
            tx_bps: tx_bps as u64,
        };

        if intervals >= HISTORY_CAPACITY {
            // Gap exceeded maximum capacity (e.g. PC suspended/slept for >60s).
            // Stale history is purged so pre-sleep peaks do not linger on the live chart.
            self.history.clear();
            self.history.push_back(sample_point);
        } else if intervals > 1 {
            // Gap was between 1.0s and 60.0s (e.g. slight delay or pause).
            // Advance history by the actual number of elapsed intervals so 1 slot always equals SAMPLING_INTERVAL_MS.
            for _ in 0..intervals {
                if self.history.len() >= HISTORY_CAPACITY {
                    self.history.pop_front();
                }
                self.history.push_back(sample_point);
            }
        } else if elapsed_secs > 0.0 {
            if self.history.len() >= HISTORY_CAPACITY {
                self.history.pop_front();
            }
            self.history.push_back(sample_point);
        }

        let now_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let session_duration_secs = now_unix.saturating_sub(self.session_start_unix);

        // Compute active chart peak (across current history)
        let mut chart_peak_rx = rx_bps;
        let mut chart_peak_tx = tx_bps;
        for s in &self.history {
            if (s.rx_bps as f64) > chart_peak_rx {
                chart_peak_rx = s.rx_bps as f64;
            }
            if (s.tx_bps as f64) > chart_peak_tx {
                chart_peak_tx = s.tx_bps as f64;
            }
        }

        NetworkSnapshot {
            rx_bps,
            tx_bps,
            session_rx: self.session_rx,
            session_tx: self.session_tx,
            session_duration_secs,
            active_interfaces: reported_active_count,
            peak_rx_bps: chart_peak_rx,
            peak_tx_bps: chart_peak_tx,
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

/// Query the active Wi-Fi SSID connected on Windows via WlanAPI.
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
                            if let Ok(ssid_str) = std::str::from_utf8(bytes) {
                                let trimmed = ssid_str.trim();
                                if !trimmed.is_empty() {
                                    found_ssid = Some(trimmed.to_string());
                                }
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

/// Cached lookup of active Wi-Fi SSID with a 2-second TTL to avoid redundant WLAN queries.
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

/// Classify an interface row from MIB_IF_ROW2 into an InterfaceCategory and InterfaceMedium.
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

/// Query system network interfaces using IP Helper API GetIfTable2.
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

        // Second sample 500ms later (1 sampling interval)
        let t1 = t0 + Duration::from_millis(500);
        let ifaces2 = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Wifi,
            1,
            1000 + 2500,
            2000 + 1000,
        )];
        let snap2 = backend.sample_from_interfaces(&ifaces2, t1);
        assert_eq!(snap2.rx_bps, 5000.0);
        assert_eq!(snap2.tx_bps, 2000.0);
        assert_eq!(snap2.session_rx, 2500);
        assert_eq!(snap2.session_tx, 1000);
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

        // Generate 140 samples at 500ms intervals (70s total, exceeds capacity of 120 / 60s)
        for i in 1..=140u64 {
            let t = t0 + Duration::from_millis(i * 500);
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
        assert_eq!(HISTORY_CAPACITY, 120);
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

        // Second sample after reset (500ms cadence) computes real delta from new baseline
        let t3 = t2 + Duration::from_millis(500);
        let ifaces4 = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Wifi,
            1,
            6500,
            6250,
        )];
        let snap_after = backend.sample_from_interfaces(&ifaces4, t3);
        assert_eq!(snap_after.rx_bps, 1000.0);
        assert_eq!(snap_after.tx_bps, 500.0);
        assert_eq!(snap_after.session_rx, 500);
        assert_eq!(snap_after.session_tx, 250);
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

        // Advance 3s (6 intervals of 500ms)
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
        // History should contain 6 samples reflecting the 3s elapsed span (3.0s / 0.5s = 6)
        assert_eq!(snap.history.len(), 6);
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

        // Push some initial samples at 500ms intervals
        for i in 1..=5u64 {
            let t = t0 + Duration::from_millis(i * 500);
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

        // Simulate 120-second suspend/sleep gap (> MAX_CHART_WINDOW_SECS of 60s)
        let t_sleep = t0 + Duration::from_millis(5 * 500) + Duration::from_secs(120);
        let ifaces_wake = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            100_000,
            50_000,
        )];
        let snap_wake = backend.sample_from_interfaces(&ifaces_wake, t_sleep);
        // Old stale samples from before sleep must be cleared!
        assert_eq!(snap_wake.history.len(), 1);
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
}
