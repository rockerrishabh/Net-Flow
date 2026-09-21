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

pub mod backend;
pub mod card;
pub mod chart;
pub mod format;
pub mod icons;
pub mod process;

pub use backend::{
    AggregateMode, HISTORY_CAPACITY, HistorySample, InterfaceCategory, InterfaceInfo,
    InterfaceLuid, InterfaceSample, NetworkBackend, NetworkSnapshot, classify_interface,
    compute_delta, query_interfaces,
};
pub use card::{
    BURST_THRESHOLD_BPS, MODERATE_THRESHOLD_BPS, WidgetConfig, build_adaptive_card,
    build_adaptive_card_data, build_adaptive_card_data_string, build_adaptive_card_template,
    build_settings_card, build_settings_card_for_size, compute_adaptive_ui_interval,
};
pub use chart::{
    GraphStyle, Palette, ResolvedTheme, ThemeMode, Track, detect_windows_light_theme,
    query_windows_light_theme, render_chart_data_uris, render_chart_png,
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
