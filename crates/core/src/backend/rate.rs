use std::collections::VecDeque;
use std::time::{Duration, Instant};

use super::types::HistorySample;

/// Accumulates measurement intervals and slices them into discrete buckets (default 500ms).
///
/// Because timer ticks on Windows can have minor jitter (e.g. 485ms or 515ms),
/// this distributes bytes proportionally across bucket boundaries so chart points
/// represent uniform time intervals without dropping or fabricating bytes.
#[derive(Clone, Debug, Default)]
pub struct RateAccumulator {
    time_acc_ns: u64,
    bytes_rx_acc: u64,
    bytes_tx_acc: u64,
}

impl RateAccumulator {
    pub fn new() -> Self {
        Self {
            time_acc_ns: 0,
            bytes_rx_acc: 0,
            bytes_tx_acc: 0,
        }
    }

    pub fn reset(&mut self) {
        self.time_acc_ns = 0;
        self.bytes_rx_acc = 0;
        self.bytes_tx_acc = 0;
    }

    pub fn time_acc_ns(&self) -> u64 {
        self.time_acc_ns
    }

    pub fn bytes_rx_acc(&self) -> u64 {
        self.bytes_rx_acc
    }

    pub fn bytes_tx_acc(&self) -> u64 {
        self.bytes_tx_acc
    }

    /// Pushes an observed time slice (`elapsed_ns`, `delta_rx`, `delta_tx`) and returns
    /// any completed buckets of duration `slot_ns`.
    pub fn push_sample(
        &mut self,
        elapsed_ns: u64,
        delta_rx: u64,
        delta_tx: u64,
        slot_ns: u64,
    ) -> Vec<HistorySample> {
        if elapsed_ns == 0 || slot_ns == 0 {
            return Vec::new();
        }

        let needed_ns = slot_ns.saturating_sub(self.time_acc_ns);

        // If the sample interval does not cross the current bucket boundary,
        // simply accumulate and return.
        if elapsed_ns < needed_ns {
            self.time_acc_ns += elapsed_ns;
            self.bytes_rx_acc += delta_rx;
            self.bytes_tx_acc += delta_tx;
            return Vec::new();
        }

        let mut emitted = Vec::new();
        let mut consumed_time_ns = 0u64;
        let mut consumed_rx = 0u64;
        let mut consumed_tx = 0u64;

        // 1. Complete the currently in-progress bucket
        let cum_boundary_1 = needed_ns;
        let target_rx_1 = ((delta_rx as u128 * cum_boundary_1 as u128) / elapsed_ns as u128) as u64;
        let target_tx_1 = ((delta_tx as u128 * cum_boundary_1 as u128) / elapsed_ns as u128) as u64;

        let slice_rx_1 = target_rx_1 - consumed_rx;
        let slice_tx_1 = target_tx_1 - consumed_tx;

        let bucket_rx_1 = self.bytes_rx_acc + slice_rx_1;
        let bucket_tx_1 = self.bytes_tx_acc + slice_tx_1;
        emitted.push(HistorySample::new(bucket_rx_1, bucket_tx_1, slot_ns));

        consumed_rx += slice_rx_1;
        consumed_tx += slice_tx_1;
        consumed_time_ns += needed_ns;

        self.time_acc_ns = 0;
        self.bytes_rx_acc = 0;
        self.bytes_tx_acc = 0;

        // 2. Emit any full buckets contained within the remainder of this interval
        while (elapsed_ns - consumed_time_ns) >= slot_ns {
            let cum_boundary = consumed_time_ns + slot_ns;
            let target_rx = ((delta_rx as u128 * cum_boundary as u128) / elapsed_ns as u128) as u64;
            let target_tx = ((delta_tx as u128 * cum_boundary as u128) / elapsed_ns as u128) as u64;

            let slice_rx = target_rx - consumed_rx;
            let slice_tx = target_tx - consumed_tx;

            emitted.push(HistorySample::new(slice_rx, slice_tx, slot_ns));

            consumed_rx += slice_rx;
            consumed_tx += slice_tx;
            consumed_time_ns += slot_ns;
        }

        // 3. Stash remaining residual fraction in accumulator
        let rem_time_ns = elapsed_ns - consumed_time_ns;
        let rem_rx = delta_rx - consumed_rx;
        let rem_tx = delta_tx - consumed_tx;

        self.time_acc_ns = rem_time_ns;
        self.bytes_rx_acc = rem_rx;
        self.bytes_tx_acc = rem_tx;

        emitted
    }
}

/// Timestamped cumulative byte totals used for boundary interpolation in `RollingRateWindow`.
#[derive(Clone, Copy, Debug)]
pub struct CounterPoint {
    pub timestamp: Instant,
    pub total_rx: u64,
    pub total_tx: u64,
}

/// Calculates a smooth 1-second rolling transfer rate by interpolating byte counters at window boundaries.
#[derive(Clone, Debug)]
pub struct RollingRateWindow {
    points: VecDeque<CounterPoint>,
    window: Duration,
}

impl RollingRateWindow {
    pub fn new(window_ns: u64) -> Self {
        Self {
            points: VecDeque::new(),
            window: Duration::from_nanos(window_ns),
        }
    }

    pub fn reset(&mut self) {
        self.points.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    pub fn record_sample(&mut self, now: Instant, total_rx: u64, total_tx: u64) {
        // If there is a massive time gap (> 5s), reset the window
        if let Some(last) = self.points.back()
            && now.saturating_duration_since(last.timestamp) > Duration::from_secs(5)
        {
            self.points.clear();
        }
        self.points.push_back(CounterPoint {
            timestamp: now,
            total_rx,
            total_tx,
        });

        // Prune points that are older than window + extra buffer, but retain at least 1 point
        // older than (now - window) so we can always bracket the boundary.
        if let Some(target_time) = now.checked_sub(self.window) {
            while self.points.len() >= 2 {
                // If the second point is also <= target_time, the first point is unneeded
                if self.points[1].timestamp <= target_time {
                    self.points.pop_front();
                } else {
                    break;
                }
            }
        }
    }

    /// Calculate the rolling rate over the specified window (default 1s) ending at `now`.
    /// Reconstructs the counter at (now - window) using piecewise-constant interpolation.
    pub fn current_rate(&self, now: Instant) -> (f64, f64) {
        let latest = match self.points.back() {
            Some(p) => p,
            None => return (0.0, 0.0),
        };

        let target_time = match now.checked_sub(self.window) {
            Some(t) => t,
            None => return (0.0, 0.0),
        };

        // If we only have 1 point, or the oldest point is newer than target_time:
        let oldest = &self.points[0];
        if self.points.len() == 1 || oldest.timestamp >= target_time {
            let elapsed = now
                .saturating_duration_since(oldest.timestamp)
                .as_secs_f64();
            if elapsed > 0.0 {
                let rx = (latest.total_rx.saturating_sub(oldest.total_rx)) as f64 / elapsed;
                let tx = (latest.total_tx.saturating_sub(oldest.total_tx)) as f64 / elapsed;
                return (rx, tx);
            } else {
                return (0.0, 0.0);
            }
        }

        // Find the bracket: points[i].timestamp <= target_time <= points[i+1].timestamp
        let mut bracket = None;
        for i in 0..self.points.len() - 1 {
            if self.points[i].timestamp <= target_time
                && self.points[i + 1].timestamp >= target_time
            {
                bracket = Some((&self.points[i], &self.points[i + 1]));
                break;
            }
        }

        let (p_start, p_end) = match bracket {
            Some(b) => b,
            None => {
                // Fallback to earliest point
                let elapsed = now
                    .saturating_duration_since(oldest.timestamp)
                    .as_secs_f64();
                if elapsed > 0.0 {
                    let rx = (latest.total_rx.saturating_sub(oldest.total_rx)) as f64 / elapsed;
                    let tx = (latest.total_tx.saturating_sub(oldest.total_tx)) as f64 / elapsed;
                    return (rx, tx);
                } else {
                    return (0.0, 0.0);
                }
            }
        };

        let bracket_duration = p_end
            .timestamp
            .saturating_duration_since(p_start.timestamp)
            .as_secs_f64();
        let (interp_rx, interp_tx) = if bracket_duration > 0.0 {
            let fraction = (target_time
                .saturating_duration_since(p_start.timestamp)
                .as_secs_f64()
                / bracket_duration)
                .clamp(0.0, 1.0);
            let rx_delta = (p_end.total_rx.saturating_sub(p_start.total_rx)) as f64;
            let tx_delta = (p_end.total_tx.saturating_sub(p_start.total_tx)) as f64;
            (
                p_start.total_rx as f64 + fraction * rx_delta,
                p_start.total_tx as f64 + fraction * tx_delta,
            )
        } else {
            (p_start.total_rx as f64, p_start.total_tx as f64)
        };

        let window_secs = self.window.as_secs_f64();
        let rx_bps = ((latest.total_rx as f64 - interp_rx) / window_secs).max(0.0);
        let tx_bps = ((latest.total_tx as f64 - interp_tx) / window_secs).max(0.0);
        (rx_bps, tx_bps)
    }
}

pub fn compute_delta(prev: u64, curr: u64) -> u64 {
    curr.saturating_sub(prev)
}

/// Discontinuity-aware counter delta evaluation.
///
/// Returns None when `curr < prev` (indicating a counter reset, interface restart,
/// driver reload, machine resume, or counter wrap). When None is returned, 0 bytes
/// should be attributed to this tick rather than erroneously adding the current counter value.
pub fn counter_delta(prev: u64, curr: u64) -> Option<u64> {
    if curr < prev { None } else { Some(curr - prev) }
}
