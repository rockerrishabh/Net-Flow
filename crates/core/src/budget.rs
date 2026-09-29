//! Data budgeting, monthly quota calculation, billing cycle boundaries, and milestone alerts.
//!
//! Provides deterministic date arithmetic for billing cycle boundaries (clamping renewal
//! days across varying month lengths and leap years), usage rollups, percentage evaluations,
//! and milestone threshold triggers (80%, 90%, 100%).

use serde::{Deserialize, Serialize};

use crate::daily_usage::{DailyUsageStore, MilestoneState};
use crate::format::format_bytes;

/// Directional scope of traffic counted toward the monthly quota.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BudgetScope {
    /// Count both download and upload (default).
    #[default]
    Combined,
    /// Count download only.
    DownloadOnly,
}

impl BudgetScope {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Combined => "Download + Upload",
            Self::DownloadOnly => "Download only",
        }
    }
}

/// User configuration for data budgeting and quota warning toasts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataBudgetConfig {
    /// Whether data quota monitoring is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Monthly quota allowance in bytes (None or 0 = unlimited).
    #[serde(default)]
    pub monthly_cap_bytes: Option<u64>,
    /// Day of month the billing cycle renews (1..=31). Clamped to month length.
    #[serde(default = "default_renewal_day")]
    pub renewal_day: u8,
    /// Scope of traffic counted towards the quota.
    #[serde(default)]
    pub scope: BudgetScope,
    /// Toast notification when usage crosses 80%.
    #[serde(default = "default_true")]
    pub notify_80: bool,
    /// Toast notification when usage crosses 90%.
    #[serde(default = "default_true")]
    pub notify_90: bool,
    /// Toast notification when usage reaches or exceeds 100%.
    #[serde(default = "default_true")]
    pub notify_100: bool,
}

const fn default_renewal_day() -> u8 {
    1
}

const fn default_true() -> bool {
    true
}

impl Default for DataBudgetConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            monthly_cap_bytes: None,
            renewal_day: default_renewal_day(),
            scope: BudgetScope::default(),
            notify_80: default_true(),
            notify_90: default_true(),
            notify_100: default_true(),
        }
    }
}

impl DataBudgetConfig {
    /// Normalizes configuration values within reasonable bounds.
    pub fn normalized(mut self) -> Self {
        self.renewal_day = self.renewal_day.clamp(1, 31);
        self
    }
}

/// Milestone tier triggered when crossing budget quota thresholds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MilestoneTier {
    Percent80,
    Percent90,
    Percent100,
}

impl MilestoneTier {
    pub fn threshold_pct(&self) -> u16 {
        match self {
            Self::Percent80 => 80,
            Self::Percent90 => 90,
            Self::Percent100 => 100,
        }
    }
}

/// Details of a crossed quota milestone ready to be delivered to the user.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetMilestone {
    pub tier: MilestoneTier,
    pub consumed_bytes: u64,
    pub cap_bytes: u64,
    pub days_remaining: u32,
    pub usage_pct: u16,
}

/// Evaluated data budget snapshot for UI presentation and alerts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetSnapshot {
    pub consumed_bytes: u64,
    pub cap_bytes: u64,
    pub usage_pct: u16,
    pub days_remaining: u32,
    pub scope: BudgetScope,
    pub is_over_budget: bool,
    pub cycle_start: String,
    pub cycle_end: String,
    /// Emitted once when a milestone threshold is crossed during this cycle.
    pub milestone_to_notify: Option<BudgetMilestone>,
}

/// Checks whether a year is a leap year in the Gregorian calendar.
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Returns the number of days in the specified month of a given year (1-12).
pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Clamps the configured renewal day to the number of days in the month.
pub fn clamped_renewal_day(year: i32, month: u32, renewal_day: u8) -> u32 {
    (renewal_day as u32).clamp(1, days_in_month(year, month))
}

/// Converts a calendar date to astronomical Julian Day Number (JDN) for exact day delta arithmetic.
pub fn date_to_days(year: i32, month: u32, day: u32) -> i64 {
    let (y, m) = if month <= 2 {
        (year - 1, month + 9)
    } else {
        (year, month - 3)
    };
    let c = (y / 100) as i64;
    let ya = (y - 100 * (y / 100)) as i64;
    (146097 * c) / 4 + (1461 * ya) / 4 + (153 * (m as i64) + 2) / 5 + day as i64 + 1721119
}

/// Formats a year, month, and day into "YYYY-MM-DD".
pub fn format_ymd(year: i32, month: u32, day: u32) -> String {
    format!("{:04}-{:02}-{:02}", year, month, day)
}

/// Computes formal billing cycle boundaries given today's calendar date and configured renewal day.
///
/// Returns `(cycle_start_ymd, cycle_end_ymd, days_remaining)`.
///
/// Invariants:
/// - `cycle_start`: latest occurrence of renewal date `<= today`
/// - `cycle_end`: next occurrence of renewal date `> cycle_start`
/// - `days_remaining`: `(cycle_end - today).days`
pub fn calculate_cycle_boundaries(
    today_year: i32,
    today_month: u32,
    today_day: u32,
    renewal_day: u8,
) -> ((i32, u32, u32), (i32, u32, u32), u32) {
    let curr_renew = clamped_renewal_day(today_year, today_month, renewal_day);

    let (start_y, start_m, start_d, end_y, end_m, end_d) = if today_day >= curr_renew {
        // Today is on or after the renewal day of this month
        let start = (today_year, today_month, curr_renew);

        let (next_y, next_m) = if today_month == 12 {
            (today_year + 1, 1)
        } else {
            (today_year, today_month + 1)
        };
        let next_renew = clamped_renewal_day(next_y, next_m, renewal_day);
        let end = (next_y, next_m, next_renew);

        (start.0, start.1, start.2, end.0, end.1, end.2)
    } else {
        // Today is before the renewal day of this month -> cycle started last month
        let (prev_y, prev_m) = if today_month == 1 {
            (today_year - 1, 12)
        } else {
            (today_year, today_month - 1)
        };
        let prev_renew = clamped_renewal_day(prev_y, prev_m, renewal_day);
        let start = (prev_y, prev_m, prev_renew);
        let end = (today_year, today_month, curr_renew);

        (start.0, start.1, start.2, end.0, end.1, end.2)
    };

    let today_days = date_to_days(today_year, today_month, today_day);
    let end_days = date_to_days(end_y, end_m, end_d);
    let days_remaining = (end_days - today_days).max(0) as u32;

    (
        (start_y, start_m, start_d),
        (end_y, end_m, end_d),
        days_remaining,
    )
}

/// Retrieves the current local calendar date (year, month, day) from the Windows operating system.
pub fn current_local_ymd() -> (i32, u32, u32) {
    #[repr(C)]
    struct Win32SystemTime {
        year: u16,
        month: u16,
        day_of_week: u16,
        day: u16,
        hour: u16,
        minute: u16,
        second: u16,
        milliseconds: u16,
    }

    unsafe {
        #[link(name = "kernel32")]
        unsafe extern "system" {
            fn GetLocalTime(lpSystemTime: *mut Win32SystemTime);
        }
        let mut st = std::mem::zeroed();
        GetLocalTime(&mut st);
        (st.year as i32, st.month as u32, st.day as u32)
    }
}

/// Evaluates usage against configured data budget for the given date.
///
/// Updates `store.notified_milestones` if milestone thresholds are crossed or if the cycle rolls over.
pub fn calculate_budget_snapshot(
    config: &DataBudgetConfig,
    store: &mut DailyUsageStore,
    today_ymd: (i32, u32, u32),
) -> BudgetSnapshot {
    let (start_ymd, end_ymd, days_remaining) =
        calculate_cycle_boundaries(today_ymd.0, today_ymd.1, today_ymd.2, config.renewal_day);

    let cycle_start = format_ymd(start_ymd.0, start_ymd.1, start_ymd.2);
    let cycle_end = format_ymd(end_ymd.0, end_ymd.1, end_ymd.2);

    // 1. Reset milestone notifications if billing cycle rolled over
    if store.notified_milestones.cycle_start != cycle_start {
        store.notified_milestones = MilestoneState {
            cycle_start: cycle_start.clone(),
            notified_80: false,
            notified_90: false,
            notified_100: false,
        };
    }

    // 2. Sum consumed bytes for the active cycle: [cycle_start, cycle_end)
    let (rx, tx) = store.usage_between(&cycle_start, &cycle_end);
    let consumed_bytes = match config.scope {
        BudgetScope::Combined => rx.saturating_add(tx),
        BudgetScope::DownloadOnly => rx,
    };

    let cap_bytes = config.monthly_cap_bytes.unwrap_or(0);
    let (usage_pct, is_over_budget) = if cap_bytes > 0 {
        let pct = ((consumed_bytes as u128 * 100) / cap_bytes as u128).min(u16::MAX as u128) as u16;
        (pct, consumed_bytes >= cap_bytes)
    } else {
        (0, false)
    };

    // 3. Milestone Crossing Detection
    let mut milestone_to_notify = None;
    if config.enabled && cap_bytes > 0 {
        if usage_pct >= 100 && !store.notified_milestones.notified_100 && config.notify_100 {
            milestone_to_notify = Some(BudgetMilestone {
                tier: MilestoneTier::Percent100,
                consumed_bytes,
                cap_bytes,
                days_remaining,
                usage_pct,
            });
            store.notified_milestones.notified_100 = true;
            store.notified_milestones.notified_90 = true;
            store.notified_milestones.notified_80 = true;
        } else if usage_pct >= 90 && !store.notified_milestones.notified_90 && config.notify_90 {
            milestone_to_notify = Some(BudgetMilestone {
                tier: MilestoneTier::Percent90,
                consumed_bytes,
                cap_bytes,
                days_remaining,
                usage_pct,
            });
            store.notified_milestones.notified_90 = true;
            store.notified_milestones.notified_80 = true;
        } else if usage_pct >= 80 && !store.notified_milestones.notified_80 && config.notify_80 {
            milestone_to_notify = Some(BudgetMilestone {
                tier: MilestoneTier::Percent80,
                consumed_bytes,
                cap_bytes,
                days_remaining,
                usage_pct,
            });
            store.notified_milestones.notified_80 = true;
        }
    }

    BudgetSnapshot {
        consumed_bytes,
        cap_bytes,
        usage_pct,
        days_remaining,
        scope: config.scope,
        is_over_budget,
        cycle_start,
        cycle_end,
        milestone_to_notify,
    }
}

/// Derives dual-column widths for the Adaptive Card budget progress bar.
///
/// Guaranteed to always return non-negative widths, even when `usage_pct > 100`.
pub fn budget_progress_width(usage_pct: u16) -> (u8, u8) {
    let used = (usage_pct as u8).min(100);
    let remaining = 100 - used;
    (used, remaining)
}

/// Returns the Fluent/Adaptive Card style name corresponding to the quota percentage.
pub fn budget_bar_style(usage_pct: u16) -> &'static str {
    if usage_pct < 80 {
        "Accent"
    } else if usage_pct < 100 {
        "Warning"
    } else {
        "Attention"
    }
}

/// Formats the primary status headline for Large widget cards.
/// E.g.: "384.2 GB of 500 GB (77%) · 14 days left"
pub fn format_budget_headline(consumed: u64, cap: u64, pct: u16, days_remaining: u32) -> String {
    let days_label = if days_remaining == 1 {
        "1 day left".to_string()
    } else {
        format!("{} days left", days_remaining)
    };

    if cap > 0 {
        format!(
            "{} of {} ({}%) · {}",
            format_bytes(consumed),
            format_bytes(cap),
            pct,
            days_label
        )
    } else {
        format!("{} used · {}", format_bytes(consumed), days_label)
    }
}

/// Formats the compact status badge for Medium widget cards.
/// E.g.: "Budget: 384.2 / 500 GB (77%) · 14d left"
pub fn format_budget_medium(consumed: u64, cap: u64, pct: u16, days_remaining: u32) -> String {
    if cap > 0 {
        format!(
            "Budget: {} / {} ({}%) · {}d left",
            format_bytes(consumed),
            format_bytes(cap),
            pct,
            days_remaining
        )
    } else {
        format!(
            "Budget: {} · {}d left",
            format_bytes(consumed),
            days_remaining
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(2024));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(2026));
        assert!(!is_leap_year(1900));
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(2026, 1), 31);
        assert_eq!(days_in_month(2026, 2), 28);
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2026, 4), 30);
    }

    #[test]
    fn test_cycle_boundaries_middle_of_month() {
        // Renewal day 1, today is Sep 29, 2026
        let (start, end, days) = calculate_cycle_boundaries(2026, 9, 29, 1);
        assert_eq!(start, (2026, 9, 1));
        assert_eq!(end, (2026, 10, 1));
        assert_eq!(days, 2); // 30 - 29 + 1 = 2 days
    }

    #[test]
    fn test_cycle_boundaries_last_day_of_month() {
        // Renewal day 1, today is Sep 30, 2026
        let (start, end, days) = calculate_cycle_boundaries(2026, 9, 30, 1);
        assert_eq!(start, (2026, 9, 1));
        assert_eq!(end, (2026, 10, 1));
        assert_eq!(days, 1);
    }

    #[test]
    fn test_cycle_boundaries_renewal_day_today() {
        // Renewal day 1, today is Oct 1, 2026 -> new cycle starts today!
        let (start, end, days) = calculate_cycle_boundaries(2026, 10, 1, 1);
        assert_eq!(start, (2026, 10, 1));
        assert_eq!(end, (2026, 11, 1));
        assert_eq!(days, 31);
    }

    #[test]
    fn test_cycle_boundaries_clamped_day_31_in_february() {
        // Configured renewal day 31, in Feb 2026 (non-leap, 28 days)
        // If today is Feb 15: cycle started Jan 31, ends Feb 28
        let (start, end, days) = calculate_cycle_boundaries(2026, 2, 15, 31);
        assert_eq!(start, (2026, 1, 31));
        assert_eq!(end, (2026, 2, 28));
        assert_eq!(days, 13);
    }

    #[test]
    fn test_cycle_boundaries_leap_year_february() {
        // Configured renewal day 31, in Feb 2024 (leap, 29 days)
        let (start, end, days) = calculate_cycle_boundaries(2024, 2, 29, 31);
        assert_eq!(start, (2024, 2, 29));
        assert_eq!(end, (2024, 3, 31));
        assert_eq!(days, 31);
    }

    #[test]
    fn test_cycle_boundaries_year_wrap() {
        // Renewal day 15, today is Jan 5, 2027 -> cycle started Dec 15, 2026, ends Jan 15, 2027
        let (start, end, days) = calculate_cycle_boundaries(2027, 1, 5, 15);
        assert_eq!(start, (2026, 12, 15));
        assert_eq!(end, (2027, 1, 15));
        assert_eq!(days, 10);
    }

    #[test]
    fn test_progress_width_clamping() {
        assert_eq!(budget_progress_width(0), (0, 100));
        assert_eq!(budget_progress_width(1), (1, 99));
        assert_eq!(budget_progress_width(79), (79, 21));
        assert_eq!(budget_progress_width(80), (80, 20));
        assert_eq!(budget_progress_width(90), (90, 10));
        assert_eq!(budget_progress_width(99), (99, 1));
        assert_eq!(budget_progress_width(100), (100, 0));
        assert_eq!(budget_progress_width(150), (100, 0));
    }

    #[test]
    fn test_budget_bar_style() {
        assert_eq!(budget_bar_style(0), "Accent");
        assert_eq!(budget_bar_style(79), "Accent");
        assert_eq!(budget_bar_style(80), "Warning");
        assert_eq!(budget_bar_style(99), "Warning");
        assert_eq!(budget_bar_style(100), "Attention");
        assert_eq!(budget_bar_style(150), "Attention");
    }

    #[test]
    fn test_milestone_evaluation_crossing() {
        let config = DataBudgetConfig {
            enabled: true,
            monthly_cap_bytes: Some(1000),
            renewal_day: 1,
            scope: BudgetScope::Combined,
            notify_80: true,
            notify_90: true,
            notify_100: true,
        };

        let mut store = DailyUsageStore::new();
        // 75% -> no milestone
        store.record_usage_delta("2026-09-10", 750, 0);
        let snap1 = calculate_budget_snapshot(&config, &mut store, (2026, 9, 10));
        assert_eq!(snap1.usage_pct, 75);
        assert_eq!(snap1.milestone_to_notify, None);

        // Crosses 80% (820 bytes = 82%)
        store.record_usage_delta("2026-09-11", 70, 0);
        let snap2 = calculate_budget_snapshot(&config, &mut store, (2026, 9, 11));
        assert_eq!(snap2.usage_pct, 82);
        assert_eq!(
            snap2.milestone_to_notify,
            Some(BudgetMilestone {
                tier: MilestoneTier::Percent80,
                consumed_bytes: 820,
                cap_bytes: 1000,
                days_remaining: 20,
                usage_pct: 82,
            })
        );

        // Next tick at 85% -> does NOT re-notify 80%
        store.record_usage_delta("2026-09-11", 30, 0);
        let snap3 = calculate_budget_snapshot(&config, &mut store, (2026, 9, 11));
        assert_eq!(snap3.usage_pct, 85);
        assert_eq!(snap3.milestone_to_notify, None);

        // Crosses 90% (910 bytes = 91%)
        store.record_usage_delta("2026-09-12", 60, 0);
        let snap4 = calculate_budget_snapshot(&config, &mut store, (2026, 9, 12));
        assert_eq!(snap4.usage_pct, 91);
        assert_eq!(
            snap4.milestone_to_notify,
            Some(BudgetMilestone {
                tier: MilestoneTier::Percent90,
                consumed_bytes: 910,
                cap_bytes: 1000,
                days_remaining: 19,
                usage_pct: 91,
            })
        );
    }
}
