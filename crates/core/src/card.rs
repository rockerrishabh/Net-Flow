use crate::backend::{InterfaceMedium, NetworkSnapshot};
use crate::chart::{
    GraphStyle, ThemeMode, render_idle_unified_chart_data_uri, render_unified_chart_data_uri,
};
use crate::format::{SpeedUnit, format_bandwidth, format_bandwidth_with_unit, format_bytes};
use crate::icons;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::time::Duration;

/// Throughput threshold above which telemetry publication accelerates to 500 ms (250 KiB/s).
pub const BURST_THRESHOLD_BPS: f64 = 250.0 * 1024.0;

/// Throughput threshold for moderate background activity updating at 1,000 ms (10 KiB/s).
pub const MODERATE_THRESHOLD_BPS: f64 = 10.0 * 1024.0;

/// Computes the traffic-adaptive data publication interval based on bidirectional throughput.
///
/// Traffic metric: `traffic = max(download_rate, upload_rate)`
/// - Burst traffic (>= 250 KiB/s): 500 ms (2 Hz) for fluid, responsive waveform animation.
/// - Moderate traffic (10 KiB/s .. 250 KiB/s): 1,000 ms (1 Hz) to balance freshness with host load.
/// - Low / near-idle traffic (< 10 KiB/s): 1,500 ms (combined with Phase B payload diffing).
pub fn compute_adaptive_ui_interval(download_rate: f64, upload_rate: f64) -> Duration {
    let traffic = download_rate.max(upload_rate);
    if traffic >= BURST_THRESHOLD_BPS {
        Duration::from_millis(500)
    } else if traffic >= MODERATE_THRESHOLD_BPS {
        Duration::from_millis(1000)
    } else {
        Duration::from_millis(1500)
    }
}

/// User configuration options saved in the widget's persistent CustomState.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetConfig {
    pub speed_unit: SpeedUnit,
    /// Chart time window in seconds (15, 30, or 60).
    pub chart_window: u32,
    /// Indicates whether the active apps drawer is expanded.
    #[serde(default)]
    pub apps_expanded: bool,
    /// Currently visible page in the active apps list (0-indexed).
    #[serde(default)]
    pub apps_page: usize,
    /// Theme preference: Auto, Dark, or Light.
    #[serde(default)]
    pub theme: ThemeMode,
    /// Presentation style of the sparkline chart: Area, Line, or Bar.
    #[serde(default)]
    pub graph_style: GraphStyle,
}

impl Default for WidgetConfig {
    fn default() -> Self {
        Self {
            speed_unit: SpeedUnit::Auto,
            chart_window: 30,
            apps_expanded: false,
            apps_page: 0,
            theme: ThemeMode::Auto,
            graph_style: GraphStyle::Area,
        }
    }
}

/// Visual layout budget and typography sizing for each widget form factor (Small, Medium, Large).
struct Layout {
    /// Font size token used for primary bandwidth figures.
    value_size: &'static str,
    /// Chart preset key passed to the sparkline rasterizer.
    chart_size: &'static str,
    /// Number of apps shown when collapsed (0 hides the section).
    apps_collapsed: usize,
    /// Number of apps shown per page when expanded.
    apps_page_size: usize,
    /// Character limit for application names before truncating with an ellipsis.
    apps_name_budget: usize,
    /// Whether to show the bottom session summary row.
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
    apps_page_size: 4,
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

fn layout_for_size(size: &str) -> &'static Layout {
    match size {
        "Small" => &SMALL_LAYOUT,
        "Large" => &LARGE_LAYOUT,
        _ => &MEDIUM_LAYOUT,
    }
}

/// Generates an Adaptive Card v1.6 JSON template for the current snapshot and widget size.
pub fn build_adaptive_card(
    snapshot: &NetworkSnapshot,
    size: &str,
    config: &WidgetConfig,
) -> String {
    let layout = layout_for_size(size);

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

/// A tappable icon control rendered inside an explicit hit-target container
/// with centered content to provide a clean, tactile Fluent hover pill/effect.
fn icon_button_box(url: &str, px: u32, verb: &str, tooltip: &str) -> Value {
    json!({
        "type": "Container",
        "minHeight": "28px",
        "roundedCorners": true,
        "verticalContentAlignment": "Center",
        "horizontalAlignment": "Center",
        "selectAction": {
            "type": "Action.Execute",
            "verb": verb,
            "title": tooltip,
            "tooltip": tooltip
        },
        "items": [
            {
                "type": "Image",
                "url": url,
                "altText": tooltip,
                "width": format!("{}px", px),
                "height": format!("{}px", px),
                "horizontalAlignment": "Center",
                "size": "Auto",
                "spacing": "None"
            }
        ]
    })
}

/// A tappable text button control rendered inside an explicit hit-target container
/// with centered content to provide a clean, tactile Fluent hover pill/effect.
fn text_button_box(
    text: &str,
    width_px: Option<u32>,
    verb: &str,
    tooltip: &str,
    color: Option<&str>,
    is_bold: bool,
    associated_inputs: Option<&str>,
) -> Value {
    let mut select_action = json!({
        "type": "Action.Execute",
        "verb": verb,
        "title": tooltip,
        "tooltip": tooltip
    });
    if let Some(inputs) = associated_inputs {
        select_action["associatedInputs"] = json!(inputs);
    }

    let mut text_item = json!({
        "type": "TextBlock",
        "text": text,
        "size": "Small",
        "horizontalAlignment": "Center",
        "wrap": false
    });
    if is_bold {
        text_item["weight"] = json!("Bolder");
    }
    if let Some(c) = color {
        text_item["color"] = json!(c);
    } else {
        text_item["isSubtle"] = json!(true);
    }

    let mut container = json!({
        "type": "Container",
        "minHeight": "28px",
        "roundedCorners": true,
        "verticalContentAlignment": "Center",
        "horizontalAlignment": "Center",
        "selectAction": select_action,
        "items": [text_item]
    });
    if let Some(w) = width_px {
        container["width"] = json!(format!("{}px", w));
    }
    container
}

fn icon_button_column_spaced(
    url: &str,
    px: u32,
    verb: &str,
    tooltip: &str,
    spacing: &str,
) -> Value {
    json!({
        "type": "Column",
        "width": "28px",
        "roundedCorners": true,
        "verticalContentAlignment": "Center",
        "spacing": spacing,
        "items": [icon_button_box(url, px, verb, tooltip)]
    })
}

fn icon_button_column_sized(url: &str, px: u32, verb: &str, tooltip: &str) -> Value {
    icon_button_column_spaced(url, px, verb, tooltip, "Small")
}

fn icon_button_column(url: &str, verb: &str, tooltip: &str) -> Value {
    icon_button_column_sized(url, 16, verb, tooltip)
}

fn icon_button_column_when(
    url: &str,
    px: u32,
    verb: &str,
    tooltip: &str,
    when: &str,
    spacing: &str,
) -> Value {
    json!({
        "type": "Column",
        "$when": when,
        "width": "28px",
        "roundedCorners": true,
        "verticalContentAlignment": "Center",
        "spacing": spacing,
        "items": [icon_button_box(url, px, verb, tooltip)]
    })
}

/// A column that holds a native text glyph (such as "↓" or "↑"), avoiding async image decoding.
fn text_glyph_column(text: &str, color: &str, size: &str, spacing: &str) -> Value {
    json!({
        "type": "Column",
        "width": "auto",
        "verticalContentAlignment": "Center",
        "spacing": spacing,
        "items": [
            {
                "type": "TextBlock",
                "text": text,
                "size": size,
                "color": color,
                "weight": "Bolder",
                "wrap": false
            }
        ]
    })
}

/// Template for a single fixed active app slot with stable visual elements.
fn app_slot_template(slot: usize) -> Value {
    let vis_cond = format!("${{app{}_visible == true}}", slot);
    let icon_bind = format!("${{app{}_icon}}", slot);
    let name_bind = format!("${{app{}_name}}", slot);
    let rate_bind = format!("${{app{}_rate}}", slot);
    let weight_bind = format!("${{app{}_weight}}", slot);
    let subtle_bind = format!("${{app{}_isSubtle}}", slot);

    json!({
        "type": "ColumnSet",
        "$when": vis_cond,
        "spacing": "Small",
        "columns": [
            {
                "type": "Column",
                "width": "auto",
                "verticalContentAlignment": "Center",
                "spacing": "None",
                "items": [
                    {
                        "type": "Image",
                        "url": icon_bind,
                        "width": "16px",
                        "height": "16px",
                        "altText": name_bind
                    }
                ]
            },
            {
                "type": "Column",
                "width": "stretch",
                "verticalContentAlignment": "Center",
                "spacing": "Small",
                "items": [
                    {
                        "type": "TextBlock",
                        "text": name_bind,
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
                        "text": rate_bind,
                        "size": "Small",
                        "weight": weight_bind,
                        "isSubtle": subtle_bind,
                        "horizontalAlignment": "Right",
                        "wrap": false
                    }
                ]
            }
        ]
    })
}

/// Header: medium glyph, interface name, live connection count, and settings button.
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
            },
            icon_button_column_spaced(icons::SETTINGS, 16, "open_settings", "Customize widget", "Small")
        ]
    })
}

/// One headline metric: direction glyph + label, big value, subtle peak.
fn metric_column(
    glyph_uri: &str,
    label: &str,
    value: &str,
    peak: &str,
    value_size: &str,
    right_aligned: bool,
) -> Value {
    let header_columns = if right_aligned {
        vec![
            json!({
                "type": "Column",
                "width": "stretch",
                "items": []
            }),
            glyph_column(glyph_uri, 12, label, "None"),
            json!({
                "type": "Column",
                "width": "auto",
                "verticalContentAlignment": "Center",
                "spacing": "Small",
                "items": [
                    {
                        "type": "TextBlock",
                        "text": label,
                        "size": "Small",
                        "isSubtle": true,
                        "horizontalAlignment": "Right",
                        "wrap": false
                    }
                ]
            }),
        ]
    } else {
        vec![
            glyph_column(glyph_uri, 12, label, "None"),
            json!({
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
            }),
        ]
    };

    let mut value_block = json!({
        "type": "TextBlock",
        "text": value,
        "size": value_size,
        "weight": "Bolder",
        "spacing": "None",
        "wrap": false
    });
    let mut peak_block = json!({
        "type": "TextBlock",
        "text": format!("Peak {}", peak),
        "size": "Small",
        "isSubtle": true,
        "spacing": "None",
        "wrap": false
    });

    if right_aligned {
        value_block["horizontalAlignment"] = json!("Right");
        peak_block["horizontalAlignment"] = json!("Right");
    }

    json!({
        "type": "Column",
        "width": "stretch",
        "items": [
            {
                "type": "ColumnSet",
                "spacing": "None",
                "columns": header_columns
            },
            value_block,
            peak_block
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
                value_size,
                false
            ),
            metric_column(
                icons::ARROW_UP,
                "Upload",
                &fmt_bw(snapshot.tx_bps, config.speed_unit),
                &fmt_bw(window_peak_tx, config.speed_unit),
                value_size,
                true
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
    let resolved_theme = config.theme.resolve();
    let graph_style = config.graph_style;

    // O(1) idle bypass: if both incremental peaks across the entire history buffer are 0,
    // we are guaranteed that every sample in any chart window is 0 bps. Directly fetch
    // the precomputed idle chart from the immutable OnceLock cache.
    let uri = if snapshot.peak_rx_bps == 0.0 && snapshot.peak_tx_bps == 0.0 {
        render_idle_unified_chart_data_uri(
            chart_size,
            config.chart_window,
            resolved_theme,
            graph_style,
        )
    } else {
        render_unified_chart_data_uri(
            &snapshot.history,
            chart_size,
            config.chart_window,
            resolved_theme,
            graph_style,
        )
    };
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
            icon_button_column_sized(icons::MEDIUM_LOOPBACK, 15, "reset_session", &tooltip),
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

/// Builds a static Adaptive Card template for the given widget size with `${...}` binding expressions.
pub fn build_adaptive_card_template(size: &str) -> String {
    let layout = layout_for_size(size);
    let mut body = Vec::new();

    // 1. Header row
    body.push(json!({
        "type": "ColumnSet",
        "spacing": "None",
        "columns": [
            {
                "type": "Column",
                "width": "auto",
                "verticalContentAlignment": "Center",
                "spacing": "None",
                "items": [
                    {
                        "type": "Image",
                        "url": "${primaryMediumGlyph}",
                        "width": "16px",
                        "height": "16px",
                        "altText": "Network medium"
                    }
                ]
            },
            {
                "type": "Column",
                "width": "stretch",
                "verticalContentAlignment": "Center",
                "spacing": "Small",
                "items": [
                    {
                        "type": "TextBlock",
                        "text": "${primaryName}",
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
                        "text": "${activeConnsText}",
                        "size": "Small",
                        "isSubtle": true,
                        "horizontalAlignment": "Right",
                        "wrap": false
                    }
                ]
            },
            icon_button_column_spaced(icons::SETTINGS, 16, "open_settings", "Customize widget", "Small")
        ]
    }));

    // 2. Metrics row (Download / Upload)
    body.push(json!({
        "type": "ColumnSet",
        "spacing": "Medium",
        "columns": [
            {
                "type": "Column",
                "width": "stretch",
                "items": [
                    {
                        "type": "ColumnSet",
                        "spacing": "None",
                        "columns": [
                            text_glyph_column("↓", "Accent", "Small", "None"),
                            {
                                "type": "Column",
                                "width": "stretch",
                                "verticalContentAlignment": "Center",
                                "spacing": "Small",
                                "items": [
                                    {
                                        "type": "TextBlock",
                                        "text": "Download",
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
                        "text": "${downloadRate}",
                        "size": layout.value_size,
                        "weight": "Bolder",
                        "spacing": "None",
                        "wrap": false
                    },
                    {
                        "type": "TextBlock",
                        "text": "${downloadPeak}",
                        "size": "Small",
                        "isSubtle": true,
                        "spacing": "None",
                        "wrap": false
                    }
                ]
            },
            {
                "type": "Column",
                "width": "stretch",
                "items": [
                    {
                        "type": "ColumnSet",
                        "spacing": "None",
                        "columns": [
                            {
                                "type": "Column",
                                "width": "stretch",
                                "items": []
                            },
                            text_glyph_column("↑", "Warning", "Small", "None"),
                            {
                                "type": "Column",
                                "width": "auto",
                                "verticalContentAlignment": "Center",
                                "spacing": "Small",
                                "items": [
                                    {
                                        "type": "TextBlock",
                                        "text": "Upload",
                                        "size": "Small",
                                        "isSubtle": true,
                                        "horizontalAlignment": "Right",
                                        "wrap": false
                                    }
                                ]
                            }
                        ]
                    },
                    {
                        "type": "TextBlock",
                        "text": "${uploadRate}",
                        "size": layout.value_size,
                        "weight": "Bolder",
                        "horizontalAlignment": "Right",
                        "spacing": "None",
                        "wrap": false
                    },
                    {
                        "type": "TextBlock",
                        "text": "${uploadPeak}",
                        "size": "Small",
                        "isSubtle": true,
                        "horizontalAlignment": "Right",
                        "spacing": "None",
                        "wrap": false
                    }
                ]
            }
        ]
    }));

    // 3. Dual-stream sparkline image
    body.push(json!({
        "type": "Image",
        "url": "${chartUrl}",
        "altText": "Download and upload bandwidth over time",
        "size": "Stretch",
        "spacing": "Medium"
    }));

    // 4. Active Apps Section (Medium and Large only)
    if layout.apps_collapsed > 0 {
        let max_slots = layout.apps_page_size;
        let mut app_slots = Vec::new();
        for i in 0..max_slots {
            app_slots.push(app_slot_template(i));
        }

        body.push(json!({
            "type": "Container",
            "$when": "${hasActiveApps == true}",
            "items": [
                {
                    "type": "ColumnSet",
                    "spacing": "Medium",
                    "separator": true,
                    "columns": [
                        {
                            "type": "Column",
                            "width": "stretch",
                            "verticalContentAlignment": "Center",
                            "items": [
                                {
                                    "type": "TextBlock",
                                    "text": "Active apps",
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
                                    "text": "${appsRangeLabel}",
                                    "size": "Small",
                                    "isSubtle": true,
                                    "horizontalAlignment": "Right",
                                    "wrap": false
                                }
                            ]
                        },
                        icon_button_column_when(
                            icons::CHEVRON_LEFT,
                            16,
                            "prev_apps_page",
                            "Previous page",
                            "${hasPrevPage == true}",
                            "Small",
                        ),
                        icon_button_column_when(
                            icons::CHEVRON_RIGHT,
                            16,
                            "next_apps_page",
                            "Next page",
                            "${hasNextPage == true}",
                            "Small",
                        ),
                        icon_button_column_when(
                            "${toggleAppsIcon}",
                            16,
                            "toggle_apps",
                            "${toggleAppsTitle}",
                            "${canToggleApps == true}",
                            "Small",
                        ),
                    ]
                },
                {
                    "type": "Container",
                    "spacing": "None",
                    "items": app_slots
                }
            ]
        }));
    }

    // 5. Session Footer (Medium and Large only)
    if layout.show_session {
        body.push(json!({
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
                            "text": "${sessionText}",
                            "size": "Small",
                            "isSubtle": true,
                            "wrap": false
                        }
                    ]
                },
                icon_button_column_sized(icons::MEDIUM_LOOPBACK, 15, "reset_session", "${sessionTooltip}"),
                {
                    "type": "Column",
                    "width": "stretch",
                    "items": []
                },
                text_glyph_column("↓", "Accent", "Small", "Small"),
                {
                    "type": "Column",
                    "width": "auto",
                    "verticalContentAlignment": "Center",
                    "spacing": "Small",
                    "items": [
                        {
                            "type": "TextBlock",
                            "text": "${sessionRx}",
                            "size": "Small",
                            "isSubtle": true,
                            "wrap": false
                        }
                    ]
                },
                text_glyph_column("↑", "Warning", "Small", "Medium"),
                {
                    "type": "Column",
                    "width": "auto",
                    "verticalContentAlignment": "Center",
                    "spacing": "Small",
                    "items": [
                        {
                            "type": "TextBlock",
                            "text": "${sessionTx}",
                            "size": "Small",
                            "isSubtle": true,
                            "wrap": false
                        }
                    ]
                }
            ]
        }));
    }

    json!({
        "$schema": "http://adaptivecards.io/schemas/adaptive-card.json",
        "type": "AdaptiveCard",
        "version": "1.6",
        "body": body
    })
    .to_string()
}

/// Builds dynamic telemetry data context matching the static Adaptive Card template bindings.
pub fn build_adaptive_card_data(
    snapshot: &NetworkSnapshot,
    config: &WidgetConfig,
    size: &str,
) -> Value {
    let layout = layout_for_size(size);

    let active_count = if snapshot.active_connections_count > 0 {
        snapshot.active_connections_count
    } else {
        snapshot.active_apps.len()
    };

    let window_samples = crate::history_samples_for_secs(config.chart_window);
    let (window_peak_rx, window_peak_tx) = snapshot.chart_window_peak(window_samples);

    let resolved_theme = config.theme.resolve();
    let graph_style = config.graph_style;
    let chart_url = if snapshot.peak_rx_bps == 0.0 && snapshot.peak_tx_bps == 0.0 {
        render_idle_unified_chart_data_uri(
            layout.chart_size,
            config.chart_window,
            resolved_theme,
            graph_style,
        )
    } else {
        render_unified_chart_data_uri(
            &snapshot.history,
            layout.chart_size,
            config.chart_window,
            resolved_theme,
            graph_style,
        )
    };

    let total_apps = snapshot.active_apps.len();
    let has_active_apps = total_apps > 0 && layout.apps_collapsed > 0;
    let page_size = layout.apps_page_size.max(1);

    let (
        start_idx,
        end_idx,
        range_label,
        has_prev_page,
        has_next_page,
        can_toggle_apps,
        toggle_apps_glyph,
        toggle_apps_icon,
        toggle_apps_title,
    ) = if config.apps_expanded {
        let total_pages = total_apps.div_ceil(page_size);
        let current_page = config.apps_page.min(total_pages.saturating_sub(1));
        let start = current_page * page_size;
        let end = (start + page_size).min(total_apps);
        let label = if total_pages > 1 {
            format!("{}–{} of {}", start + 1, end, total_apps)
        } else {
            format!("All {}", total_apps)
        };
        (
            start,
            end,
            label,
            current_page > 0,
            current_page + 1 < total_pages,
            true,
            "▴",
            icons::CHEVRON_UP,
            "Collapse",
        )
    } else {
        let count = total_apps.min(layout.apps_collapsed);
        (
            0,
            count,
            format!("Top {} of {}", count, total_apps),
            false,
            false,
            total_apps > layout.apps_collapsed,
            "▾",
            icons::CHEVRON_DOWN,
            "Show all apps",
        )
    };

    let max_slots = layout.apps_page_size;
    let mut data_map = serde_json::Map::new();
    let mut active_apps_items = Vec::new();

    for i in 0..max_slots {
        let app_idx = start_idx + i;
        if has_active_apps && app_idx < end_idx && app_idx < total_apps {
            let app = &snapshot.active_apps[app_idx];
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

            let app_name = truncate_name(&app.name, layout.apps_name_budget);
            let weight = if active { "Bolder" } else { "Default" };
            let is_subtle = !active;

            data_map.insert(format!("app{}_visible", i), json!(true));
            data_map.insert(format!("app{}_name", i), json!(app_name));
            data_map.insert(format!("app{}_icon", i), json!(icon_uri));
            data_map.insert(format!("app{}_rate", i), json!(status));
            data_map.insert(format!("app{}_weight", i), json!(weight));
            data_map.insert(format!("app{}_isSubtle", i), json!(is_subtle));

            active_apps_items.push(json!({
                "name": app_name,
                "icon": icon_uri,
                "rate": status,
                "weight": weight,
                "isSubtle": is_subtle,
            }));
        } else {
            data_map.insert(format!("app{}_visible", i), json!(false));
            data_map.insert(format!("app{}_name", i), json!(""));
            data_map.insert(format!("app{}_icon", i), json!(""));
            data_map.insert(format!("app{}_rate", i), json!(""));
            data_map.insert(format!("app{}_weight", i), json!("Default"));
            data_map.insert(format!("app{}_isSubtle", i), json!(true));
        }
    }

    let duration_str = format_duration(snapshot.session_duration_secs);
    let session_text = if snapshot.session_duration_secs > 0 {
        format!("Session ({})", duration_str)
    } else {
        "Session".to_string()
    };
    let session_tooltip = format!(
        "Reset session totals (Duration: {} • All-time peak: ↓ {}  ↑ {})",
        duration_str,
        format_bandwidth(snapshot.session_peak_rx_bps),
        format_bandwidth(snapshot.session_peak_tx_bps)
    );

    data_map.insert(
        "primaryMediumGlyph".to_string(),
        json!(medium_glyph(snapshot.primary_medium)),
    );
    data_map.insert(
        "primaryName".to_string(),
        json!(truncate_name(&snapshot.primary_name, 22)),
    );
    data_map.insert(
        "activeConnsText".to_string(),
        json!(format!("{} conns", active_count)),
    );
    data_map.insert(
        "downloadRate".to_string(),
        json!(fmt_bw(snapshot.rx_bps, config.speed_unit)),
    );
    data_map.insert(
        "downloadPeak".to_string(),
        json!(format!(
            "Peak {}",
            fmt_bw(window_peak_rx, config.speed_unit)
        )),
    );
    data_map.insert(
        "uploadRate".to_string(),
        json!(fmt_bw(snapshot.tx_bps, config.speed_unit)),
    );
    data_map.insert(
        "uploadPeak".to_string(),
        json!(format!(
            "Peak {}",
            fmt_bw(window_peak_tx, config.speed_unit)
        )),
    );
    data_map.insert("chartUrl".to_string(), json!(chart_url));
    data_map.insert("hasActiveApps".to_string(), json!(has_active_apps));
    data_map.insert("appsRangeLabel".to_string(), json!(range_label));
    data_map.insert("hasPrevPage".to_string(), json!(has_prev_page));
    data_map.insert("hasNextPage".to_string(), json!(has_next_page));
    data_map.insert("canToggleApps".to_string(), json!(can_toggle_apps));
    data_map.insert("toggleAppsGlyph".to_string(), json!(toggle_apps_glyph));
    data_map.insert("toggleAppsIcon".to_string(), json!(toggle_apps_icon));
    data_map.insert("toggleAppsTitle".to_string(), json!(toggle_apps_title));
    data_map.insert("activeApps".to_string(), json!(active_apps_items));
    data_map.insert("sessionText".to_string(), json!(session_text));
    data_map.insert("sessionTooltip".to_string(), json!(session_tooltip));
    data_map.insert(
        "sessionRx".to_string(),
        json!(format_bytes(snapshot.session_rx)),
    );
    data_map.insert(
        "sessionTx".to_string(),
        json!(format_bytes(snapshot.session_tx)),
    );

    Value::Object(data_map)
}

/// Serializes dynamic telemetry data directly to JSON string for SetData.
pub fn build_adaptive_card_data_string(
    snapshot: &NetworkSnapshot,
    config: &WidgetConfig,
    size: &str,
) -> String {
    serde_json::to_string(&build_adaptive_card_data(snapshot, config, size))
        .unwrap_or_else(|_| "{}".to_string())
}

/// Builds the default settings card template for the customization flyout.
pub fn build_settings_card(current_config: &WidgetConfig) -> String {
    build_settings_card_for_size(current_config, "Medium", 0)
}

/// Builds a size-optimized settings card template.
///
/// Small widgets receive a simplified 2-column layout to remain accessible within 160px height,
/// while Medium and Large widgets provide full speed unit and chart window controls.
pub fn build_settings_card_for_size(
    current_config: &WidgetConfig,
    size: &str,
    _session_duration_secs: u64,
) -> String {
    let (cancel_w, save_w, save_spacing) = if size == "Small" {
        (48, 42, "Small")
    } else {
        (56, 48, "Medium")
    };

    let header = json!({
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
                        "size": if size == "Small" { "Default" } else { "Medium" },
                        "wrap": false
                    }
                ]
            },
            {
                "type": "Column",
                "width": format!("{}px", cancel_w),
                "roundedCorners": true,
                "verticalContentAlignment": "Center",
                "spacing": "Small",
                "items": [
                    text_button_box(
                        "Cancel",
                        Some(cancel_w),
                        "cancel_settings",
                        "Cancel",
                        None,
                        false,
                        Some("none"),
                    )
                ]
            },
            {
                "type": "Column",
                "width": format!("{}px", save_w),
                "roundedCorners": true,
                "verticalContentAlignment": "Center",
                "spacing": save_spacing,
                "items": [
                    text_button_box(
                        "Save",
                        Some(save_w),
                        "save_settings",
                        "Save",
                        Some("Accent"),
                        true,
                        None,
                    )
                ]
            }
        ]
    });

    let mut body = vec![header];

    if size == "Small" {
        // Small widget (~160px): 2 balanced rows of 2-column compact grids
        // Row 1: Units and History
        body.push(json!({
            "type": "ColumnSet",
            "spacing": "Small",
            "columns": [
                {
                    "type": "Column",
                    "width": "stretch",
                    "items": [
                        {
                            "type": "TextBlock",
                            "text": "Units",
                            "weight": "Bolder",
                            "size": "Small",
                            "wrap": false
                        },
                        {
                            "type": "Input.ChoiceSet",
                            "id": "speed_unit",
                            "style": "compact",
                            "spacing": "None",
                            "value": current_config.speed_unit.to_str_value(),
                            "choices": [
                                { "title": "Auto", "value": "auto" },
                                { "title": "B/s", "value": "b" },
                                { "title": "KB/s", "value": "kb" },
                                { "title": "MB/s", "value": "mb" },
                                { "title": "GB/s", "value": "gb" }
                            ]
                        }
                    ]
                },
                {
                    "type": "Column",
                    "width": "stretch",
                    "spacing": "Small",
                    "items": [
                        {
                            "type": "TextBlock",
                            "text": "History",
                            "weight": "Bolder",
                            "size": "Small",
                            "wrap": false
                        },
                        {
                            "type": "Input.ChoiceSet",
                            "id": "chart_window",
                            "style": "compact",
                            "spacing": "None",
                            "value": current_config.chart_window.to_string(),
                            "choices": [
                                { "title": "15s", "value": "15" },
                                { "title": "30s", "value": "30" },
                                { "title": "60s", "value": "60" }
                            ]
                        }
                    ]
                }
            ]
        }));

        // Row 2: Theme and Graph style
        body.push(json!({
            "type": "ColumnSet",
            "spacing": "Small",
            "columns": [
                {
                    "type": "Column",
                    "width": "stretch",
                    "items": [
                        {
                            "type": "TextBlock",
                            "text": "Theme",
                            "weight": "Bolder",
                            "size": "Small",
                            "wrap": false
                        },
                        {
                            "type": "Input.ChoiceSet",
                            "id": "theme",
                            "style": "compact",
                            "spacing": "None",
                            "value": current_config.theme.to_str_value(),
                            "choices": [
                                { "title": "Auto", "value": "auto" },
                                { "title": "Dark", "value": "dark" },
                                { "title": "Light", "value": "light" }
                            ]
                        }
                    ]
                },
                {
                    "type": "Column",
                    "width": "stretch",
                    "spacing": "Small",
                    "items": [
                        {
                            "type": "TextBlock",
                            "text": "Style",
                            "weight": "Bolder",
                            "size": "Small",
                            "wrap": false
                        },
                        {
                            "type": "Input.ChoiceSet",
                            "id": "graph_style",
                            "style": "compact",
                            "spacing": "None",
                            "value": current_config.graph_style.to_str_value(),
                            "choices": [
                                { "title": "Area", "value": "area" },
                                { "title": "Line", "value": "line" },
                                { "title": "Bar", "value": "bar" }
                            ]
                        }
                    ]
                }
            ]
        }));
    } else {
        // Medium & Large (~340px): balanced rows for clear organization
        // Row 1: 2-column layout for Speed units and History window
        body.push(json!({
            "type": "ColumnSet",
            "spacing": "Small",
            "columns": [
                {
                    "type": "Column",
                    "width": "stretch",
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
                                { "title": "Auto", "value": "auto" },
                                { "title": "B/s", "value": "b" },
                                { "title": "KB/s", "value": "kb" },
                                { "title": "MB/s", "value": "mb" },
                                { "title": "GB/s", "value": "gb" }
                            ]
                        }
                    ]
                },
                {
                    "type": "Column",
                    "width": "stretch",
                    "spacing": "Medium",
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
                }
            ]
        }));

        // Row 2: 2-column layout for Theme and Graph style
        body.push(json!({
            "type": "ColumnSet",
            "spacing": "Small",
            "columns": [
                {
                    "type": "Column",
                    "width": "stretch",
                    "items": [
                        {
                            "type": "TextBlock",
                            "text": "Theme",
                            "weight": "Bolder",
                            "size": "Small",
                            "wrap": false
                        },
                        {
                            "type": "Input.ChoiceSet",
                            "id": "theme",
                            "style": "compact",
                            "spacing": "Small",
                            "value": current_config.theme.to_str_value(),
                            "choices": [
                                { "title": "Auto", "value": "auto" },
                                { "title": "Dark", "value": "dark" },
                                { "title": "Light", "value": "light" }
                            ]
                        }
                    ]
                },
                {
                    "type": "Column",
                    "width": "stretch",
                    "spacing": "Medium",
                    "items": [
                        {
                            "type": "TextBlock",
                            "text": "Graph style",
                            "weight": "Bolder",
                            "size": "Small",
                            "wrap": false
                        },
                        {
                            "type": "Input.ChoiceSet",
                            "id": "graph_style",
                            "style": "compact",
                            "spacing": "Small",
                            "value": current_config.graph_style.to_str_value(),
                            "choices": [
                                { "title": "Area (Waveform)", "value": "area" },
                                { "title": "Line (Minimal)", "value": "line" },
                                { "title": "Bar (Columns)", "value": "bar" }
                            ]
                        }
                    ]
                }
            ]
        }));

        // Row 3: Active apps section
        body.push(json!({
            "type": "TextBlock",
            "text": "Active apps",
            "weight": "Bolder",
            "size": "Small",
            "spacing": "Medium",
            "wrap": false
        }));
        body.push(json!({
            "type": "Input.Toggle",
            "id": "apps_expanded",
            "spacing": "Small",
            "title": "Expand apps",
            "value": if current_config.apps_expanded { "true" } else { "false" },
            "valueOn": "true",
            "valueOff": "false"
        }));

        // Row 4: Compact left-aligned Reset session button
        body.push(json!({
            "type": "ColumnSet",
            "spacing": "Medium",
            "columns": [
                {
                    "type": "Column",
                    "width": "auto",
                    "items": [
                        {
                            "type": "ActionSet",
                            "spacing": "None",
                            "actions": [
                                {
                                    "type": "Action.Execute",
                                    "title": "Reset session",
                                    "verb": "reset_session",
                                    "associatedInputs": "none"
                                }
                            ]
                        }
                    ]
                }
            ]
        }));
    }

    let card = json!({
        "$schema": "http://adaptivecards.io/schemas/adaptive-card.json",
        "type": "AdaptiveCard",
        "version": "1.6",
        "body": body
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
            rx_bps_500ms: 10485760,
            tx_bps_500ms: 2097152,
            instant_rx_bps: 10485760.0,
            instant_tx_bps: 2097152.0,
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
                HistorySample::from_bps(1000, 500, 500_000_000),
                HistorySample::from_bps(2000, 1000, 500_000_000),
                HistorySample::from_bps(5000, 3000, 500_000_000),
                HistorySample::from_bps(3000, 2000, 500_000_000),
                HistorySample::from_bps(8000, 4000, 500_000_000),
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
        assert!(json_str.contains(icons::SETTINGS));
        assert!(json_str.contains("open_settings"));
    }

    #[test]
    fn settings_button_is_present_on_all_card_sizes() {
        let snap = snapshot_fixture();
        for size in ["Small", "Medium", "Large"] {
            let json_str = build_adaptive_card(&snap, size, &WidgetConfig::default());
            assert!(
                json_str.contains("open_settings"),
                "Size {} missing open_settings verb",
                size
            );
            assert!(
                json_str.contains(icons::SETTINGS),
                "Size {} missing settings icon",
                size
            );
        }
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
    fn medium_card_pages_four_apps() {
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
        assert!(json_str.contains("1–4 of 13"));
        assert!(json_str.contains("App 4"));
        assert!(!json_str.contains("App 5"));
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
    fn settings_card_adapts_to_small_and_large_sizes() {
        let small_json = build_settings_card_for_size(&WidgetConfig::default(), "Small", 185);
        assert!(small_json.contains("speed_unit"));
        assert!(small_json.contains("chart_window"));
        assert!(
            !small_json.contains("apps_expanded"),
            "Small settings card must omit apps_expanded to stay minimal"
        );
        assert!(
            !small_json.contains("reset_session"),
            "Small settings card must omit reset_session to avoid clipping"
        );
        assert!(small_json.contains("save_settings"));
        assert!(small_json.contains("cancel_settings"));

        let med_json = build_settings_card_for_size(&WidgetConfig::default(), "Medium", 0);
        assert!(med_json.contains("speed_unit"));
        assert!(med_json.contains("chart_window"));
        assert!(med_json.contains("apps_expanded"));
        assert!(med_json.contains("save_settings"));
        assert!(med_json.contains("cancel_settings"));
        assert!(med_json.contains("reset_session"));
        assert!(med_json.contains("\"title\":\"Reset session\""));
    }

    #[test]
    fn widget_config_json_roundtrip() {
        let config = WidgetConfig {
            speed_unit: SpeedUnit::Megabytes,
            chart_window: 60,
            apps_expanded: false,
            apps_page: 0,
            theme: ThemeMode::Auto,
            graph_style: GraphStyle::Area,
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
        snap.history.push(HistorySample::from_bps(
            100_000_000,
            50_000_000,
            500_000_000,
        ));
        for _ in 0..10 {
            snap.history
                .push(HistorySample::from_bps(10_000, 5_000, 500_000_000));
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
    fn test_widget_config_backwards_compat() {
        // v0.1.1 JSON without theme or graph_style must deserialize seamlessly to defaults
        let old_json =
            r#"{"speed_unit":"Auto","chart_window":30,"apps_expanded":false,"apps_page":0}"#;
        let config: WidgetConfig = serde_json::from_str(old_json).unwrap();
        assert_eq!(config.theme, ThemeMode::Auto);
        assert_eq!(config.graph_style, GraphStyle::Area);

        // Deserializing with explicit theme and graph style works
        let new_json = r#"{"speed_unit":"Megabytes","chart_window":60,"apps_expanded":true,"apps_page":1,"theme":"light","graph_style":"bar"}"#;
        let config2: WidgetConfig = serde_json::from_str(new_json).unwrap();
        assert_eq!(config2.theme, ThemeMode::Light);
        assert_eq!(config2.graph_style, GraphStyle::Bar);
    }

    #[test]
    fn manifest_version_matches_cargo_pkg_version() {
        let manifest_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../widget/Package.appxmanifest");
        let content =
            std::fs::read_to_string(&manifest_path).expect("Package.appxmanifest must be readable");

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

    #[test]
    fn test_template_contains_expected_bindings() {
        for size in &["Small", "Medium", "Large"] {
            let tpl_str = build_adaptive_card_template(size);
            let val: Value = serde_json::from_str(&tpl_str)
                .unwrap_or_else(|e| panic!("template for {} must be valid json: {}", size, e));
            assert_eq!(val["type"], "AdaptiveCard");
            assert_eq!(val["version"], "1.6");

            // All templates must contain core rate bindings and chartUrl
            assert!(tpl_str.contains("${primaryMediumGlyph}"));
            assert!(tpl_str.contains("${primaryName}"));
            assert!(tpl_str.contains("${activeConnsText}"));
            assert!(tpl_str.contains("${downloadRate}"));
            assert!(tpl_str.contains("${uploadRate}"));
            assert!(tpl_str.contains("${chartUrl}"));

            if *size != "Small" {
                // Medium and Large templates must contain active apps slots and session footer
                assert!(tpl_str.contains("${app0_name}"));
                assert!(tpl_str.contains("${app0_icon}"));
                assert!(tpl_str.contains("${app0_rate}"));
                assert!(tpl_str.contains("${sessionText}"));
                assert!(tpl_str.contains("${sessionRx}"));
                assert!(tpl_str.contains("${sessionTx}"));
            }
        }
    }

    #[test]
    fn test_data_contains_all_required_keys() {
        let snap = NetworkSnapshot {
            primary_medium: crate::backend::InterfaceMedium::Wifi,
            primary_name: "Wi-Fi Test".to_string(),
            rx_bps: 12_500_000.0,
            tx_bps: 2_100_000.0,
            active_connections_count: 5,
            active_apps: vec![crate::process::ActiveAppInfo {
                name: "browser.exe".to_string(),
                process_name: "browser.exe".to_string(),
                icon: "🌐",
                icon_data_uri: None,
                rx_bps: 10_000_000.0,
                tx_bps: 1_000_000.0,
                connection_count: 4,
            }],
            session_duration_secs: 360,
            session_rx: 500_000_000,
            session_tx: 50_000_000,
            ..Default::default()
        };
        let config = WidgetConfig::default();

        for size in &["Small", "Medium", "Large"] {
            let data = build_adaptive_card_data(&snap, &config, size);
            assert!(data["primaryMediumGlyph"].is_string());
            assert!(data["primaryName"].is_string());
            assert!(data["activeConnsText"].is_string());
            assert!(data["downloadRate"].is_string());
            assert!(data["uploadRate"].is_string());
            assert!(data["chartUrl"].is_string());
            assert!(data["activeApps"].is_array());
            assert!(data["sessionText"].is_string());
            assert!(data["sessionRx"].is_string());
            assert!(data["sessionTx"].is_string());

            let json_str = build_adaptive_card_data_string(&snap, &config, size);
            assert!(!json_str.is_empty());
        }
    }

    #[test]
    fn test_compute_adaptive_ui_interval() {
        // High traffic burst: >= 250 KiB/s (256,000 B/s) -> 500 ms
        assert_eq!(
            compute_adaptive_ui_interval(300_000.0, 0.0),
            Duration::from_millis(500)
        );
        // Asymmetric upload burst: download is 0, but upload is 500 KiB/s -> 500 ms
        assert_eq!(
            compute_adaptive_ui_interval(0.0, 500_000.0),
            Duration::from_millis(500)
        );
        // Exact boundary for burst
        assert_eq!(
            compute_adaptive_ui_interval(BURST_THRESHOLD_BPS, 100.0),
            Duration::from_millis(500)
        );

        // Moderate traffic: 10 KiB/s .. 250 KiB/s -> 1000 ms
        assert_eq!(
            compute_adaptive_ui_interval(50_000.0, 10_000.0),
            Duration::from_millis(1000)
        );
        assert_eq!(
            compute_adaptive_ui_interval(0.0, MODERATE_THRESHOLD_BPS),
            Duration::from_millis(1000)
        );

        // Low / near-idle traffic: < 10 KiB/s -> 1500 ms
        assert_eq!(
            compute_adaptive_ui_interval(5_000.0, 2_000.0),
            Duration::from_millis(1500)
        );
        assert_eq!(
            compute_adaptive_ui_interval(0.0, 0.0),
            Duration::from_millis(1500)
        );
    }

    #[test]
    fn test_icon_and_text_button_dimensions_and_rounded_corners() {
        let icon_col =
            icon_button_column_spaced(icons::SETTINGS, 16, "open_settings", "Settings", "Small");
        assert_eq!(icon_col["type"], "Column");
        assert_eq!(icon_col["width"], "28px");
        assert_eq!(icon_col["roundedCorners"], true);
        let inner_container = &icon_col["items"][0];
        assert_eq!(inner_container["type"], "Container");
        assert_eq!(inner_container["minHeight"], "28px");
        assert_eq!(inner_container["roundedCorners"], true);

        let icon_when_col = icon_button_column_when(
            icons::CHEVRON_LEFT,
            16,
            "prev",
            "Prev",
            "${hasPrev == true}",
            "Small",
        );
        assert_eq!(icon_when_col["width"], "28px");
        assert_eq!(icon_when_col["roundedCorners"], true);

        // Verify Settings card header has explicit column widths and rounded corners
        let settings_medium = build_settings_card_for_size(&WidgetConfig::default(), "Medium", 0);
        let parsed_med: Value = serde_json::from_str(&settings_medium).unwrap();
        let header_cols = &parsed_med["body"][0]["columns"];
        let cancel_col = &header_cols[1];
        let save_col = &header_cols[2];
        assert_eq!(cancel_col["width"], "56px");
        assert_eq!(cancel_col["roundedCorners"], true);
        assert_eq!(save_col["width"], "48px");
        assert_eq!(save_col["roundedCorners"], true);
        assert_eq!(save_col["spacing"], "Medium");

        let settings_small = build_settings_card_for_size(&WidgetConfig::default(), "Small", 0);
        let parsed_small: Value = serde_json::from_str(&settings_small).unwrap();
        let small_header_cols = &parsed_small["body"][0]["columns"];
        assert_eq!(small_header_cols[1]["width"], "48px");
        assert_eq!(small_header_cols[2]["width"], "42px");
    }

    #[test]
    fn test_metrics_row_upload_right_alignment() {
        for size in &["Small", "Medium", "Large"] {
            let tpl_str = build_adaptive_card_template(size);
            let val: Value = serde_json::from_str(&tpl_str).unwrap();
            let metrics_row = &val["body"][1];
            assert_eq!(metrics_row["type"], "ColumnSet");

            let upload_col = &metrics_row["columns"][1];
            let upload_items = upload_col["items"].as_array().unwrap();

            // Item 0: ColumnSet with empty stretch spacer pushing Upload to the right
            let header_colset = &upload_items[0];
            let header_cols = header_colset["columns"].as_array().unwrap();
            assert_eq!(header_cols[0]["width"], "stretch");
            assert_eq!(header_cols[2]["items"][0]["horizontalAlignment"], "Right");

            // Item 1: ${uploadRate} TextBlock with horizontalAlignment: Right
            assert_eq!(upload_items[1]["text"], "${uploadRate}");
            assert_eq!(upload_items[1]["horizontalAlignment"], "Right");

            // Item 2: ${uploadPeak} TextBlock with horizontalAlignment: Right
            assert_eq!(upload_items[2]["text"], "${uploadPeak}");
            assert_eq!(upload_items[2]["horizontalAlignment"], "Right");
        }

        // Test non-template fallback card (build_card)
        let snap = snapshot_fixture();
        let card_json = build_adaptive_card(&snap, "Medium", &WidgetConfig::default());
        let val: Value = serde_json::from_str(&card_json).unwrap();
        let metrics_row = &val["body"][1];
        let upload_col = &metrics_row["columns"][1];
        let upload_items = upload_col["items"].as_array().unwrap();
        assert_eq!(upload_items[1]["horizontalAlignment"], "Right");
        assert_eq!(upload_items[2]["horizontalAlignment"], "Right");
    }
}
