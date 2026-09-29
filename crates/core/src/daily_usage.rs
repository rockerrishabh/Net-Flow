//! Bounded day-keyed daily usage history and milestone notification tracking.
//!
//! Maintains a strictly sorted, unique-date time-series of up to 90 calendar days
//! tracking cumulative byte consumption (`rx_bytes` and `tx_bytes`).

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Maximum number of daily usage entries retained on disk (90 days).
pub const MAX_DAILY_ENTRIES: usize = 90;

/// Network usage recorded for a single local calendar day.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyUsageEntry {
    /// Local calendar date in "YYYY-MM-DD" format.
    pub date: String,
    /// Cumulative received bytes for this day.
    pub rx_bytes: u64,
    /// Cumulative transmitted bytes for this day.
    pub tx_bytes: u64,
}

/// Persistent milestone notification tracking for the active billing cycle.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneState {
    /// Date boundary identifying the active billing cycle ("YYYY-MM-DD").
    #[serde(default)]
    pub cycle_start: String,
    /// Whether the 80% quota warning has been fired in this cycle.
    #[serde(default)]
    pub notified_80: bool,
    /// Whether the 90% quota warning has been fired in this cycle.
    #[serde(default)]
    pub notified_90: bool,
    /// Whether the 100% quota warning has been fired in this cycle.
    #[serde(default)]
    pub notified_100: bool,
}

/// Day-keyed bounded usage store with automatic retention pruning.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyUsageStore {
    /// Bounded entries, sorted strictly ascending by date string ("YYYY-MM-DD").
    #[serde(default)]
    pub entries: Vec<DailyUsageEntry>,
    /// Persisted milestone alert states for the active billing cycle.
    #[serde(default)]
    pub notified_milestones: MilestoneState,
}

impl DailyUsageStore {
    /// Creates an empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records an incremental byte delta into the store for the specified date.
    ///
    /// Invariants maintained:
    /// 1. Exactly one entry per calendar date.
    /// 2. Entries strictly sorted ascending by date string.
    /// 3. Maximum 90 entries (oldest entries pruned when capacity exceeded).
    pub fn record_usage_delta(&mut self, date: &str, rx_delta: u64, tx_delta: u64) {
        if date.is_empty() || (rx_delta == 0 && tx_delta == 0) {
            return;
        }

        match self.entries.binary_search_by(|e| e.date.as_str().cmp(date)) {
            Ok(idx) => {
                let entry = &mut self.entries[idx];
                entry.rx_bytes = entry.rx_bytes.saturating_add(rx_delta);
                entry.tx_bytes = entry.tx_bytes.saturating_add(tx_delta);
            }
            Err(insert_idx) => {
                self.entries.insert(
                    insert_idx,
                    DailyUsageEntry {
                        date: date.to_string(),
                        rx_bytes: rx_delta,
                        tx_bytes: tx_delta,
                    },
                );
            }
        }

        self.prune();
    }

    /// Enforces the maximum 90-entry retention boundary by pruning oldest records.
    pub fn prune(&mut self) {
        if self.entries.len() > MAX_DAILY_ENTRIES {
            let overflow = self.entries.len() - MAX_DAILY_ENTRIES;
            self.entries.drain(0..overflow);
        }
    }

    /// Retrieves usage record for the specified date, if present.
    pub fn get_entry(&self, date: &str) -> Option<&DailyUsageEntry> {
        self.entries
            .binary_search_by(|e| e.date.as_str().cmp(date))
            .ok()
            .map(|idx| &self.entries[idx])
    }

    /// Computes the total received and transmitted bytes within the date interval
    /// `[start_date, end_date)` (start inclusive, end exclusive).
    pub fn usage_between(&self, start_date: &str, end_date: &str) -> (u64, u64) {
        let mut total_rx = 0u64;
        let mut total_tx = 0u64;

        for entry in &self.entries {
            if entry.date.as_str() >= start_date && entry.date.as_str() < end_date {
                total_rx = total_rx.saturating_add(entry.rx_bytes);
                total_tx = total_tx.saturating_add(entry.tx_bytes);
            }
        }

        (total_rx, total_tx)
    }

    /// Enforces store invariants: sorts by date, coalesces duplicate dates,
    /// and prunes to MAX_DAILY_ENTRIES.
    pub fn sanitize(&mut self) {
        self.entries.sort_by(|a, b| a.date.cmp(&b.date));

        let mut deduplicated: Vec<DailyUsageEntry> = Vec::with_capacity(self.entries.len());
        for entry in self.entries.drain(..) {
            if let Some(last) = deduplicated.last_mut()
                && last.date == entry.date
            {
                last.rx_bytes = last.rx_bytes.saturating_add(entry.rx_bytes);
                last.tx_bytes = last.tx_bytes.saturating_add(entry.tx_bytes);
                continue;
            }
            deduplicated.push(entry);
        }

        self.entries = deduplicated;
        self.prune();
    }
}

/// Returns the filesystem path to `%LOCALAPPDATA%\NetFlow\daily_usage.json`.
pub fn get_daily_usage_path() -> PathBuf {
    let base = std::env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir());
    base.join("NetFlow").join("daily_usage.json")
}

/// Loads and sanitizes the daily usage store from disk, or returns default on failure.
pub fn load_daily_usage() -> DailyUsageStore {
    let path = get_daily_usage_path();
    let mut store = std::fs::read_to_string(path)
        .ok()
        .and_then(|json| serde_json::from_str::<DailyUsageStore>(&json).ok())
        .unwrap_or_default();
    store.sanitize();
    store
}

/// Atomically persists the daily usage store to disk using a process-unique temp file.
pub fn save_daily_usage(store: &DailyUsageStore) {
    let path = get_daily_usage_path();
    if let Ok(json) = serde_json::to_string_pretty(store) {
        let _ = crate::backend::write_atomic(&path, json.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_and_search() {
        let mut store = DailyUsageStore::new();
        store.record_usage_delta("2026-09-28", 100, 50);
        store.record_usage_delta("2026-09-29", 200, 100);
        store.record_usage_delta("2026-09-28", 50, 25);

        assert_eq!(store.entries.len(), 2);
        let e28 = store.get_entry("2026-09-28").unwrap();
        assert_eq!(e28.rx_bytes, 150);
        assert_eq!(e28.tx_bytes, 75);

        let e29 = store.get_entry("2026-09-29").unwrap();
        assert_eq!(e29.rx_bytes, 200);
        assert_eq!(e29.tx_bytes, 100);
    }

    #[test]
    fn test_sorted_insertion() {
        let mut store = DailyUsageStore::new();
        store.record_usage_delta("2026-09-30", 10, 10);
        store.record_usage_delta("2026-09-28", 20, 20);
        store.record_usage_delta("2026-09-29", 30, 30);

        assert_eq!(store.entries[0].date, "2026-09-28");
        assert_eq!(store.entries[1].date, "2026-09-29");
        assert_eq!(store.entries[2].date, "2026-09-30");
    }

    #[test]
    fn test_retention_pruning() {
        let mut store = DailyUsageStore::new();
        for day in 1..=100 {
            let date = format!("2026-01-{:03}", day);
            store.record_usage_delta(&date, 10, 10);
        }

        assert_eq!(store.entries.len(), MAX_DAILY_ENTRIES);
        assert_eq!(store.entries[0].date, "2026-01-011");
        assert_eq!(store.entries.last().unwrap().date, "2026-01-100");
    }

    #[test]
    fn test_usage_between_range() {
        let mut store = DailyUsageStore::new();
        store.record_usage_delta("2026-09-01", 100, 10);
        store.record_usage_delta("2026-09-15", 200, 20);
        store.record_usage_delta("2026-09-30", 300, 30);
        store.record_usage_delta("2026-10-01", 400, 40);

        // [2026-09-01, 2026-10-01) -> includes Sep 1, 15, 30, excludes Oct 1
        let (rx, tx) = store.usage_between("2026-09-01", "2026-10-01");
        assert_eq!(rx, 600);
        assert_eq!(tx, 60);
    }

    #[test]
    fn test_sanitize_coalesces_duplicates() {
        let mut store = DailyUsageStore {
            entries: vec![
                DailyUsageEntry {
                    date: "2026-09-29".to_string(),
                    rx_bytes: 100,
                    tx_bytes: 50,
                },
                DailyUsageEntry {
                    date: "2026-09-28".to_string(),
                    rx_bytes: 10,
                    tx_bytes: 5,
                },
                DailyUsageEntry {
                    date: "2026-09-29".to_string(),
                    rx_bytes: 200,
                    tx_bytes: 70,
                },
            ],
            notified_milestones: MilestoneState::default(),
        };

        store.sanitize();
        assert_eq!(store.entries.len(), 2);
        assert_eq!(store.entries[0].date, "2026-09-28");
        assert_eq!(store.entries[1].date, "2026-09-29");
        assert_eq!(store.entries[1].rx_bytes, 300);
        assert_eq!(store.entries[1].tx_bytes, 120);
    }
}
