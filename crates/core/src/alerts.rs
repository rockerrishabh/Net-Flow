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
    /// Download threshold in bytes per second (legacy alias: threshold_bps).
    #[serde(default = "default_threshold_bps", alias = "download_threshold_bps")]
    pub threshold_bps: u64,
    /// Separate upload threshold in bytes per second. If None, uses threshold_bps.
    #[serde(default)]
    pub upload_threshold_bps: Option<u64>,
    /// Download sustain duration in seconds (legacy alias: sustain_secs).
    #[serde(default = "default_sustain_secs", alias = "download_sustain_secs")]
    pub sustain_secs: u32,
    /// Separate upload sustain duration in seconds. If None, uses sustain_secs.
    #[serde(default)]
    pub upload_sustain_secs: Option<u32>,
    /// Minimum time between toasts for download alerts.
    #[serde(default = "default_cooldown_secs", alias = "download_cooldown_secs")]
    pub cooldown_secs: u32,
    /// Separate upload cooldown in seconds. If None, uses cooldown_secs.
    #[serde(default)]
    pub upload_cooldown_secs: Option<u32>,
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
            upload_threshold_bps: None,
            sustain_secs: default_sustain_secs(),
            upload_sustain_secs: None,
            cooldown_secs: default_cooldown_secs(),
            upload_cooldown_secs: None,
            download_enabled: true,
            upload_enabled: false,
        }
    }
}

impl BandwidthAlertConfig {
    pub fn download_threshold(&self) -> u64 {
        self.threshold_bps
    }

    pub fn upload_threshold(&self) -> u64 {
        self.upload_threshold_bps.unwrap_or(self.threshold_bps)
    }

    pub fn download_sustain(&self) -> u32 {
        self.sustain_secs
    }

    pub fn upload_sustain(&self) -> u32 {
        self.upload_sustain_secs.unwrap_or(self.sustain_secs)
    }

    pub fn download_cooldown(&self) -> u32 {
        self.cooldown_secs
    }

    pub fn upload_cooldown(&self) -> u32 {
        self.upload_cooldown_secs.unwrap_or(self.cooldown_secs)
    }

    pub fn normalized(mut self) -> Self {
        self.threshold_bps = self.threshold_bps.max(1);
        if let Some(up) = self.upload_threshold_bps {
            self.upload_threshold_bps = Some(up.max(1));
        }
        self.sustain_secs = self.sustain_secs.clamp(1, 3_600);
        if let Some(up_sustain) = self.upload_sustain_secs {
            self.upload_sustain_secs = Some(up_sustain.clamp(1, 3_600));
        }
        self.cooldown_secs = self.cooldown_secs.clamp(1, 86_400);
        if let Some(up_cooldown) = self.upload_cooldown_secs {
            self.upload_cooldown_secs = Some(up_cooldown.clamp(1, 86_400));
        }
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
    if let Ok(json) = serde_json::to_string_pretty(&config.clone().normalized()) {
        let _ = crate::backend::write_atomic(&path, json.as_bytes());
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
                config.download_threshold(),
                config.download_sustain(),
                config.download_cooldown(),
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
                config.upload_threshold(),
                config.upload_sustain(),
                config.upload_cooldown(),
                now,
                &mut self.upload_since,
                &mut self.last_upload_alert,
            )
        {
            events.push(event);
        }
        events
    }

    #[allow(clippy::too_many_arguments)]
    fn evaluate_direction(
        direction: AlertDirection,
        observed_bps: f64,
        threshold_bps: u64,
        sustain_secs: u32,
        cooldown_secs: u32,
        now: Instant,
        above_since: &mut Option<Instant>,
        last_alert: &mut Option<Instant>,
    ) -> Option<AlertEvent> {
        if !observed_bps.is_finite() || observed_bps < threshold_bps as f64 {
            *above_since = None;
            return None;
        }
        let sustained_since = *above_since.get_or_insert(now);
        if now.saturating_duration_since(sustained_since)
            < Duration::from_secs(sustain_secs.into())
        {
            return None;
        }
        if last_alert.is_some_and(|last| {
            now.saturating_duration_since(last) < Duration::from_secs(cooldown_secs.into())
        }) {
            return None;
        }
        *last_alert = Some(now);
        Some(AlertEvent {
            direction,
            observed_bps: observed_bps.max(0.0) as u64,
            threshold_bps,
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

    #[test]
    fn test_independent_download_and_upload_alert_state_machines() {
        let mut engine = BandwidthAlertEngine::default();
        let config = BandwidthAlertConfig {
            enabled: true,
            download_enabled: true,
            threshold_bps: 1000,
            sustain_secs: 5,
            cooldown_secs: 60,
            upload_enabled: true,
            upload_threshold_bps: Some(500),
            upload_sustain_secs: Some(10),
            upload_cooldown_secs: Some(30),
        };
        let start = Instant::now();

        // DL is 1200 (>1000), UL is 600 (>500)
        assert!(engine.evaluate(&config, 1200.0, 600.0, start).is_empty());

        // At +5s: DL triggers (sustain is 5s), but UL has not triggered yet (sustain is 10s)
        let events_at_5s = engine.evaluate(&config, 1200.0, 600.0, start + Duration::from_secs(5));
        assert_eq!(events_at_5s.len(), 1);
        assert_eq!(events_at_5s[0].direction, AlertDirection::Download);

        // At +10s: UL triggers (sustain is 10s), while DL is in cooldown
        let events_at_10s = engine.evaluate(&config, 1200.0, 600.0, start + Duration::from_secs(10));
        assert_eq!(events_at_10s.len(), 1);
        assert_eq!(events_at_10s[0].direction, AlertDirection::Upload);
    }
}
