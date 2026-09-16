use crate::backend::{InterfaceMedium, NetworkSnapshot};
use crate::chart::render_unified_chart_data_uri;
use crate::format::{SpeedUnit, format_bandwidth, format_bandwidth_with_unit, format_bytes};
use crate::icons;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Per-widget configuration persisted via CustomState.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetConfig {
    pub speed_unit: SpeedUnit,
    /// Chart window in seconds: 15, 30, or 60.
    pub chart_window: u32,
    /// Whether in-widget active apps view is currently expanded.
    #[serde(default)]
    pub apps_expanded: bool,
    /// Current page of apps when expanded (0-indexed).
    #[serde(default)]
    pub apps_page: usize,
}

impl Default for WidgetConfig {
    fn default() -> Self {
        Self {
            speed_unit: SpeedUnit::Auto,
            chart_window: 30,
            apps_expanded: false,
            apps_page: 0,
        }
    }
}

/// Size-specific layout metrics. Keeping these in one place is what stops the
/// three card variants from drifting apart visually.
struct Layout {
    /// Adaptive Card font size token for the headline bandwidth figures.
    value_size: &'static str,
    /// Chart preset name passed to the renderer.
    chart_size: &'static str,
    /// Apps shown while the section is collapsed (0 = section hidden).
    apps_collapsed: usize,
    /// Apps shown per page while the section is expanded.
    apps_page_size: usize,
    /// Character budget for app names before they are elided.
    apps_name_budget: usize,
    /// Whether the session-total footer is rendered.
    show_session: bool,
}

const SMALL_LAYOUT: Layout = Layout {
    value_size: "Medium",
    chart_size: "Small",
    apps_collapsed: 0,
    apps_page_size: 0,
    apps_name_budget: 0,
    show_session: false,
};

const MEDIUM_LAYOUT: Layout = Layout {
    value_size: "Medium",
    chart_size: "Medium",
    apps_collapsed: 3,
    apps_page_size: 5,
    apps_name_budget: 22,
    show_session: true,
};

const LARGE_LAYOUT: Layout = Layout {
    value_size: "Large",
    chart_size: "Large",
    apps_collapsed: 5,
    apps_page_size: 8,
    apps_name_budget: 24,
    show_session: true,
};

/// Build an Adaptive Card v1.6 template string for the given snapshot, widget size, and config.
pub fn build_adaptive_card(
    snapshot: &NetworkSnapshot,
    size: &str,
    config: &WidgetConfig,
) -> String {
    let layout = match size {
        "Small" => &SMALL_LAYOUT,
        "Large" => &LARGE_LAYOUT,
        _ => &MEDIUM_LAYOUT,
    };

    serde_json::to_string(&build_card(snapshot, config, layout))
        .unwrap_or_else(|_| "{}".to_string())
}

fn fmt_bw(bps: f64, unit: SpeedUnit) -> String {
    format_bandwidth_with_unit(bps, unit)
}

fn fmt_compact_app_bw(bytes_per_sec: f64) -> String {
    if bytes_per_sec.is_nan() || bytes_per_sec <= 0.0 {
        return "0 B/s".to_string();
    }
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;
    if bytes_per_sec >= GB {
        format!("{:.1} GB/s", bytes_per_sec / GB)
    } else if bytes_per_sec >= MB {
        format!("{:.1} MB/s", bytes_per_sec / MB)
    } else if bytes_per_sec >= KB {
        format!("{:.1} KB/s", bytes_per_sec / KB)
    } else {
        format!("{:.0} B/s", bytes_per_sec)
    }
}

/// Compact dual-stream bandwidth formatting that shares the unit when directions match
/// (e.g. "↓ 11.1 ↑ 9.1 KB/s") to prevent horizontal text crowding on narrow cards.
fn fmt_dual_compact_bw(rx: f64, tx: f64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;

    let unit_and_divisor = |val: f64| -> (&'static str, f64) {
        if val >= GB {
            ("GB/s", GB)
        } else if val >= MB {
            ("MB/s", MB)
        } else if val >= KB {
            ("KB/s", KB)
        } else {
            ("B/s", 1.0)
        }
    };

    let (rx_unit, rx_div) = unit_and_divisor(rx);
    let (tx_unit, tx_div) = unit_and_divisor(tx);

    if rx_unit == tx_unit {
        if rx_unit == "B/s" {
            format!("↓ {:.0} ↑ {:.0} {}", rx / rx_div, tx / tx_div, rx_unit)
        } else {
            format!("↓ {:.1} ↑ {:.1} {}", rx / rx_div, tx / tx_div, rx_unit)
        }
    } else {
        let fmt_val = |val: f64, div: f64, u: &str| {
            if u == "B/s" {
                format!("{:.0} {}", val / div, u)
            } else {
                format!("{:.1} {}", val / div, u)
            }
        };
        format!(
            "↓ {} ↑ {}",
            fmt_val(rx, rx_div, rx_unit),
            fmt_val(tx, tx_div, tx_unit)
        )
    }
}

/// Trim a display name to `max` characters on a word boundary where possible,
/// so the app list never produces a hard mid-word cut like "OmenCommandCenterB…".
pub fn truncate_name(name: &str, max: usize) -> String {
    let chars: Vec<char> = name.chars().collect();
    if chars.len() <= max {
        return name.to_string();
    }
    let budget = max.saturating_sub(1).max(1);
    let head: String = chars[..budget].iter().collect();
    // If the cut already lands on a word boundary, keep the whole head.
    let trimmed = if chars.get(budget).is_some_and(|c| c.is_whitespace()) {
        head
    } else {
        match head.rfind(' ') {
            // Only snap back to a word boundary if it keeps most of the budget.
            Some(idx) if idx * 2 >= budget => head[..idx].to_string(),
            _ => head,
        }
    };
    format!("{}…", trimmed.trim_end())
}

/// Vector glyph for a network medium — never an emoji.
fn medium_glyph(medium: InterfaceMedium) -> &'static str {
    match medium {
        InterfaceMedium::Wifi => icons::MEDIUM_WIFI,
        InterfaceMedium::Ethernet => icons::MEDIUM_ETHERNET,
        InterfaceMedium::Cellular => icons::MEDIUM_CELLULAR,
        InterfaceMedium::Loopback => icons::MEDIUM_LOOPBACK,
        InterfaceMedium::Virtual | InterfaceMedium::Other => icons::MEDIUM_GLOBE,
    }
}

/// A fixed-size icon element.
fn glyph(url: &str, px: u32, alt: &str) -> Value {
    json!({
        "type": "Image",
        "url": url,
        "altText": alt,
        "width": format!("{}px", px),
        "height": format!("{}px", px),
        "size": "Auto",
        "spacing": "None"
    })
}

/// A column that just holds one icon, vertically centred against its text.
fn glyph_column(url: &str, px: u32, alt: &str, spacing: &str) -> Value {
    json!({
        "type": "Column",
        "width": "auto",
        "verticalContentAlignment": "Center",
        "spacing": spacing,
        "items": [glyph(url, px, alt)]
    })
}

/// A tappable icon control (used for expand / collapse / paging) rendered as a
/// bare glyph rather than a full-width Adaptive Card button.
fn icon_button_column(url: &str, verb: &str, tooltip: &str) -> Value {
    icon_button_column_sized(url, 18, verb, tooltip)
}

fn icon_button_column_sized(url: &str, px: u32, verb: &str, tooltip: &str) -> Value {
    json!({
        "type": "Column",
        "width": "auto",
        "verticalContentAlignment": "Center",
        "spacing": "Small",
        "selectAction": {
            "type": "Action.Execute",
            "verb": verb,
            "title": tooltip,
            "tooltip": tooltip
        },
        "items": [glyph(url, px, tooltip)]
    })
}

/// Header: medium glyph, interface name, live connection count.
fn header_row(snapshot: &NetworkSnapshot) -> Value {
    let active_count = if snapshot.active_connections_count > 0 {
        snapshot.active_connections_count
    } else {
        snapshot.active_apps.len()
    };

    json!({
        "type": "ColumnSet",
        "spacing": "None",
        "columns": [
            glyph_column(
                medium_glyph(snapshot.primary_medium),
                16,
                snapshot.primary_medium.label(),
                "None"
            ),
            {
                "type": "Column",
                "width": "stretch",
                "verticalContentAlignment": "Center",
                "spacing": "Small",
                "items": [
                    {
                        "type": "TextBlock",
                        "text": truncate_name(&snapshot.primary_name, 22),
                        "size": "Default",
                        "weight": "Bolder",
                        "wrap": false
                    }
                ]
            },
            {
                "type": "Column",
                "width": "auto",
                "verticalContentAlignment": "Center",
                "items": [
                    {
                        "type": "TextBlock",
                        "text": format!("{} conns", active_count),
                        "size": "Small",
                        "isSubtle": true,
                        "horizontalAlignment": "Right",
                        "wrap": false
                    }
                ]
            }
        ]
    })
}

/// One headline metric: direction glyph + label, big value, subtle peak.
fn metric_column(glyph_uri: &str, label: &str, value: &str, peak: &str, value_size: &str) -> Value {
    json!({
        "type": "Column",
        "width": "stretch",
        "items": [
            {
                "type": "ColumnSet",
                "spacing": "None",
                "columns": [
                    glyph_column(glyph_uri, 12, label, "None"),
                    {
                        "type": "Column",
                        "width": "stretch",
                        "verticalContentAlignment": "Center",
                        "spacing": "Small",
                        "items": [
                            {
                                "type": "TextBlock",
                                "text": label,
                                "size": "Small",
                                "isSubtle": true,
                                "wrap": false
                            }
                        ]
                    }
                ]
            },
            {
                "type": "TextBlock",
                "text": value,
                "size": value_size,
                "weight": "Bolder",
                "spacing": "None",
                "wrap": false
            },
            {
                "type": "TextBlock",
                "text": format!("Peak {}", peak),
                "size": "Small",
                "isSubtle": true,
                "spacing": "None",
                "wrap": false
            }
        ]
    })
}

fn metrics_row(snapshot: &NetworkSnapshot, config: &WidgetConfig, value_size: &str) -> Value {
    let window_samples = crate::history_samples_for_secs(config.chart_window);
    let (window_peak_rx, window_peak_tx) = snapshot.chart_window_peak(window_samples);

    json!({
        "type": "ColumnSet",
        "spacing": "Medium",
        "columns": [
            metric_column(
                icons::ARROW_DOWN,
                "Download",
                &fmt_bw(snapshot.rx_bps, config.speed_unit),
                &fmt_bw(window_peak_rx, config.speed_unit),
                value_size
            ),
            metric_column(
                icons::ARROW_UP,
                "Upload",
                &fmt_bw(snapshot.tx_bps, config.speed_unit),
                &fmt_bw(window_peak_tx, config.speed_unit),
                value_size
            )
        ]
    })
}

/// The dual-stream telemetry chart.
fn chart_element(
    snapshot: &NetworkSnapshot,
    config: &WidgetConfig,
    chart_size: &str,
) -> Vec<Value> {
    let uri = render_unified_chart_data_uri(&snapshot.history, chart_size, config.chart_window);
    if uri.is_empty() {
        return Vec::new();
    }
    vec![json!({
        "type": "Image",
        "url": uri,
        "altText": "Download and upload bandwidth over time",
        "size": "Stretch",
        "spacing": "Medium"
    })]
}

/// Active apps list: section header with inline paging controls, then rows.
fn apps_section(snapshot: &NetworkSnapshot, config: &WidgetConfig, layout: &Layout) -> Vec<Value> {
    if snapshot.active_apps.is_empty() || layout.apps_collapsed == 0 {
        return Vec::new();
    }

    let mut elements = Vec::new();
    let total_apps = snapshot.active_apps.len();
    let page_size = layout.apps_page_size.max(1);

    let (start_idx, end_idx, range_label, mut controls) = if config.apps_expanded {
        let total_pages = total_apps.div_ceil(page_size);
        let current_page = config.apps_page.min(total_pages.saturating_sub(1));
        let start = current_page * page_size;
        let end = (start + page_size).min(total_apps);

        let label = if total_pages > 1 {
            format!("{}–{} of {}", start + 1, end, total_apps)
        } else {
            format!("All {}", total_apps)
        };

        let mut controls = Vec::new();
        if current_page > 0 {
            controls.push(icon_button_column(
                icons::CHEVRON_LEFT,
                "prev_apps_page",
                "Previous page",
            ));
        }
        if current_page + 1 < total_pages {
            controls.push(icon_button_column(
                icons::CHEVRON_RIGHT,
                "next_apps_page",
                "Next page",
            ));
        }
        (start, end, label, controls)
    } else {
        let count = total_apps.min(layout.apps_collapsed);
        (
            0,
            count,
            format!("Top {} of {}", count, total_apps),
            Vec::new(),
        )
    };

    if config.apps_expanded || total_apps > layout.apps_collapsed {
        let (uri, verb, tip) = if config.apps_expanded {
            (icons::CHEVRON_UP, "toggle_apps", "Collapse")
        } else {
            (icons::CHEVRON_DOWN, "toggle_apps", "Show all apps")
        };
        controls.push(icon_button_column(uri, verb, tip));
    }

    let mut header_columns = vec![
        glyph_column(icons::PULSE, 14, "Active apps", "None"),
        json!({
            "type": "Column",
            "width": "stretch",
            "verticalContentAlignment": "Center",
            "spacing": "Small",
            "items": [
                {
                    "type": "TextBlock",
                    "text": "Active apps",
                    "size": "Small",
                    "weight": "Bolder",
                    "wrap": false
                }
            ]
        }),
        json!({
            "type": "Column",
            "width": "auto",
            "verticalContentAlignment": "Center",
            "items": [
                {
                    "type": "TextBlock",
                    "text": range_label,
                    "size": "Small",
                    "isSubtle": true,
                    "horizontalAlignment": "Right",
                    "wrap": false
                }
            ]
        }),
    ];
    header_columns.append(&mut controls);

    elements.push(json!({
        "type": "ColumnSet",
        "spacing": "Medium",
        "separator": true,
        "columns": header_columns
    }));

    for app in &snapshot.active_apps[start_idx..end_idx] {
        elements.push(app_row(app, layout.apps_name_budget));
    }

    elements
}

fn app_row(app: &crate::process::ActiveAppInfo, name_budget: usize) -> Value {
    let rx_active = app.rx_bps >= 1.0;
    let tx_active = app.tx_bps >= 1.0;
    let active = rx_active || tx_active;

    let conn_suffix = if app.connection_count > 0 {
        format!(" ({})", app.connection_count)
    } else {
        String::new()
    };

    let status = if rx_active && tx_active {
        format!(
            "{}{}",
            fmt_dual_compact_bw(app.rx_bps, app.tx_bps),
            conn_suffix
        )
    } else if rx_active {
        format!("↓ {}{}", fmt_compact_app_bw(app.rx_bps), conn_suffix)
    } else if tx_active {
        format!("↑ {}{}", fmt_compact_app_bw(app.tx_bps), conn_suffix)
    } else if app.connection_count == 1 {
        "1 conn".to_string()
    } else {
        format!("{} conns", app.connection_count)
    };

    let icon_uri: &str = app
        .icon_data_uri
        .as_deref()
        .unwrap_or_else(|| icons::app_glyph_for_emoji(app.icon));

    json!({
        "type": "ColumnSet",
        "spacing": "Small",
        "columns": [
            glyph_column(icon_uri, 16, &app.name, "None"),
            {
                "type": "Column",
                "width": "stretch",
                "verticalContentAlignment": "Center",
                "spacing": "Small",
                "items": [
                    {
                        "type": "TextBlock",
                        "text": truncate_name(&app.name, name_budget),
                        "size": "Default",
                        "wrap": false
                    }
                ]
            },
            {
                "type": "Column",
                "width": "auto",
                "verticalContentAlignment": "Center",
                "spacing": "Small",
                "items": [
                    {
                        "type": "TextBlock",
                        "text": status,
                        "size": "Small",
                        "weight": if active { "Bolder" } else { "Default" },
                        "isSubtle": !active,
                        "horizontalAlignment": "Right",
                        "wrap": false
                    }
                ]
            }
        ]
    })
}

/// Format elapsed seconds into a concise human-readable duration (e.g. "45s", "12m", "2h 15m").
pub fn format_duration(seconds: u64) -> String {
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;

    if days > 0 {
        let rem_hours = hours % 24;
        if rem_hours > 0 {
            format!("{}d {}h", days, rem_hours)
        } else {
            format!("{}d", days)
        }
    } else if hours > 0 {
        let rem_mins = minutes % 60;
        if rem_mins > 0 {
            format!("{}h {}m", hours, rem_mins)
        } else {
            format!("{}h", hours)
        }
    } else if minutes > 0 {
        format!("{}m", minutes)
    } else {
        format!("{}s", seconds)
    }
}

/// Footer: session totals, duration, and reset button.
fn session_row(snapshot: &NetworkSnapshot) -> Value {
    let duration_str = format_duration(snapshot.session_duration_secs);
    let session_text = if snapshot.session_duration_secs > 0 {
        format!("Session ({})", duration_str)
    } else {
        "Session".to_string()
    };
    let tooltip = format!(
        "Reset session totals (Duration: {} • All-time peak: ↓ {}  ↑ {})",
        duration_str,
        format_bandwidth(snapshot.session_peak_rx_bps),
        format_bandwidth(snapshot.session_peak_tx_bps)
    );

    json!({
        "type": "ColumnSet",
        "spacing": "Medium",
        "separator": true,
        "columns": [
            {
                "type": "Column",
                "width": "auto",
                "verticalContentAlignment": "Center",
                "items": [
                    {
                        "type": "TextBlock",
                        "text": session_text,
                        "size": "Small",
                        "isSubtle": true,
                        "wrap": false
                    }
                ]
            },
            icon_button_column_sized(icons::MEDIUM_LOOPBACK, 13, "reset_session", &tooltip),
            {
                "type": "Column",
                "width": "stretch",
                "items": []
            },
            glyph_column(icons::ARROW_DOWN, 12, "Downloaded", "Small"),
            {
                "type": "Column",
                "width": "auto",
                "verticalContentAlignment": "Center",
                "spacing": "Small",
                "items": [
                    {
                        "type": "TextBlock",
                        "text": format_bytes(snapshot.session_rx),
                        "size": "Small",
                        "isSubtle": true,
                        "wrap": false
                    }
                ]
            },
            glyph_column(icons::ARROW_UP, 12, "Uploaded", "Medium"),
            {
                "type": "Column",
                "width": "auto",
                "verticalContentAlignment": "Center",
                "spacing": "Small",
                "items": [
                    {
                        "type": "TextBlock",
                        "text": format_bytes(snapshot.session_tx),
                        "size": "Small",
                        "isSubtle": true,
                        "wrap": false
                    }
                ]
            }
        ]
    })
}

fn build_card(snapshot: &NetworkSnapshot, config: &WidgetConfig, layout: &Layout) -> Value {
    let mut body: Vec<Value> = vec![
        header_row(snapshot),
        metrics_row(snapshot, config, layout.value_size),
    ];

    body.extend(chart_element(snapshot, config, layout.chart_size));
    body.extend(apps_section(snapshot, config, layout));

    if layout.show_session {
        body.push(session_row(snapshot));
    }

    json!({
        "$schema": "http://adaptivecards.io/schemas/adaptive-card.json",
        "type": "AdaptiveCard",
        "version": "1.6",
        "body": body
    })
}

/// Build the settings (customization) card displayed when the user picks
/// "Customize widget" from the widget's native overflow menu.
pub fn build_settings_card(current_config: &WidgetConfig) -> String {
    let card = json!({
        "$schema": "http://adaptivecards.io/schemas/adaptive-card.json",
        "type": "AdaptiveCard",
        "version": "1.6",
        "body": [
            {
                "type": "ColumnSet",
                "spacing": "None",
                "columns": [
                    {
                        "type": "Column",
                        "width": "stretch",
                        "verticalContentAlignment": "Center",
                        "items": [
                            {
                                "type": "TextBlock",
                                "text": "Net Flow Settings",
                                "weight": "Bolder",
                                "size": "Medium",
                                "wrap": false
                            }
                        ]
                    },
                    {
                        "type": "Column",
                        "width": "auto",
                        "verticalContentAlignment": "Center",
                        "selectAction": {
                            "type": "Action.Execute",
                            "verb": "cancel_settings",
                            "title": "Done",
                            "tooltip": "Done",
                            "associatedInputs": "none"
                        },
                        "items": [
                            {
                                "type": "TextBlock",
                                "text": "Done",
                                "weight": "Bolder",
                                "size": "Small",
                                "color": "Accent"
                            }
                        ]
                    }
                ]
            },
            {
                "type": "TextBlock",
                "text": "Units, history window, and app list behaviour.",
                "size": "Small",
                "isSubtle": true,
                "spacing": "None",
                "wrap": true
            },
            {
                "type": "Container",
                "spacing": "Medium",
                "separator": true,
                "items": [
                    {
                        "type": "TextBlock",
                        "text": "Speed units",
                        "weight": "Bolder",
                        "size": "Small",
                        "wrap": false
                    },
                    {
                        "type": "Input.ChoiceSet",
                        "id": "speed_unit",
                        "style": "compact",
                        "spacing": "Small",
                        "value": current_config.speed_unit.to_str_value(),
                        "choices": [
                            { "title": "Auto (adaptive)", "value": "auto" },
                            { "title": "Bytes (B/s)", "value": "b" },
                            { "title": "Kilobytes (KB/s)", "value": "kb" },
                            { "title": "Megabytes (MB/s)", "value": "mb" },
                            { "title": "Gigabytes (GB/s)", "value": "gb" }
                        ]
                    }
                ]
            },
            {
                "type": "Container",
                "spacing": "Medium",
                "separator": true,
                "items": [
                    {
                        "type": "TextBlock",
                        "text": "History window",
                        "weight": "Bolder",
                        "size": "Small",
                        "wrap": false
                    },
                    {
                        "type": "Input.ChoiceSet",
                        "id": "chart_window",
                        "style": "compact",
                        "spacing": "Small",
                        "value": current_config.chart_window.to_string(),
                        "choices": [
                            { "title": "15 seconds", "value": "15" },
                            { "title": "30 seconds", "value": "30" },
                            { "title": "60 seconds", "value": "60" }
                        ]
                    }
                ]
            },
            {
                "type": "Container",
                "spacing": "Medium",
                "separator": true,
                "items": [
                    {
                        "type": "TextBlock",
                        "text": "Active apps",
                        "weight": "Bolder",
                        "size": "Small",
                        "wrap": false
                    },
                    {
                        "type": "Input.Toggle",
                        "id": "apps_expanded",
                        "spacing": "Small",
                        "title": "Expand the app list by default",
                        "value": if current_config.apps_expanded { "true" } else { "false" },
                        "valueOn": "true",
                        "valueOff": "false"
                    }
                ]
            },
            {
                "type": "ActionSet",
                "spacing": "Medium",
                "actions": [
                    {
                        "type": "Action.Execute",
                        "title": "Save",
                        "verb": "save_settings",
                        "style": "positive"
                    },
                    {
                        "type": "Action.Execute",
                        "title": "Reset session",
                        "verb": "reset_session",
                        "associatedInputs": "none"
                    },
                    {
                        "type": "Action.Execute",
                        "title": "Back",
                        "verb": "cancel_settings",
                        "associatedInputs": "none"
                    }
                ]
            }
        ]
    });

    serde_json::to_string(&card).unwrap_or_else(|_| "{}".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::HistorySample;
    use std::time::Instant;

    fn snapshot_fixture() -> NetworkSnapshot {
        NetworkSnapshot {
            rx_bps: 10485760.0, // 10 MB/s
            tx_bps: 2097152.0,  // 2 MB/s
            session_rx: 104857600,
            session_tx: 20971520,
            active_interfaces: 2,
            peak_rx_bps: 25000000.0,
            peak_tx_bps: 5000000.0,
            session_duration_secs: 180,
            session_peak_rx_bps: 25000000.0,
            session_peak_tx_bps: 5000000.0,
            timestamp: Instant::now(),
            per_interface: vec![],
            history: vec![
                HistorySample {
                    rx_bps: 1000,
                    tx_bps: 500,
                },
                HistorySample {
                    rx_bps: 2000,
                    tx_bps: 1000,
                },
                HistorySample {
                    rx_bps: 5000,
                    tx_bps: 3000,
                },
                HistorySample {
                    rx_bps: 3000,
                    tx_bps: 2000,
                },
                HistorySample {
                    rx_bps: 8000,
                    tx_bps: 4000,
                },
            ],
            primary_medium: crate::backend::InterfaceMedium::Wifi,
            primary_name: "Wi-Fi".to_string(),
            active_apps: vec![
                crate::process::ActiveAppInfo {
                    name: "Chrome".to_string(),
                    process_name: "chrome.exe".to_string(),
                    icon: "🌐",
                    icon_data_uri: None,
                    connection_count: 8,
                    rx_bps: 14500.0,
                    tx_bps: 2100.0,
                },
                crate::process::ActiveAppInfo {
                    name: "Discord".to_string(),
                    process_name: "discord.exe".to_string(),
                    icon: "💬",
                    icon_data_uri: None,
                    connection_count: 3,
                    rx_bps: 0.0,
                    tx_bps: 0.0,
                },
            ],
            active_connections_count: 11,
        }
    }

    fn many_apps_fixture(count: usize) -> NetworkSnapshot {
        let mut snap = snapshot_fixture();
        snap.active_apps.clear();
        for idx in 0..count {
            snap.active_apps.push(crate::process::ActiveAppInfo {
                name: format!("App {}", idx + 1),
                process_name: format!("app{}.exe", idx + 1),
                icon: "🌐",
                icon_data_uri: None,
                connection_count: count - idx,
                rx_bps: (count - idx) as f64 * 1024.0,
                tx_bps: 0.0,
            });
        }
        snap.active_connections_count = 213;
        snap
    }

    #[test]
    fn cards_are_valid_adaptive_cards() {
        let snap = snapshot_fixture();
        let config = WidgetConfig::default();
        for size in ["Small", "Medium", "Large"] {
            let parsed: Value =
                serde_json::from_str(&build_adaptive_card(&snap, size, &config)).expect("valid");
            assert_eq!(parsed["version"], "1.6");
            assert_eq!(parsed["type"], "AdaptiveCard");
            assert!(!parsed["body"].as_array().unwrap().is_empty());
        }
    }

    #[test]
    fn card_contains_chart_image() {
        let snap = snapshot_fixture();
        let json_str = build_adaptive_card(&snap, "Medium", &WidgetConfig::default());
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        let body = parsed["body"].as_array().unwrap();
        assert!(body.iter().any(|e| e["type"] == "Image"));
    }

    #[test]
    fn card_never_emits_emoji() {
        let snap = many_apps_fixture(9);
        for expanded in [false, true] {
            let config = WidgetConfig {
                apps_expanded: expanded,
                ..WidgetConfig::default()
            };
            for size in ["Small", "Medium", "Large"] {
                let json_str = build_adaptive_card(&snap, size, &config);
                for emoji in ["⚡", "🛜", "🌐", "💬", "▲", "▼", "◀", "▶"] {
                    assert!(
                        !json_str.contains(emoji),
                        "{} card still renders the {} glyph as text",
                        size,
                        emoji
                    );
                }
            }
        }
    }

    #[test]
    fn header_uses_vector_medium_glyph_and_conn_count() {
        let snap = snapshot_fixture();
        let json_str = build_adaptive_card(&snap, "Medium", &WidgetConfig::default());
        assert!(json_str.contains(icons::MEDIUM_WIFI));
        assert!(json_str.contains("Wi-Fi"));
        assert!(json_str.contains("11 conns"));
        // Settings live in the host's native overflow menu, not on the card.
        assert!(!json_str.contains("open_settings"));
    }

    #[test]
    fn app_rows_render_rates_and_fallback_glyphs() {
        let snap = snapshot_fixture();
        let json_str = build_adaptive_card(&snap, "Large", &WidgetConfig::default());
        // An active app displays both its throughput and its connection count compactly
        assert!(json_str.contains("↓ 14.2 ↑ 2.1 KB/s (8)"));
        // An idle app is described by the sockets it is holding open.
        assert!(json_str.contains("3 conns"));
        // Emoji hints are resolved to vector glyphs.
        assert!(json_str.contains(icons::APP_CHAT));
    }

    #[test]
    fn native_icon_wins_over_fallback_glyph() {
        let mut snap = snapshot_fixture();
        snap.active_apps[0].icon_data_uri =
            Some("data:image/png;base64,iVBORw0KGgoAAAANSUhEUg==".to_string());
        let json_str = build_adaptive_card(&snap, "Large", &WidgetConfig::default());
        assert!(json_str.contains("data:image/png;base64,iVBORw0KGgoAAAANSUhEUg=="));
    }

    #[test]
    fn apps_paging_uses_inline_icon_controls() {
        let snap = many_apps_fixture(13);

        let collapsed = build_adaptive_card(&snap, "Large", &WidgetConfig::default());
        assert!(collapsed.contains("Top 5 of 13"));
        assert!(collapsed.contains("toggle_apps"));
        assert!(collapsed.contains(icons::CHEVRON_DOWN));
        assert!(collapsed.contains("App 5"));
        assert!(!collapsed.contains("App 6"));
        // Paging controls are bare glyphs with selectAction, not full-width buttons.
        assert!(!collapsed.contains("\"type\":\"ActionSet\""));

        let page1 = build_adaptive_card(
            &snap,
            "Large",
            &WidgetConfig {
                apps_expanded: true,
                apps_page: 0,
                ..WidgetConfig::default()
            },
        );
        assert!(page1.contains("1–8 of 13"));
        assert!(page1.contains("next_apps_page"));
        assert!(!page1.contains("prev_apps_page"));
        assert!(page1.contains(icons::CHEVRON_UP));

        let page2 = build_adaptive_card(
            &snap,
            "Large",
            &WidgetConfig {
                apps_expanded: true,
                apps_page: 1,
                ..WidgetConfig::default()
            },
        );
        assert!(page2.contains("9–13 of 13"));
        assert!(page2.contains("prev_apps_page"));
        assert!(!page2.contains("next_apps_page"));

        // Out-of-range pages clamp to the last page instead of rendering empty.
        let page_overflow = build_adaptive_card(
            &snap,
            "Large",
            &WidgetConfig {
                apps_expanded: true,
                apps_page: 99,
                ..WidgetConfig::default()
            },
        );
        assert!(page_overflow.contains("9–13 of 13"));
        assert!(!page_overflow.contains("next_apps_page"));
    }

    #[test]
    fn medium_card_pages_five_apps() {
        let snap = many_apps_fixture(13);
        let json_str = build_adaptive_card(
            &snap,
            "Medium",
            &WidgetConfig {
                apps_expanded: true,
                apps_page: 0,
                ..WidgetConfig::default()
            },
        );
        assert!(json_str.contains("1–5 of 13"));
        assert!(json_str.contains("App 5"));
        assert!(!json_str.contains("App 6"));
        // Active apps use compact (N) format; only the header total uses " conns".
        assert_eq!(json_str.matches(" conns").count(), 1);
        assert!(json_str.contains("(13)"));
    }

    #[test]
    fn small_card_stays_minimal() {
        let snap = many_apps_fixture(13);
        let json_str = build_adaptive_card(&snap, "Small", &WidgetConfig::default());
        assert!(!json_str.contains("Active apps"));
        assert!(!json_str.contains("Session"));
        assert!(json_str.contains("Download"));
    }

    #[test]
    fn truncation_snaps_to_word_boundaries() {
        assert_eq!(truncate_name("Chrome", 20), "Chrome");
        assert_eq!(
            truncate_name("Omen Command Center Background", 20),
            "Omen Command Center…"
        );
        assert_eq!(truncate_name("Supercalifragilistic", 10), "Supercali…");
    }

    #[test]
    fn settings_card_is_valid_and_complete() {
        let json_str = build_settings_card(&WidgetConfig::default());
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["version"], "1.6");
        for verb in ["save_settings", "reset_session", "cancel_settings"] {
            assert!(json_str.contains(verb));
        }
        assert!(json_str.contains("\"id\":\"apps_expanded\""));
    }

    #[test]
    fn widget_config_json_roundtrip() {
        let config = WidgetConfig {
            speed_unit: SpeedUnit::Megabytes,
            chart_window: 60,
            apps_expanded: false,
            apps_page: 0,
        };
        let parsed: WidgetConfig =
            serde_json::from_str(&serde_json::to_string(&config).unwrap()).unwrap();
        assert_eq!(config, parsed);
    }

    #[test]
    fn fixed_unit_is_honoured() {
        let snap = snapshot_fixture();
        let json_str = build_adaptive_card(
            &snap,
            "Medium",
            &WidgetConfig {
                speed_unit: SpeedUnit::Megabytes,
                ..WidgetConfig::default()
            },
        );
        assert!(json_str.contains("MB/s"));
    }

    #[test]
    fn chart_window_reflected_in_card_for_all_sizes() {
        let snap = snapshot_fixture();
        for size in ["Small", "Medium", "Large"] {
            for window in [15, 30, 60] {
                let config = WidgetConfig {
                    chart_window: window,
                    ..WidgetConfig::default()
                };
                let json_str = build_adaptive_card(&snap, size, &config);
                assert!(json_str.contains("data:image/png;base64,"));
                let parsed: Value = serde_json::from_str(&json_str).unwrap();
                assert_eq!(parsed["type"], "AdaptiveCard");
            }
        }
    }

    #[test]
    fn format_duration_scales_humanely() {
        assert_eq!(format_duration(0), "0s");
        assert_eq!(format_duration(45), "45s");
        assert_eq!(format_duration(59), "59s");
        assert_eq!(format_duration(60), "1m");
        assert_eq!(format_duration(120), "2m");
        assert_eq!(format_duration(185), "3m");
        assert_eq!(format_duration(3600), "1h");
        assert_eq!(format_duration(3660), "1h 1m");
        assert_eq!(format_duration(7800), "2h 10m");
    }

    #[test]
    fn session_row_renders_duration_and_inline_reset_action() {
        let snap = snapshot_fixture();
        let json_str = build_adaptive_card(&snap, "Medium", &WidgetConfig::default());
        assert!(json_str.contains("Session (3m)"));
        assert!(json_str.contains("reset_session"));
        assert!(json_str.contains("Reset session totals"));
    }

    #[test]
    fn chart_window_peak_tracks_only_window_samples() {
        let mut snap = snapshot_fixture();
        snap.rx_bps = 10_000.0;
        snap.tx_bps = 5_000.0;
        // Clear history and push an old giant peak followed by small traffic
        snap.history.clear();
        snap.history.push(HistorySample {
            rx_bps: 100_000_000,
            tx_bps: 50_000_000,
        });
        for _ in 0..10 {
            snap.history.push(HistorySample {
                rx_bps: 10_000,
                tx_bps: 5_000,
            });
        }
        // If window is 5 samples, the old peak (index 0) is excluded
        let (peak_rx, peak_tx) = snap.chart_window_peak(5);
        assert_eq!(peak_rx, 10_000.0);
        assert_eq!(peak_tx, 5_000.0);

        // If window covers all 11 samples, the old peak is included
        let (peak_rx_all, peak_tx_all) = snap.chart_window_peak(15);
        assert_eq!(peak_rx_all, 100_000_000.0);
        assert_eq!(peak_tx_all, 50_000_000.0);
    }

    #[test]
    fn manifest_version_matches_cargo_pkg_version() {
        let manifest_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../widget/Package.appxmanifest");
        let content = std::fs::read_to_string(&manifest_path)
            .expect("Package.appxmanifest must be readable");

        // Expected 4-part MSIX version: e.g. "0.1.0" -> "0.1.0.0"
        let pkg_ver = env!("CARGO_PKG_VERSION");
        let expected_quad = format!("{}.0", pkg_ver);

        let re_pattern = format!(r#"Version="{}""#, expected_quad);
        assert!(
            content.contains(&re_pattern),
            "Package.appxmanifest version must match Cargo.toml version (expected: {}, manifest path: {:?})",
            expected_quad,
            manifest_path
        );
    }
}
