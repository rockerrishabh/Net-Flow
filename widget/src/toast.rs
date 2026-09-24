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
