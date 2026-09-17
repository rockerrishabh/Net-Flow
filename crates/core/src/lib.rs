pub const SAMPLING_INTERVAL_MS: u64 = 500;
pub const UPDATE_INTERVAL_MS: u64 = 500;
pub const MAX_CHART_WINDOW_SECS: u32 = 60;

/// How many history samples a chart window of `secs` covers at the sampling cadence.
pub fn history_samples_for_secs(secs: u32) -> usize {
    let secs = u64::from(secs.max(1));
    ((secs * 1000) / SAMPLING_INTERVAL_MS) as usize
}

/// Allowed history windows: 15, 30, or 60 seconds.
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
pub use card::{WidgetConfig, build_adaptive_card, build_settings_card, build_settings_card_for_size};
pub use chart::{Track, render_chart_data_uris, render_chart_png};
pub use format::{SpeedUnit, format_bandwidth, format_bandwidth_with_unit, format_bytes};
pub use icons::app_glyph_for_emoji;
pub use process::{ActiveAppInfo, ProcessTracker, map_process_to_app, query_active_apps};
