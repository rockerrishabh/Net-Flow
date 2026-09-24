//! Persistent bandwidth alert configuration and sustained-threshold evaluation.
//!
//! The engine is platform independent. The widget host turns emitted events into
//! Windows toast notifications, which keeps the timing logic deterministic and testable.

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Direction monitored by a bandwidth alert.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertDirection {
    Download,
    Upload,
}

/// User-owned alert preferences persisted outside the MSIX package.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BandwidthAlertConfig {
    #[serde(default)]
    pub enabled: bool,
    /// Threshold in bytes per second. 100 MiB/s is intentionally the useful default.
    #[serde(default = "default_threshold_bps")]
    pub threshold_bps: u64,
    /// A threshold must remain exceeded for this long before a notification is sent.
    #[serde(default = "default_sustain_secs")]
    pub sustain_secs: u32,
    /// Minimum time between toasts for the same direction.
    #[serde(default = "default_cooldown_secs")]
    pub cooldown_secs: u32,
    #[serde(default = "default_download_enabled")]
    pub download_enabled: bool,
    #[serde(default)]
    pub upload_enabled: bool,
}

const fn default_threshold_bps() -> u64 {
    100 * 1024 * 1024
}
const fn default_sustain_secs() -> u32 {
    10
}
const fn default_cooldown_secs() -> u32 {
    60
}
const fn default_download_enabled() -> bool {
    true
}

impl Default for BandwidthAlertConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            threshold_bps: default_threshold_bps(),
            sustain_secs: default_sustain_secs(),
            cooldown_secs: default_cooldown_secs(),
            download_enabled: true,
            upload_enabled: false,
        }
    }
}

impl BandwidthAlertConfig {
    pub fn normalized(mut self) -> Self {
        self.threshold_bps = self.threshold_bps.max(1);
        self.sustain_secs = self.sustain_secs.clamp(1, 3_600);
        self.cooldown_secs = self.cooldown_secs.clamp(1, 86_400);
        self
    }
}

pub fn get_alert_config_path() -> std::path::PathBuf {
    let base = std::env::var("LOCALAPPDATA")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir());
    base.join("NetFlow").join("alerts.json")
}

pub fn load_alert_config() -> BandwidthAlertConfig {
    std::fs::read_to_string(get_alert_config_path())
        .ok()
        .and_then(|json| serde_json::from_str(&json).ok())
        .map(BandwidthAlertConfig::normalized)
        .unwrap_or_default()
}

pub fn save_alert_config(config: &BandwidthAlertConfig) {
    let path = get_alert_config_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(&config.clone().normalized()) {
        let _ = std::fs::write(path, json);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertEvent {
    pub direction: AlertDirection,
    pub observed_bps: u64,
    pub threshold_bps: u64,
}

/// Detects sustained threshold crossings and rate-limits duplicate notifications.
#[derive(Debug, Default)]
pub struct BandwidthAlertEngine {
    download_since: Option<Instant>,
    upload_since: Option<Instant>,
    last_download_alert: Option<Instant>,
    last_upload_alert: Option<Instant>,
}

impl BandwidthAlertEngine {
    pub fn evaluate(
        &mut self,
        config: &BandwidthAlertConfig,
        download_bps: f64,
        upload_bps: f64,
        now: Instant,
    ) -> Vec<AlertEvent> {
        let config = config.clone().normalized();
        if !config.enabled {
            self.download_since = None;
            self.upload_since = None;
            return Vec::new();
        }

        let mut events = Vec::new();
        if config.download_enabled
            && let Some(event) = Self::evaluate_direction(
                AlertDirection::Download,
                download_bps,
                &config,
                now,
                &mut self.download_since,
                &mut self.last_download_alert,
            )
        {
            events.push(event);
        }
        if config.upload_enabled
            && let Some(event) = Self::evaluate_direction(
                AlertDirection::Upload,
                upload_bps,
                &config,
                now,
                &mut self.upload_since,
                &mut self.last_upload_alert,
            )
        {
            events.push(event);
        }
        events
    }

    fn evaluate_direction(
        direction: AlertDirection,
        observed_bps: f64,
        config: &BandwidthAlertConfig,
        now: Instant,
        above_since: &mut Option<Instant>,
        last_alert: &mut Option<Instant>,
    ) -> Option<AlertEvent> {
        if !observed_bps.is_finite() || observed_bps < config.threshold_bps as f64 {
            *above_since = None;
            return None;
        }
        let sustained_since = *above_since.get_or_insert(now);
        if now.saturating_duration_since(sustained_since)
            < Duration::from_secs(config.sustain_secs.into())
        {
            return None;
        }
        if last_alert.is_some_and(|last| {
            now.saturating_duration_since(last) < Duration::from_secs(config.cooldown_secs.into())
        }) {
            return None;
        }
        *last_alert = Some(now);
        Some(AlertEvent {
            direction,
            observed_bps: observed_bps.max(0.0) as u64,
            threshold_bps: config.threshold_bps,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_alerts_after_the_threshold_is_sustained() {
        let mut engine = BandwidthAlertEngine::default();
        let config = BandwidthAlertConfig {
            enabled: true,
            threshold_bps: 100,
            sustain_secs: 10,
            cooldown_secs: 60,
            ..Default::default()
        };
        let start = Instant::now();
        assert!(engine.evaluate(&config, 101.0, 0.0, start).is_empty());
        assert!(
            engine
                .evaluate(&config, 101.0, 0.0, start + Duration::from_secs(9))
                .is_empty()
        );
        assert_eq!(
            engine
                .evaluate(&config, 101.0, 0.0, start + Duration::from_secs(10))
                .len(),
            1
        );
        assert!(
            engine
                .evaluate(&config, 101.0, 0.0, start + Duration::from_secs(11))
                .is_empty()
        );
    }

    #[test]
    fn a_dip_resets_the_sustain_window() {
        let mut engine = BandwidthAlertEngine::default();
        let config = BandwidthAlertConfig {
            enabled: true,
            threshold_bps: 100,
            sustain_secs: 10,
            cooldown_secs: 60,
            ..Default::default()
        };
        let start = Instant::now();
        engine.evaluate(&config, 101.0, 0.0, start);
        assert!(
            engine
                .evaluate(&config, 99.0, 0.0, start + Duration::from_secs(8))
                .is_empty()
        );
        engine.evaluate(&config, 101.0, 0.0, start + Duration::from_secs(9));
        assert!(
            engine
                .evaluate(&config, 101.0, 0.0, start + Duration::from_secs(18))
                .is_empty()
        );
        assert_eq!(
            engine
                .evaluate(&config, 101.0, 0.0, start + Duration::from_secs(19))
                .len(),
            1
        );
    }
}
