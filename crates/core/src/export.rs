//! Pure-Rust diagnostic telemetry data models, schema serialization, and exporters.
//!
//! Provides the canonical `DiagnosticsSnapshot` model (schema_version = 1) along with
//! high-fidelity JSON export and the Net Flow CSV dialect (`netflow-diagnostics-v1`).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::budget::days_in_month;
use crate::daily_usage::DailyUsageEntry;
use crate::format::format_bytes;

/// Active schema version for the canonical diagnostic export format.
pub const DIAGNOSTICS_SCHEMA_VERSION: u32 = 1;

/// Dialect identifier embedded in Net Flow CSV export headers.
pub const CSV_DIALECT_IDENTIFIER: &str = "netflow-diagnostics-v1";

/// CPU architecture of the binary generating the diagnostic report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    X64,
    Arm64,
    Unknown,
}

impl Architecture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::X64 => "x64",
            Self::Arm64 => "arm64",
            Self::Unknown => "unknown",
        }
    }
}

/// Broad network interface categorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterfaceType {
    Ethernet,
    Wifi,
    Cellular,
    Vpn,
    Loopback,
    Other,
}

impl InterfaceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ethernet => "ethernet",
            Self::Wifi => "wifi",
            Self::Cellular => "cellular",
            Self::Vpn => "vpn",
            Self::Loopback => "loopback",
            Self::Other => "other",
        }
    }
}

/// Semantic connection quality states evaluated across packet loss and latency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticHealth {
    Healthy,
    Degraded,
    Timeout,
    Unavailable,
}

impl SemanticHealth {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Timeout => "timeout",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Wi-Fi generation classification derived from 802.11 PHY indicators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WifiGeneration {
    #[default]
    #[serde(rename = "unknown")]
    Unknown,
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
            Self::Unknown => "unknown",
            Self::Wifi4 => "wifi_4",
            Self::Wifi5 => "wifi_5",
            Self::Wifi6 => "wifi_6",
            Self::Wifi6E => "wifi_6e",
            Self::Wifi7 => "wifi_7",
        }
    }
}

/// Structured machine-readable privacy disclosure flags.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacyMetadata {
    pub contains_network_identifiers: bool,
    pub contains_wifi_identifiers: bool,
    pub contains_process_names: bool,
}

impl Default for PrivacyMetadata {
    fn default() -> Self {
        Self {
            contains_network_identifiers: true,
            contains_wifi_identifiers: true,
            contains_process_names: true,
        }
    }
}

/// Timezone and calendar coverage metadata for the historical usage series.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryMetadata {
    pub timezone: String,
    pub start_date: String,
    pub end_date: String,
    pub days: u32,
}

/// Realtime physical and Wi-Fi RF parameters for wireless adapters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WifiDiagnostics {
    pub ssid: String,
    pub bssid: String,
    pub generation: WifiGeneration,
    pub band_ghz: String,
    pub channel: u32,
    pub channel_width_mhz: u32,
    pub rssi_dbm: i32,
    pub transmit_rate_bps: u64,
    pub receive_rate_bps: u64,
}

/// Deep network adapter diagnostics gathered from IP Helper and WLAN APIs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterDiagnostics {
    pub name: String,
    pub description: String,
    pub friendly_name: String,
    pub interface_type: InterfaceType,
    pub mac_address: String,
    pub ipv4_addresses: Vec<String>,
    pub ipv6_addresses: Vec<String>,
    pub default_gateways: Vec<String>,
    pub dns_servers: Vec<String>,
    pub dhcp_enabled: bool,
    pub dhcp_server: Option<String>,
    pub mtu: u32,
    pub link_speed_bps: u64,
    pub wifi: Option<WifiDiagnostics>,
}

/// Brief summary of secondary or non-active network adapters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterSummary {
    pub name: String,
    pub friendly_name: String,
    pub interface_type: InterfaceType,
    pub is_up: bool,
    pub ipv4_addresses: Vec<String>,
}

/// Active session bandwidth accumulation and quota tracking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionDiagnostics {
    pub session_start_utc: String,
    pub duration_seconds: u64,
    pub bytes_downloaded: u64,
    pub bytes_uploaded: u64,
    pub bytes_total: u64,
    pub peak_download_bps: u64,
    pub peak_upload_bps: u64,
    pub budget_cap_bytes: Option<u64>,
    pub budget_consumed_bytes: u64,
    pub budget_usage_pct: u16,
    pub budget_days_remaining: u32,
}

/// Round-trip latency, interarrival jitter, and rolling packet loss diagnostics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityDiagnostics {
    pub target_host: String,
    pub rtt_ms: f64,
    pub rtt_jitter_ms: f64,
    pub jitter_method: String,
    pub sample_count: usize,
    pub packets_received: usize,
    pub packets_lost: usize,
    pub packet_loss_percent: f64,
    pub semantic_health: SemanticHealth,
}

/// Chronological daily byte total record for one calendar day.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyUsageRecord {
    pub date: String,
    pub download_bytes: u64,
    pub upload_bytes: u64,
    pub total_bytes: u64,
}

/// Realtime active application network attribution record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessAttributionRecord {
    pub process_name: String,
    pub download_bps: u64,
    pub upload_bps: u64,
    pub socket_count: u32,
}

/// Canonical root diagnostic snapshot model (`schema_version = 1`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticsSnapshot {
    pub schema_version: u32,
    pub app_version: String,
    pub architecture: Architecture,
    pub snapshot_timestamp_utc: String,
    pub export_generated_at_utc: String,
    pub history_metadata: HistoryMetadata,
    pub privacy: PrivacyMetadata,
    pub active_adapter: Option<AdapterDiagnostics>,
    pub other_adapters: Vec<AdapterSummary>,
    pub session: SessionDiagnostics,
    pub quality: QualityDiagnostics,
    pub history: Vec<DailyUsageRecord>,
    pub processes: Vec<ProcessAttributionRecord>,
}

/// Calculates mean absolute RTT difference jitter across an array of latency samples.
///
/// $$\text{Jitter} = \frac{1}{N - 1} \sum_{i=1}^{N - 1} |\text{RTT}_i - \text{RTT}_{i-1}|$$
pub fn calculate_mean_absolute_rtt_difference(samples: &[f64]) -> f64 {
    if samples.len() < 2 {
        return 0.0;
    }
    let mut sum_diff = 0.0;
    for i in 1..samples.len() {
        sum_diff += (samples[i] - samples[i - 1]).abs();
    }
    let jitter = sum_diff / (samples.len() - 1) as f64;
    (jitter * 100.0).round() / 100.0
}

/// Computes the previous calendar date in "YYYY-MM-DD" format.
fn previous_date(year: i32, month: u32, day: u32) -> (i32, u32, u32) {
    if day > 1 {
        (year, month, day - 1)
    } else if month > 1 {
        let prev_month = month - 1;
        let prev_days = days_in_month(year, prev_month);
        (year, prev_month, prev_days)
    } else {
        (year - 1, 12, 31)
    }
}

/// Parses "YYYY-MM-DD" into numeric components (year, month, day).
fn parse_date_str(s: &str) -> Option<(i32, u32, u32)> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return None;
    }
    let y: i32 = parts[0].parse().ok()?;
    let m: u32 = parts[1].parse().ok()?;
    let d: u32 = parts[2].parse().ok()?;
    Some((y, m, d))
}

/// Pads a 90-day daily usage history window with zero-byte records for missing dates.
///
/// Returns exactly `count` continuous chronological records ending on `end_date`.
pub fn zero_fill_history(
    entries: &[DailyUsageEntry],
    end_date: &str,
    count: usize,
) -> (Vec<DailyUsageRecord>, String) {
    let mut map: HashMap<&str, (u64, u64)> = HashMap::with_capacity(entries.len());
    for entry in entries {
        map.insert(entry.date.as_str(), (entry.rx_bytes, entry.tx_bytes));
    }

    let mut records = Vec::with_capacity(count);
    let (mut y, mut m, mut d) = parse_date_str(end_date).unwrap_or((2026, 1, 1));

    // Generate dates working backwards from end_date
    let mut dates_rev = Vec::with_capacity(count);
    for _ in 0..count {
        dates_rev.push(format!("{y:04}-{m:02}-{d:02}"));
        let (py, pm, pd) = previous_date(y, m, d);
        y = py;
        m = pm;
        d = pd;
    }

    // Earliest start date is the last element of dates_rev
    let start_date = dates_rev
        .last()
        .cloned()
        .unwrap_or_else(|| end_date.to_string());

    // Reverse to chronological order (earliest -> newest)
    for date in dates_rev.into_iter().rev() {
        let (rx, tx) = map.get(date.as_str()).copied().unwrap_or((0, 0));
        records.push(DailyUsageRecord {
            date,
            download_bytes: rx,
            upload_bytes: tx,
            total_bytes: rx.saturating_add(tx),
        });
    }

    (records, start_date)
}

/// Serializes a `DiagnosticsSnapshot` into canonical JSON format.
pub fn export_to_json(
    snapshot: &DiagnosticsSnapshot,
    pretty: bool,
) -> Result<String, serde_json::Error> {
    if pretty {
        serde_json::to_string_pretty(snapshot)
    } else {
        serde_json::to_string(snapshot)
    }
}

/// Quotes a CSV field value per RFC 4180 rules if it contains commas, quotes, or newlines.
fn rfc4180_escape(val: &str) -> String {
    if val.contains(',') || val.contains('"') || val.contains('\n') || val.contains('\r') {
        format!("\"{}\"", val.replace('"', "\"\""))
    } else {
        val.to_string()
    }
}

/// Formats a `DiagnosticsSnapshot` into the Net Flow CSV dialect (`netflow-diagnostics-v1`).
pub fn export_to_csv(snapshot: &DiagnosticsSnapshot) -> String {
    let mut out = String::with_capacity(4096);

    // Comment-prefixed metadata headers
    out.push_str(&format!("# CSV Dialect: {CSV_DIALECT_IDENTIFIER}\r\n"));
    out.push_str(&format!(
        "# Net Flow Diagnostic Report - schema_version: {}\r\n",
        snapshot.schema_version
    ));
    out.push_str(&format!(
        "# Snapshot UTC: {} | Exported UTC: {}\r\n",
        snapshot.snapshot_timestamp_utc, snapshot.export_generated_at_utc
    ));
    out.push_str(&format!(
        "# App Version: {} | Architecture: {} | Timezone: {}\r\n",
        snapshot.app_version,
        snapshot.architecture.as_str(),
        snapshot.history_metadata.timezone
    ));
    out.push_str(&format!(
        "# Privacy: Network Identifiers = {}, Wi-Fi Identifiers = {}, Process Names = {}\r\n",
        snapshot.privacy.contains_network_identifiers,
        snapshot.privacy.contains_wifi_identifiers,
        snapshot.privacy.contains_process_names
    ));

    if let Some(adapter) = &snapshot.active_adapter {
        let link_mbps = (adapter.link_speed_bps / 1_000_000).max(1);
        let wifi_suffix = if let Some(w) = &adapter.wifi {
            format!(" | SSID: {} | RSSI: {} dBm", w.ssid, w.rssi_dbm)
        } else {
            String::new()
        };
        out.push_str(&format!(
            "# Active Adapter: {} ({}) | Type: {} | Link: {} Mbps{}\r\n",
            adapter.friendly_name,
            adapter.description,
            adapter.interface_type.as_str(),
            link_mbps,
            wifi_suffix
        ));
        out.push_str(&format!(
            "# IP: {} | Gateway: {} | DNS: {}\r\n",
            adapter.ipv4_addresses.join(", "),
            adapter.default_gateways.join(", "),
            adapter.dns_servers.join(", ")
        ));
    } else {
        out.push_str("# Active Adapter: None (Offline / No default gateway)\r\n");
    }

    out.push_str(&format!(
        "# Quality: Target = {} | RTT = {:.1} ms | Jitter = {:.1} ms ({}) | Loss = {:.1}% ({}/{})\r\n",
        snapshot.quality.target_host,
        snapshot.quality.rtt_ms,
        snapshot.quality.rtt_jitter_ms,
        snapshot.quality.jitter_method,
        snapshot.quality.packet_loss_percent,
        snapshot.quality.packets_received,
        snapshot.quality.sample_count
    ));

    let hours = snapshot.session.duration_seconds / 3600;
    let mins = (snapshot.session.duration_seconds % 3600) / 60;
    out.push_str(&format!(
        "# Session: {}h {}m | Download: {} | Upload: {} | Total: {}\r\n",
        hours,
        mins,
        format_bytes(snapshot.session.bytes_downloaded),
        format_bytes(snapshot.session.bytes_uploaded),
        format_bytes(snapshot.session.bytes_total)
    ));

    // Tabular Section: RFC 4180 Table
    out.push_str(
        "Date,Download_Bytes,Upload_Bytes,Total_Bytes,Download_Human,Upload_Human,Total_Human\r\n",
    );

    for record in &snapshot.history {
        let dl_human = rfc4180_escape(&format_bytes(record.download_bytes));
        let ul_human = rfc4180_escape(&format_bytes(record.upload_bytes));
        let total_human = rfc4180_escape(&format_bytes(record.total_bytes));

        out.push_str(&format!(
            "{},{},{},{},{},{},{}\r\n",
            record.date,
            record.download_bytes,
            record.upload_bytes,
            record.total_bytes,
            dl_human,
            ul_human,
            total_human
        ));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_snapshot() -> DiagnosticsSnapshot {
        DiagnosticsSnapshot {
            schema_version: DIAGNOSTICS_SCHEMA_VERSION,
            app_version: "0.9.0".to_string(),
            architecture: Architecture::X64,
            snapshot_timestamp_utc: "2026-10-03T08:30:00Z".to_string(),
            export_generated_at_utc: "2026-10-03T08:30:05Z".to_string(),
            history_metadata: HistoryMetadata {
                timezone: "Asia/Kolkata".to_string(),
                start_date: "2026-07-06".to_string(),
                end_date: "2026-10-03".to_string(),
                days: 90,
            },
            privacy: PrivacyMetadata::default(),
            active_adapter: Some(AdapterDiagnostics {
                name: "{B91E3B87-4340-4D2A-A6C5-F329598EC001}".to_string(),
                description: "Intel(R) Wi-Fi 6E AX211 160MHz".to_string(),
                friendly_name: "Wi-Fi".to_string(),
                interface_type: InterfaceType::Wifi,
                mac_address: "AA:BB:CC:DD:EE:FF".to_string(),
                ipv4_addresses: vec!["192.168.1.105/24".to_string()],
                ipv6_addresses: vec!["fe80::1/64".to_string()],
                default_gateways: vec!["192.168.1.1".to_string()],
                dns_servers: vec!["1.1.1.1".to_string(), "1.0.0.1".to_string()],
                dhcp_enabled: true,
                dhcp_server: Some("192.168.1.1".to_string()),
                mtu: 1500,
                link_speed_bps: 1_200_000_000,
                wifi: Some(WifiDiagnostics {
                    ssid: "Home-5G".to_string(),
                    bssid: "00:11:22:33:44:55".to_string(),
                    generation: WifiGeneration::Wifi6E,
                    band_ghz: "5 GHz".to_string(),
                    channel: 36,
                    channel_width_mhz: 160,
                    rssi_dbm: -54,
                    transmit_rate_bps: 1_200_000_000,
                    receive_rate_bps: 1_200_000_000,
                }),
            }),
            other_adapters: vec![AdapterSummary {
                name: "{78E29840-0012-421A-9112-C89123891002}".to_string(),
                friendly_name: "Ethernet".to_string(),
                interface_type: InterfaceType::Ethernet,
                is_up: false,
                ipv4_addresses: vec![],
            }],
            session: SessionDiagnostics {
                session_start_utc: "2026-10-03T05:00:00Z".to_string(),
                duration_seconds: 12600,
                bytes_downloaded: 4_294_967_296,
                bytes_uploaded: 1_073_741_824,
                bytes_total: 5_368_709_120,
                peak_download_bps: 45_000_000,
                peak_upload_bps: 12_000_000,
                budget_cap_bytes: Some(100_000_000_000),
                budget_consumed_bytes: 45_000_000_000,
                budget_usage_pct: 45,
                budget_days_remaining: 18,
            },
            quality: QualityDiagnostics {
                target_host: "1.1.1.1".to_string(),
                rtt_ms: 18.2,
                rtt_jitter_ms: 1.8,
                jitter_method: "mean_absolute_rtt_difference".to_string(),
                sample_count: 20,
                packets_received: 20,
                packets_lost: 0,
                packet_loss_percent: 0.0,
                semantic_health: SemanticHealth::Healthy,
            },
            history: vec![
                DailyUsageRecord {
                    date: "2026-10-02".to_string(),
                    download_bytes: 3_000_000_000,
                    upload_bytes: 500_000_000,
                    total_bytes: 3_500_000_000,
                },
                DailyUsageRecord {
                    date: "2026-10-03".to_string(),
                    download_bytes: 4_294_967_296,
                    upload_bytes: 1_073_741_824,
                    total_bytes: 5_368_709_120,
                },
            ],
            processes: vec![ProcessAttributionRecord {
                process_name: "msedge.exe".to_string(),
                download_bps: 15_000_000,
                upload_bps: 1_200_000,
                socket_count: 14,
            }],
        }
    }

    #[test]
    fn test_json_round_trip() {
        let snap = sample_snapshot();
        let json = export_to_json(&snap, false).expect("JSON serialization failed");
        let parsed: DiagnosticsSnapshot =
            serde_json::from_str(&json).expect("JSON deserialization failed");
        assert_eq!(snap, parsed);
    }

    #[test]
    fn test_schema_version_is_one() {
        let snap = sample_snapshot();
        assert_eq!(snap.schema_version, 1);
        let json = export_to_json(&snap, false).unwrap();
        assert!(json.contains("\"schema_version\":1"));
    }

    #[test]
    fn test_enum_serde_representation() {
        let snap = sample_snapshot();
        let json = export_to_json(&snap, false).unwrap();
        assert!(json.contains("\"architecture\":\"x64\""));
        assert!(json.contains("\"interface_type\":\"wifi\""));
        assert!(json.contains("\"generation\":\"wifi_6e\""));
        assert!(json.contains("\"semantic_health\":\"healthy\""));
    }

    #[test]
    fn test_csv_dialect_header() {
        let snap = sample_snapshot();
        let csv = export_to_csv(&snap);
        assert!(csv.starts_with("# CSV Dialect: netflow-diagnostics-v1\r\n"));
        assert!(csv.contains("# Net Flow Diagnostic Report - schema_version: 1\r\n"));
        assert!(csv.contains("# Privacy: Network Identifiers = true, Wi-Fi Identifiers = true, Process Names = true\r\n"));
    }

    #[test]
    fn test_csv_crlf_row_endings() {
        let snap = sample_snapshot();
        let csv = export_to_csv(&snap);
        for line in csv.split('\n') {
            if !line.is_empty() {
                assert!(line.ends_with('\r'), "Line did not end in CRLF: {line}");
            }
        }
    }

    #[test]
    fn test_csv_quoting_and_escaping() {
        let mut snap = sample_snapshot();
        snap.history = vec![DailyUsageRecord {
            date: "2026-10-03".to_string(),
            download_bytes: 1_000_000,
            upload_bytes: 2_000_000,
            total_bytes: 3_000_000,
        }];
        let csv = export_to_csv(&snap);
        assert!(csv.contains("2026-10-03,1000000,2000000,3000000,"));
        // Escaping test
        assert_eq!(rfc4180_escape("plain_text"), "plain_text");
        assert_eq!(rfc4180_escape("comma,value"), "\"comma,value\"");
        assert_eq!(rfc4180_escape("quote\"value"), "\"quote\"\"value\"");
        assert_eq!(rfc4180_escape("line1\r\nline2"), "\"line1\r\nline2\"");
    }

    #[test]
    fn test_mean_absolute_rtt_difference_jitter() {
        let samples = vec![20.0, 24.0, 18.0, 22.0];
        // differences: |24-20|=4, |18-24|=6, |22-18|=4
        // sum = 14, count = 3, mean = 14 / 3 = 4.666... -> rounded to 4.67
        let jitter = calculate_mean_absolute_rtt_difference(&samples);
        assert_eq!(jitter, 4.67);

        // Under 2 samples
        assert_eq!(calculate_mean_absolute_rtt_difference(&[20.0]), 0.0);
        assert_eq!(calculate_mean_absolute_rtt_difference(&[]), 0.0);
    }

    #[test]
    fn test_history_90_day_zero_fill() {
        let entries = vec![
            DailyUsageEntry {
                date: "2026-10-01".to_string(),
                rx_bytes: 100,
                tx_bytes: 50,
            },
            DailyUsageEntry {
                date: "2026-10-03".to_string(),
                rx_bytes: 500,
                tx_bytes: 200,
            },
        ];

        let (records, start_date) = zero_fill_history(&entries, "2026-10-03", 5);
        assert_eq!(records.len(), 5);
        assert_eq!(records[0].date, "2026-09-29");
        assert_eq!(records[0].total_bytes, 0);

        assert_eq!(records[1].date, "2026-09-30");
        assert_eq!(records[1].total_bytes, 0);

        assert_eq!(records[2].date, "2026-10-01");
        assert_eq!(records[2].download_bytes, 100);
        assert_eq!(records[2].upload_bytes, 50);
        assert_eq!(records[2].total_bytes, 150);

        assert_eq!(records[3].date, "2026-10-02");
        assert_eq!(records[3].total_bytes, 0);

        assert_eq!(records[4].date, "2026-10-03");
        assert_eq!(records[4].download_bytes, 500);
        assert_eq!(records[4].upload_bytes, 200);
        assert_eq!(records[4].total_bytes, 700);

        assert_eq!(start_date, "2026-09-29");
    }

    #[test]
    fn test_no_active_adapter_csv() {
        let mut snap = sample_snapshot();
        snap.active_adapter = None;
        let csv = export_to_csv(&snap);
        assert!(csv.contains("# Active Adapter: None (Offline / No default gateway)\r\n"));
    }
}
