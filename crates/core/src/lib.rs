//! Core network monitoring engine, Adaptive Card templating, and chart rendering
//! for the Net-Flow Windows 11 widget.

/// Sampling period for network interface polling (4 samples per second).
pub const SAMPLING_INTERVAL_MS: u64 = 250;
/// Cadence for pushing updated Adaptive Cards to the Windows Widget Board.
pub const UPDATE_INTERVAL_MS: u64 = 500;
/// Maximum rolling history buffer duration in seconds.
pub const MAX_CHART_WINDOW_SECS: u32 = 60;

/// Calculates how many telemetry samples fit into a given chart window duration.
pub fn history_samples_for_secs(secs: u32) -> usize {
    let secs = u64::from(secs.max(1));
    ((secs * 1000) / SAMPLING_INTERVAL_MS) as usize
}

/// Validates and snaps user-selected chart windows to supported presets (15s, 30s, or 60s).
pub fn clamp_chart_window(secs: u32) -> u32 {
    match secs {
        15 | 60 => secs,
        _ => 30,
    }
}

pub mod alerts;
pub mod backend;
pub mod budget;
pub mod card;
pub mod chart;
pub mod daily_usage;
pub mod format;
pub mod icons;
pub mod process;

pub use alerts::{
    AlertDirection, AlertEvent, BandwidthAlertConfig, BandwidthAlertEngine, get_alert_config_path,
    load_alert_config, save_alert_config,
};
pub use backend::{
    AggregateMode, EthernetLinkMetrics, HISTORY_CAPACITY, HistorySample, InterfaceCategory,
    InterfaceCounterState, InterfaceInfo, InterfaceLuid, InterfaceSample, IpProtocol, Ipv6Route,
    LatencyHealth, LatencySnapshot, LatencyState, LatencyTarget, LatencyTargetMode, NetworkBackend,
    NetworkSnapshot, PacketLossTracker, PhysicalLinkInfo, ProbeResult, SessionState, WifiBand,
    WifiGeneration, WifiPhyMetrics, classify_interface, compute_delta, counter_delta,
    load_persisted_session_state, probe_latency, probe_latency_ipv4, probe_latency_ipv6,
    query_active_ethernet_metrics, query_active_wifi_metrics, query_cached_wifi_phy,
    query_interfaces, query_ipv4_gateway_address, query_ipv6_route, query_physical_link_info,
    sample_latency_snapshot, sample_latency_snapshot_dual_stack, save_persisted_session_state,
    signal_quality_to_rssi_dbm, wifi_band, wifi_generation,
};
pub use budget::{
    BudgetMilestone, BudgetScope, BudgetSnapshot, DataBudgetConfig, MilestoneTier,
    budget_bar_style, budget_progress_width, calculate_budget_snapshot, calculate_cycle_boundaries,
    clamped_renewal_day, current_local_ymd, days_in_month, format_budget_headline,
    format_budget_medium, format_ymd, is_leap_year,
};
pub use card::{
    BURST_THRESHOLD_BPS, MODERATE_THRESHOLD_BPS, WidgetConfig, build_adaptive_card,
    build_adaptive_card_data, build_adaptive_card_data_string, build_adaptive_card_template,
    build_settings_card, build_settings_card_for_size, compute_adaptive_ui_interval,
    get_user_config_path, load_user_config, save_user_config,
};
pub use chart::{
    GraphStyle, Palette, ResolvedTheme, ThemeMode, Track, detect_windows_light_theme,
    query_windows_light_theme, render_chart_data_uris, render_chart_png,
};
pub use daily_usage::{
    DailyUsageEntry, DailyUsageStore, MAX_DAILY_ENTRIES, MilestoneState, get_daily_usage_path,
    load_daily_usage, save_daily_usage,
};
pub use format::{SpeedUnit, format_bandwidth, format_bandwidth_with_unit, format_bytes};
pub use icons::app_glyph_for_emoji;
pub use process::{ActiveAppInfo, ProcessTracker, map_process_to_app, query_active_apps};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_capacity_matches_sampling_cadence() {
        assert_eq!(
            history_samples_for_secs(MAX_CHART_WINDOW_SECS),
            HISTORY_CAPACITY
        );
        assert_eq!(HISTORY_CAPACITY, 240);
    }
}
