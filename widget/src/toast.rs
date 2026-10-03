//! Packaged-app Windows toast delivery for sustained bandwidth alerts.

use net_flow_core::{AlertDirection, AlertEvent, format_bandwidth};
use windows::Data::Xml::Dom::XmlDocument;
use windows::UI::Notifications::{ToastNotification, ToastNotificationManager};
use windows::core::HSTRING;

pub fn show_bandwidth_alert(event: AlertEvent) -> windows::core::Result<()> {
    let (direction, glyph) = match event.direction {
        AlertDirection::Download => ("Download", "↓"),
        AlertDirection::Upload => ("Upload", "↑"),
    };
    let title = format!("Net Flow: high {direction} usage");
    let body = format!(
        "{glyph} {} has remained above {} for the configured duration.",
        format_bandwidth(event.observed_bps as f64),
        format_bandwidth(event.threshold_bps as f64),
    );
    let xml = XmlDocument::new()?;
    xml.LoadXml(&HSTRING::from(format!(
        "<toast><visual><binding template=\"ToastGeneric\"><text>{}</text><text>{}</text></binding></visual></toast>",
        escape_xml(&title),
        escape_xml(&body),
    )))?;
    let toast = ToastNotification::CreateToastNotification(&xml)?;
    // Net Flow is an MSIX-packaged application, so its package identity is used
    // automatically rather than requiring an unpackaged-app shortcut/AppUserModelID.
    ToastNotificationManager::CreateToastNotifier()?.Show(&toast)
}

pub fn show_budget_alert(
    milestone: net_flow_core::budget::BudgetMilestone,
) -> windows::core::Result<()> {
    let (icon, title_suffix) = match milestone.tier {
        net_flow_core::budget::MilestoneTier::Percent80 => ("⚠️", "approaching monthly cap (80%)"),
        net_flow_core::budget::MilestoneTier::Percent90 => ("⚠️", "near monthly cap (90%)"),
        net_flow_core::budget::MilestoneTier::Percent100 => {
            ("🚨", "monthly data cap reached (100%)")
        }
    };
    let title = format!("Net Flow: {icon} {title_suffix}");
    let days_label = if milestone.days_remaining == 1 {
        "1 day remaining in billing cycle.".to_string()
    } else {
        format!(
            "{} days remaining in billing cycle.",
            milestone.days_remaining
        )
    };
    let body = format!(
        "Used {} of {} ({}%). {}",
        net_flow_core::format_bytes(milestone.consumed_bytes),
        net_flow_core::format_bytes(milestone.cap_bytes),
        milestone.usage_pct,
        days_label
    );
    let xml = XmlDocument::new()?;
    xml.LoadXml(&HSTRING::from(format!(
        "<toast><visual><binding template=\"ToastGeneric\"><text>{}</text><text>{}</text></binding></visual></toast>",
        escape_xml(&title),
        escape_xml(&body),
    )))?;
    let toast = ToastNotification::CreateToastNotification(&xml)?;
    ToastNotificationManager::CreateToastNotifier()?.Show(&toast)
}

pub fn show_export_complete_toast(
    path: &std::path::Path,
    token: &str,
) -> windows::core::Result<()> {
    let filename = path
        .file_name()
        .map(|f| f.to_string_lossy().into_owned())
        .unwrap_or_else(|| "report".to_string());
    let title = "Net Flow: Diagnostic report exported";
    let body = format!("Saved {} successfully.", filename);
    let launch_args = format!("action=open-file&token={}", token);

    let xml = XmlDocument::new()?;
    xml.LoadXml(&HSTRING::from(format!(
        "<toast activationType=\"foreground\" launch=\"{}\">\
            <visual>\
                <binding template=\"ToastGeneric\">\
                    <text>{}</text>\
                    <text>{}</text>\
                </binding>\
            </visual>\
            <actions>\
                <action content=\"Open Report\" arguments=\"{}\" activationType=\"foreground\"/>\
            </actions>\
        </toast>",
        escape_xml(&launch_args),
        escape_xml(title),
        escape_xml(&body),
        escape_xml(&launch_args),
    )))?;
    let toast = ToastNotification::CreateToastNotification(&xml)?;
    ToastNotificationManager::CreateToastNotifier()?.Show(&toast)
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_toast_content() {
        assert_eq!(escape_xml("A & B < C"), "A &amp; B &lt; C");
    }
}
