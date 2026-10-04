use super::*;
use std::time::{Duration, Instant};

fn mock_iface(
    luid: u64,
    category: InterfaceCategory,
    medium: InterfaceMedium,
    oper_status: i32,
    in_bytes: u64,
    out_bytes: u64,
) -> InterfaceInfo {
    InterfaceInfo {
        luid,
        index: luid as u32,
        name: format!("eth{}", luid),
        description: "Mock Adapter".to_string(),
        category,
        medium,
        oper_status,
        in_octets: in_bytes,
        out_octets: out_bytes,
        speed: 1_000_000_000,
    }
}

#[test]
fn test_classify_interface() {
    assert_eq!(
        classify_interface(24, 0, "Loopback", ""),
        (InterfaceCategory::Loopback, InterfaceMedium::Loopback)
    );
    assert_eq!(
        classify_interface(6, 1, "Tunnel adapter", ""),
        (InterfaceCategory::Tunnel, InterfaceMedium::Virtual)
    );
    assert_eq!(
        classify_interface(6, 0, "Hyper-V Virtual Ethernet Adapter", ""),
        (InterfaceCategory::Virtual, InterfaceMedium::Virtual)
    );
    assert_eq!(
        classify_interface(6, 0, "Realtek PCIe GbE Family Controller", "Ethernet"),
        (InterfaceCategory::Physical, InterfaceMedium::Ethernet)
    );
    assert_eq!(
        classify_interface(71, 0, "Intel(R) Wi-Fi 6 AX200", "Wi-Fi"),
        (InterfaceCategory::Physical, InterfaceMedium::Wifi)
    );
    assert_eq!(
        classify_interface(999, 0, "Something Else", ""),
        (InterfaceCategory::Physical, InterfaceMedium::Other)
    );
}

#[test]
fn test_compute_delta_normal_and_reset() {
    assert_eq!(compute_delta(100, 250), 150);
    assert_eq!(compute_delta(250, 250), 0);
    assert_eq!(compute_delta(500, 50), 0); // counter reset
}

#[test]
fn test_counter_delta_discontinuity() {
    assert_eq!(counter_delta(100, 250), Some(150));
    assert_eq!(counter_delta(250, 250), Some(0));
    assert_eq!(counter_delta(8_000_000, 100_000), None);
}

#[test]
fn test_backend_daily_usage_accumulation() {
    let mut backend = NetworkBackend::new();
    let t0 = Instant::now();
    let ifaces = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Wifi,
        1,
        1000,
        2000,
    )];
    // Sample 1 establishes baseline
    backend.sample_from_interfaces(&ifaces, t0);

    // Sample 2 adds 500 bytes RX and 300 bytes TX
    let t1 = t0 + Duration::from_secs(1);
    let ifaces2 = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Wifi,
        1,
        1500,
        2300,
    )];
    let snap = backend.sample_from_interfaces(&ifaces2, t1);
    assert_eq!(snap.session_rx, 500);
    assert_eq!(snap.session_tx, 300);
    assert!(snap.budget.is_some());

    let today_ymd = crate::budget::current_local_ymd();
    let today_str = crate::budget::format_ymd(today_ymd.0, today_ymd.1, today_ymd.2);
    let entry = backend.daily_usage.get_entry(&today_str).unwrap();
    assert_eq!(entry.rx_bytes, 500);
    assert_eq!(entry.tx_bytes, 300);
}

#[test]
fn test_first_sample_establishes_baseline() {
    let mut backend = NetworkBackend::new();
    let t0 = Instant::now();
    let ifaces = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Wifi,
        1,
        1000,
        2000,
    )];

    let snap = backend.sample_from_interfaces(&ifaces, t0);
    assert_eq!(snap.rx_bps, 0.0);
    assert_eq!(snap.tx_bps, 0.0);
    assert_eq!(snap.session_rx, 0);
    assert_eq!(snap.session_tx, 0);
    assert!(snap.history.is_empty());
    assert_eq!(snap.active_interfaces, 1);
    assert_eq!(snap.primary_medium, InterfaceMedium::Wifi);

    // Second sample 250ms later (1 sampling interval)
    let t1 = t0 + Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
    let ifaces2 = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Wifi,
        1,
        1000 + 1250,
        2000 + 500,
    )];
    let snap2 = backend.sample_from_interfaces(&ifaces2, t1);
    assert_eq!(snap2.rx_bps, 5000.0);
    assert_eq!(snap2.tx_bps, 2000.0);
    assert_eq!(snap2.session_rx, 1250);
    assert_eq!(snap2.session_tx, 500);
    assert_eq!(snap2.history.len(), 1);
    assert_eq!(snap2.history[0].rx_bps, 5000);
    assert_eq!(snap2.history[0].tx_bps, 2000);
}

#[test]
fn test_counter_reset_updates_baseline_immediately() {
    let mut backend = NetworkBackend::new();
    let t0 = Instant::now();
    let ifaces = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        1,
        10_000,
        10_000,
    )];
    backend.sample_from_interfaces(&ifaces, t0);

    // Sample 2: normal delta
    let t1 = t0 + Duration::from_secs(1);
    let ifaces2 = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        1,
        15_000,
        15_000,
    )];
    let snap2 = backend.sample_from_interfaces(&ifaces2, t1);
    assert_eq!(snap2.rx_bps, 5000.0);

    // Sample 3: adapter counter resets to 100
    let t2 = t1 + Duration::from_secs(1);
    let ifaces3 = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        1,
        100,
        100,
    )];
    let snap3 = backend.sample_from_interfaces(&ifaces3, t2);
    assert_eq!(snap3.rx_bps, 0.0); // clamped to 0

    // Sample 4: delta from new baseline (100 -> 300 = 200)
    let t3 = t2 + Duration::from_secs(1);
    let ifaces4 = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        1,
        300,
        300,
    )];
    let snap4 = backend.sample_from_interfaces(&ifaces4, t3);
    assert_eq!(snap4.rx_bps, 200.0);
}

#[test]
fn test_disappeared_interface_pruned() {
    let mut backend = NetworkBackend::new();
    let t0 = Instant::now();
    let ifaces = vec![
        mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            1000,
            1000,
        ),
        mock_iface(
            2,
            InterfaceCategory::Physical,
            InterfaceMedium::Wifi,
            1,
            1000,
            1000,
        ),
    ];
    backend.sample_from_interfaces(&ifaces, t0);
    assert_eq!(backend.prev_counters.len(), 2);

    // Sample 2: interface 2 disappears
    let t1 = t0 + Duration::from_secs(1);
    let ifaces2 = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        1,
        2000,
        2000,
    )];
    backend.sample_from_interfaces(&ifaces2, t1);
    assert_eq!(backend.prev_counters.len(), 1);
    assert!(backend.prev_counters.contains_key(&1));
    assert!(!backend.prev_counters.contains_key(&2));
}

#[test]
fn test_mode_filtering() {
    let mut backend = NetworkBackend::with_mode(AggregateMode::PhysicalTransport);
    let t0 = Instant::now();
    let ifaces = vec![
        mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            1000,
            1000,
        ),
        mock_iface(
            2,
            InterfaceCategory::Virtual,
            InterfaceMedium::Virtual,
            1,
            1000,
            1000,
        ),
        mock_iface(
            3,
            InterfaceCategory::Loopback,
            InterfaceMedium::Loopback,
            1,
            1000,
            1000,
        ),
    ];
    let snap = backend.sample_from_interfaces(&ifaces, t0);
    assert_eq!(snap.active_interfaces, 1);

    let t1 = t0 + Duration::from_secs(1);
    let ifaces2 = vec![
        mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            2000,
            2000,
        ),
        mock_iface(
            2,
            InterfaceCategory::Virtual,
            InterfaceMedium::Virtual,
            1,
            9999,
            9999,
        ),
        mock_iface(
            3,
            InterfaceCategory::Loopback,
            InterfaceMedium::Loopback,
            1,
            9999,
            9999,
        ),
    ];
    let snap2 = backend.sample_from_interfaces(&ifaces2, t1);
    // Only physical (iface 1) delta should be counted
    assert_eq!(snap2.rx_bps, 1000.0);
    assert_eq!(snap2.tx_bps, 1000.0);
}

#[test]
fn test_history_rolling_fifo() {
    let mut backend = NetworkBackend::new();
    let t0 = Instant::now();
    let ifaces = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        1,
        0,
        0,
    )];
    backend.sample_from_interfaces(&ifaces, t0);

    // Generate 260 samples at 250ms intervals (65s total, exceeds capacity of 240 / 60s)
    for i in 1..=260u64 {
        let t = t0 + Duration::from_millis(i * crate::SAMPLING_INTERVAL_MS);
        let ifaces = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            i * 1000,
            i * 500,
        )];
        backend.sample_from_interfaces(&ifaces, t);
    }

    assert_eq!(backend.history.len(), HISTORY_CAPACITY);
    assert_eq!(HISTORY_CAPACITY, crate::history_samples_for_secs(60));
    assert_eq!(HISTORY_CAPACITY, 240);
    // The oldest samples should have been dropped
    assert!(backend.history.len() <= HISTORY_CAPACITY);
}

#[test]
fn test_reset_session() {
    let mut backend = NetworkBackend::new();
    let t0 = Instant::now();
    let ifaces = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Wifi,
        1,
        1000,
        1000,
    )];
    backend.sample_from_interfaces(&ifaces, t0);

    let t1 = t0 + Duration::from_secs(1);
    let ifaces2 = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Wifi,
        1,
        5000,
        5000,
    )];
    backend.sample_from_interfaces(&ifaces2, t1);
    assert!(backend.session_rx > 0);
    assert!(!backend.history.is_empty());

    backend.reset_session();
    assert_eq!(backend.session_rx, 0);
    assert_eq!(backend.session_tx, 0);
    assert_eq!(backend.peak_rx, 0.0);
    assert_eq!(backend.peak_tx, 0.0);
    assert!(backend.history.is_empty());
    assert!(backend.prev_counters.is_empty());

    // First sample after reset re-establishes baseline without recording history or spiking rates
    let t2 = t1 + Duration::from_secs(1);
    let ifaces3 = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Wifi,
        1,
        6000,
        6000,
    )];
    let snap_reset = backend.sample_from_interfaces(&ifaces3, t2);
    assert_eq!(snap_reset.rx_bps, 0.0);
    assert_eq!(snap_reset.tx_bps, 0.0);
    assert_eq!(snap_reset.session_rx, 0);
    assert_eq!(snap_reset.session_tx, 0);
    assert!(snap_reset.history.is_empty());

    // Second sample after reset (250ms cadence) computes real delta from new baseline
    let t3 = t2 + Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
    let ifaces4 = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Wifi,
        1,
        6000 + 250,
        6000 + 125,
    )];
    let snap_after = backend.sample_from_interfaces(&ifaces4, t3);
    assert_eq!(snap_after.rx_bps, 1000.0);
    assert_eq!(snap_after.tx_bps, 500.0);
    assert_eq!(snap_after.session_rx, 250);
    assert_eq!(snap_after.session_tx, 125);
    assert_eq!(snap_after.history.len(), 1);
    assert_eq!(snap_after.history[0].rx_bps, 1000);
    assert_eq!(snap_after.history[0].tx_bps, 500);
}

#[test]
fn test_zero_or_backwards_duration_handled_safely() {
    let mut backend = NetworkBackend::new();
    let t0 = Instant::now();
    let ifaces = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        1,
        1000,
        1000,
    )];
    backend.sample_from_interfaces(&ifaces, t0);

    // Immediate second sample at exact same instant t0 (duration = 0)
    let snap_zero = backend.sample_from_interfaces(&ifaces, t0);
    assert_eq!(snap_zero.rx_bps, 0.0);
    assert_eq!(snap_zero.tx_bps, 0.0);
    assert_eq!(snap_zero.history.len(), 0);

    // Third sample with backwards time (clock jitter where now < prev)
    let t_before = t0 - Duration::from_millis(100);
    let snap_jitter = backend.sample_from_interfaces(&ifaces, t_before);
    assert_eq!(snap_jitter.rx_bps, 0.0);
    assert_eq!(snap_jitter.tx_bps, 0.0);
    assert_eq!(snap_jitter.history.len(), 0);
}

#[test]
fn test_inactive_interface_baseline_fix() {
    // Scenario: interface starts Down, comes Up — should NOT spike.
    let mut backend = NetworkBackend::new();
    let t0 = Instant::now();

    // Sample 1: iface is Down with counters at 10000
    let ifaces = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        2,
        10_000,
        10_000,
    )]; // status=2 (Down)
    backend.sample_from_interfaces(&ifaces, t0);

    // Sample 2: iface is still Down, counters advanced
    let t1 = t0 + Duration::from_secs(1);
    let ifaces2 = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        2,
        50_000,
        50_000,
    )];
    backend.sample_from_interfaces(&ifaces2, t1);

    // Sample 3: iface comes Up — delta should be from 50000, NOT from 0
    let t2 = t1 + Duration::from_secs(1);
    let ifaces3 = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        1,
        51_000,
        51_000,
    )]; // Up
    let snap3 = backend.sample_from_interfaces(&ifaces3, t2);

    // Only 1000 bytes delta (51000-50000), not 51000
    assert_eq!(snap3.rx_bps, 1000.0);
    assert_eq!(snap3.tx_bps, 1000.0);
}

#[test]
fn test_wifi_ssid_query_does_not_panic() {
    // Safe execution check: whether running on Wi-Fi, Ethernet, or headless VM,
    // this must never panic and must return cleanly.
    let _ = query_cached_wifi_ssid();
    let _ = query_cached_wifi_phy();
}

#[test]
fn test_history_time_cadence_gap_expansion() {
    let mut backend = NetworkBackend::new();
    let t0 = Instant::now();
    let ifaces = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        1,
        1000,
        1000,
    )];
    backend.sample_from_interfaces(&ifaces, t0);

    // Advance 3s (12 intervals of 250ms)
    let t1 = t0 + Duration::from_secs(3);
    let ifaces2 = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        1,
        1000 + 15000,
        1000 + 6000,
    )];
    let snap = backend.sample_from_interfaces(&ifaces2, t1);
    assert_eq!(snap.rx_bps, 5000.0);
    assert_eq!(snap.tx_bps, 2000.0);
    // History should contain 12 samples reflecting the 3s elapsed span (3.0s / 0.25s = 12)
    assert_eq!(snap.history.len(), 12);
    for s in &snap.history {
        assert_eq!(s.rx_bps, 5000);
        assert_eq!(s.tx_bps, 2000);
    }
}

#[test]
fn test_history_clears_on_extended_gap() {
    let mut backend = NetworkBackend::new();
    let t0 = Instant::now();
    let ifaces = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        1,
        1000,
        1000,
    )];
    backend.sample_from_interfaces(&ifaces, t0);

    // Push some initial samples at 250ms intervals
    for i in 1..=5u64 {
        let t = t0 + Duration::from_millis(i * crate::SAMPLING_INTERVAL_MS);
        let ifaces = vec![mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            1000 + i * 1000,
            1000 + i * 500,
        )];
        backend.sample_from_interfaces(&ifaces, t);
    }
    assert_eq!(backend.history.len(), 5);

    // Simulate 120-second suspend/sleep gap (> 5s threshold)
    let t_sleep =
        t0 + Duration::from_millis(5 * crate::SAMPLING_INTERVAL_MS) + Duration::from_secs(120);
    let ifaces_wake = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        1,
        100_000,
        50_000,
    )];
    let snap_wake = backend.sample_from_interfaces(&ifaces_wake, t_sleep);
    // Stale samples from before sleep are cleared, and wake establishes a clean new baseline
    assert_eq!(snap_wake.history.len(), 0);
    assert_eq!(backend.accumulator.time_acc_ns(), 0);
    assert_eq!(backend.accumulator.bytes_rx_acc(), 0);

    // Next 250ms sample after wake starts fresh without incorporating pre-suspend data
    let t_after_wake = t_sleep + Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
    let ifaces_after_wake = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        1,
        102_500,
        51_250,
    )];
    let snap_after_wake = backend.sample_from_interfaces(&ifaces_after_wake, t_after_wake);
    assert_eq!(snap_after_wake.history.len(), 1);
    assert_eq!(snap_after_wake.history[0].rx_bytes, 2500);
    assert_eq!(snap_after_wake.history[0].tx_bytes, 1250);
    assert_eq!(snap_after_wake.history[0].rx_bps, 10_000);
    assert_eq!(snap_after_wake.history[0].tx_bps, 5_000);
}

#[test]
fn test_adversarial_fractional_allocation() {
    let mut acc = RateAccumulator::new();
    // 10 bytes transferred across 3 ns with 1-ns boundaries
    let emitted = acc.push_sample(3, 10, 20, 1);
    assert_eq!(emitted.len(), 3);
    // Under cumulative proportional allocation:
    // boundary 1 (1/3 of 10 = 3) -> 3
    // boundary 2 (2/3 of 10 = 6 - 3 = 3) -> 3
    // boundary 3 (3/3 of 10 = 10 - 6 = 4) -> 4
    assert_eq!(emitted[0].rx_bytes, 3);
    assert_eq!(emitted[1].rx_bytes, 3);
    assert_eq!(emitted[2].rx_bytes, 4);

    // TX (20 bytes across 3 ns):
    // boundary 1 (1/3 of 20 = 6) -> 6
    // boundary 2 (2/3 of 20 = 13 - 6 = 7) -> 7
    // boundary 3 (3/3 of 20 = 20 - 13 = 7) -> 7
    assert_eq!(emitted[0].tx_bytes, 6);
    assert_eq!(emitted[1].tx_bytes, 7);
    assert_eq!(emitted[2].tx_bytes, 7);

    assert_eq!(acc.time_acc_ns(), 0);
    assert_eq!(acc.bytes_rx_acc(), 0);
    assert_eq!(acc.bytes_tx_acc(), 0);
}

#[test]
fn test_exact_byte_conservation_under_jitter() {
    let mut acc = RateAccumulator::new();
    let slot_ns = 500_000_000; // 500ms

    // 4 jittered intervals summing to exactly 2000ms (4 completed 500ms slots)
    let intervals = [
        (450_000_000, 10_000, 5_000),
        (550_000_000, 20_000, 10_000),
        (480_000_000, 15_000, 7_500),
        (520_000_000, 25_000, 12_500),
    ];

    let mut total_input_rx = 0u64;
    let mut total_input_tx = 0u64;
    let mut emitted_all = Vec::new();

    for (dt, rx, tx) in intervals {
        total_input_rx += rx;
        total_input_tx += tx;
        let mut buckets = acc.push_sample(dt, rx, tx, slot_ns);
        emitted_all.append(&mut buckets);
    }

    assert_eq!(emitted_all.len(), 4);
    assert_eq!(acc.time_acc_ns(), 0);
    assert_eq!(acc.bytes_rx_acc(), 0);
    assert_eq!(acc.bytes_tx_acc(), 0);

    let sum_emitted_rx: u64 = emitted_all.iter().map(|b| b.rx_bytes).sum();
    let sum_emitted_tx: u64 = emitted_all.iter().map(|b| b.tx_bytes).sum();

    // Exact integral conservation invariant
    assert_eq!(sum_emitted_rx, total_input_rx);
    assert_eq!(sum_emitted_tx, total_input_tx);

    for b in &emitted_all {
        assert_eq!(b.duration_ns, slot_ns);
        assert_eq!(b.rx_bps, b.rx_bytes * 2);
        assert_eq!(b.tx_bps, b.tx_bytes * 2);
    }
}

#[test]
fn test_long_running_phase_stability() {
    let mut acc = RateAccumulator::new();
    let slot_ns = 500_000_000;

    let mut total_input_time = 0u64;
    let mut total_input_rx = 0u64;
    let mut total_emitted_rx = 0u64;
    let mut total_emitted_slots = 0u64;

    // Run 10,000 jittered intervals alternating around 500ms
    for i in 0..10_000u64 {
        let jitter_offset = (i % 7) * 5_000_000; // 0ms to 30ms
        let dt = if i % 2 == 0 {
            485_000_000 + jitter_offset
        } else {
            515_000_000 - jitter_offset
        };
        let rx = 1000 + (i % 100);

        total_input_time += dt;
        total_input_rx += rx;

        let buckets = acc.push_sample(dt, rx, 0, slot_ns);
        total_emitted_slots += buckets.len() as u64;
        for b in buckets {
            total_emitted_rx += b.rx_bytes;
        }
    }

    // Timing phase invariant: emitted time + residual time == total input time
    let emitted_time = total_emitted_slots * slot_ns;
    assert_eq!(emitted_time + acc.time_acc_ns(), total_input_time);

    // Byte flux conservation invariant: emitted bytes + residual bytes == total input bytes
    assert_eq!(total_emitted_rx + acc.bytes_rx_acc(), total_input_rx);
}

#[test]
fn test_rolling_rate_window_boundary_interpolation() {
    let mut window = RollingRateWindow::new(1_000_000_000); // 1-second window
    let t0 = Instant::now();

    // Feed points:
    // 0.0s -> 0 bytes
    // 0.4s -> 400 bytes
    // 0.9s -> 900 bytes
    // 1.4s -> 1400 bytes
    window.record_sample(t0, 0, 0);
    window.record_sample(t0 + Duration::from_millis(400), 400, 0);
    window.record_sample(t0 + Duration::from_millis(900), 900, 0);
    window.record_sample(t0 + Duration::from_millis(1400), 1400, 0);

    // At t = 1.4s, the 1-second window looks back to t = 0.4s.
    // Counter at 0.4s is exactly 400.
    // Rate = (1400 - 400) / 1.0s = 1000 B/s.
    let (rate_rx, _) = window.current_rate(t0 + Duration::from_millis(1400));
    assert!((rate_rx - 1000.0).abs() < 1e-4);

    // Now advance to t = 1.5s (1500 bytes)
    // 1-second window looks back to t = 0.5s.
    // t = 0.5s falls between 0.4s (400 bytes) and 0.9s (900 bytes).
    // dt = 0.5s, fraction = (0.5 - 0.4) / 0.5 = 0.2.
    // Interpolated counter = 400 + 0.2 * 500 = 500 bytes.
    // Rate = (1500 - 500) / 1.0s = 1000 B/s!
    window.record_sample(t0 + Duration::from_millis(1500), 1500, 0);
    let (rate_rx_interp, _) = window.current_rate(t0 + Duration::from_millis(1500));
    assert!((rate_rx_interp - 1000.0).abs() < 1e-4);
}

#[test]
fn test_session_state_serde_roundtrip() {
    let state = SessionState {
        generation: 1,
        session_rx: 1234567,
        session_tx: 987654,
        session_start_unix: 1700000000,
        all_time_peak_rx: 4500000.0,
        all_time_peak_tx: 1200000.0,
        updated_at_unix: 1700000500,
        latency: LatencySnapshot::default(),
        physical_link: None,
    };
    let serialized = serde_json::to_string(&state).expect("serialize");
    let deserialized: SessionState = serde_json::from_str(&serialized).expect("deserialize");
    assert_eq!(state, deserialized);

    // Verify backward-compatible aliases
    let json_with_aliases = r#"{
        "generation": 42,
        "download_bytes": 1000,
        "upload_bytes": 2000,
        "started_at": 1700000000
    }"#;
    let from_aliases: SessionState =
        serde_json::from_str(json_with_aliases).expect("deserialize aliases");
    assert_eq!(from_aliases.generation, 42);
    assert_eq!(from_aliases.session_rx, 1000);
    assert_eq!(from_aliases.session_tx, 2000);
    assert_eq!(from_aliases.session_start_unix, 1700000000);
}
#[test]
fn test_incremental_chart_peak_tracking() {
    let mut backend = NetworkBackend::new();
    let t0 = Instant::now();
    let ifaces = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        1,
        0,
        0,
    )];
    backend.sample_from_interfaces(&ifaces, t0);

    let mut t = t0;
    let mut total_rx = 0u64;
    let mut total_tx = 0u64;

    // Sample 1: rate 20,000 rx, 10,000 tx (5,000 rx bytes, 2,500 tx bytes in 250ms)
    t += Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
    total_rx += 5000;
    total_tx += 2500;
    let snap = backend.sample_from_interfaces(
        &[mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            total_rx,
            total_tx,
        )],
        t,
    );
    assert_eq!(snap.peak_rx_bps, 20_000.0);
    assert_eq!(snap.peak_tx_bps, 10_000.0);

    // Sample 2: burst! rate 100,000 rx, 50,000 tx (25,000 rx bytes, 12,500 tx bytes)
    t += Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
    total_rx += 25000;
    total_tx += 12500;
    let snap = backend.sample_from_interfaces(
        &[mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            total_rx,
            total_tx,
        )],
        t,
    );
    assert_eq!(snap.peak_rx_bps, 100_000.0);
    assert_eq!(snap.peak_tx_bps, 50_000.0);

    // Sample 3: drop down to lower rate 4,000 rx, 2,000 tx (1,000 rx bytes, 500 tx bytes)
    t += Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
    total_rx += 1000;
    total_tx += 500;
    let snap = backend.sample_from_interfaces(
        &[mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            total_rx,
            total_tx,
        )],
        t,
    );
    assert_eq!(snap.peak_rx_bps, 100_000.0);
    assert_eq!(snap.peak_tx_bps, 50_000.0);

    // Push 237 more samples at 4,000 rx, 2,000 tx (total 240 samples in history)
    for _ in 0..237 {
        t += Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
        total_rx += 1000;
        total_tx += 500;
        backend.sample_from_interfaces(
            &[mock_iface(
                1,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                total_rx,
                total_tx,
            )],
            t,
        );
    }
    assert_eq!(backend.history.len(), HISTORY_CAPACITY);
    assert_eq!(backend.chart_peak_rx, 100_000);

    // Push 1 more sample: evicts sample 1 (20,000 rx). Peak 100,000 is still in history!
    t += Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
    total_rx += 1000;
    total_tx += 500;
    let snap = backend.sample_from_interfaces(
        &[mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            total_rx,
            total_tx,
        )],
        t,
    );
    assert_eq!(snap.peak_rx_bps, 100_000.0);

    // Push 1 more sample: evicts sample 2 (the 100,000 peak holder!). Lazy rescan occurs.
    // History now contains only 4,000 rx samples. Peak drops to 4,000!
    t += Duration::from_millis(crate::SAMPLING_INTERVAL_MS);
    total_rx += 1000;
    total_tx += 500;
    let snap = backend.sample_from_interfaces(
        &[mock_iface(
            1,
            InterfaceCategory::Physical,
            InterfaceMedium::Ethernet,
            1,
            total_rx,
            total_tx,
        )],
        t,
    );
    assert_eq!(snap.peak_rx_bps, 4_000.0);
    assert_eq!(snap.peak_tx_bps, 2_000.0);

    // Reset session drops peaks to 0
    backend.reset_session();
    assert_eq!(backend.chart_peak_rx, 0);
    assert_eq!(backend.chart_peak_tx, 0);
}

#[test]
fn test_atomic_write_simulation() {
    let dir = std::env::temp_dir().join(format!("netflow_test_atomic_{}", std::process::id()));
    let file_path = dir.join("nested").join("test_atomic.json");

    // 1. Initial write creates directories and writes content
    let initial_payload = b"{\"generation\": 1, \"data\": \"init\"}";
    write_atomic(&file_path, initial_payload).expect("initial write should succeed");
    assert_eq!(std::fs::read(&file_path).unwrap(), initial_payload);

    // 2. Overwrite replaces content atomically
    let updated_payload = b"{\"generation\": 2, \"data\": \"updated\"}";
    write_atomic(&file_path, updated_payload).expect("replacement should succeed");
    assert_eq!(std::fs::read(&file_path).unwrap(), updated_payload);

    // Clean up
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_session_state_generation_increments() {
    let mut backend = NetworkBackend::new();
    assert_eq!(backend.generation, 1);
    backend.reset_session();
    assert_eq!(backend.generation, 2);
    backend.reset_session();
    assert_eq!(backend.generation, 3);
}

#[test]
fn test_latency_snapshot_semantics_and_formatting() {
    let healthy = LatencySnapshot {
        latency_ms: Some(18),
        state: LatencyState::Healthy,
        target: LatencyTarget::Internet,
        sequence: 1,
        sampled_at_unix: 1700000000,
        jitter_ms: None,
        protocol: None,
        packet_loss_pct: None,
        pings_lost: 0,
        pings_total: 1,
        health: LatencyHealth::Healthy,
    };
    assert_eq!(healthy.display_text(), "18 ms");
    assert_eq!(healthy.detailed_display_text(), "18 ms · Internet");
    assert!(healthy.is_fresh(1700000005, 10));
    assert!(!healthy.is_fresh(1700000020, 10));

    let with_jitter = LatencySnapshot {
        latency_ms: Some(18),
        state: LatencyState::Healthy,
        target: LatencyTarget::Internet,
        sequence: 2,
        sampled_at_unix: 1700000001,
        jitter_ms: Some(2),
        protocol: None,
        packet_loss_pct: None,
        pings_lost: 0,
        pings_total: 2,
        health: LatencyHealth::Healthy,
    };
    assert_eq!(
        with_jitter.detailed_display_text(),
        "18 ms (±2 ms) · Internet"
    );

    let with_dual_stack_and_loss = LatencySnapshot {
        latency_ms: Some(18),
        state: LatencyState::Healthy,
        target: LatencyTarget::Internet,
        sequence: 3,
        sampled_at_unix: 1700000002,
        jitter_ms: Some(2),
        protocol: Some(IpProtocol::Ipv6),
        packet_loss_pct: Some(5),
        pings_lost: 1,
        pings_total: 20,
        health: LatencyHealth::Degraded,
    };
    assert_eq!(
        with_dual_stack_and_loss.detailed_display_text(),
        "18 ms (±2 ms) · 5% loss · Internet (IPv6)"
    );

    let timeout = LatencySnapshot {
        latency_ms: None,
        state: LatencyState::Timeout,
        target: LatencyTarget::Gateway,
        sequence: 4,
        sampled_at_unix: 1700000003,
        jitter_ms: None,
        protocol: Some(IpProtocol::Ipv4),
        packet_loss_pct: Some(100),
        pings_lost: 20,
        pings_total: 20,
        health: LatencyHealth::Timeout,
    };
    assert_eq!(timeout.display_text(), "Timeout · 100% loss");
    assert_eq!(
        timeout.detailed_display_text(),
        "Timeout · 100% loss · Gateway (IPv4)"
    );

    let unavailable = LatencySnapshot::default();
    assert_eq!(unavailable.display_text(), "-- ms");
    assert_eq!(unavailable.detailed_display_text(), "-- ms");
}

#[test]
fn test_wifi_generation_pure_mapping() {
    assert_eq!(wifi_generation(7), WifiGeneration::Wifi4);
    assert_eq!(wifi_generation(8), WifiGeneration::Wifi5);
    assert_eq!(wifi_generation(10), WifiGeneration::Wifi6);
    assert_eq!(wifi_generation(11), WifiGeneration::Wifi7);
    assert_eq!(wifi_generation(4), WifiGeneration::Legacy);
    assert_eq!(wifi_generation(9), WifiGeneration::Legacy);
    assert_eq!(wifi_generation(99), WifiGeneration::Unknown);
    assert_eq!(WifiGeneration::Wifi7.as_str(), "Wi-Fi 7");
    assert_eq!(WifiGeneration::Wifi6E.as_str(), "Wi-Fi 6E");
    assert_eq!(WifiGeneration::Wifi6.as_str(), "Wi-Fi 6");
    assert_eq!(
        wifi_generation_with_band(10, WifiBand::Band6Ghz),
        WifiGeneration::Wifi6E
    );
    assert_eq!(
        wifi_generation_with_band(10, WifiBand::Band5Ghz),
        WifiGeneration::Wifi6
    );
}

#[test]
fn test_signal_quality_to_rssi_dbm() {
    assert_eq!(signal_quality_to_rssi_dbm(0), -100);
    assert_eq!(signal_quality_to_rssi_dbm(50), -75);
    assert_eq!(signal_quality_to_rssi_dbm(100), -50);
    assert_eq!(signal_quality_to_rssi_dbm(80), -60);
}

#[test]
fn test_wifi_band_resolution() {
    assert_eq!(wifi_band(2412, None), WifiBand::Band24Ghz);
    assert_eq!(wifi_band(5180, None), WifiBand::Band5Ghz);
    assert_eq!(wifi_band(6100, None), WifiBand::Band6Ghz);
    assert_eq!(wifi_band(0, Some(6)), WifiBand::Band24Ghz);
    assert_eq!(wifi_band(0, Some(36)), WifiBand::Band5Ghz);
    assert_eq!(wifi_band(0, None), WifiBand::Unknown);
    assert_eq!(wifi_channel_number(2412), Some(1));
    assert_eq!(wifi_channel_number(2484), Some(14));
    assert_eq!(wifi_channel_number(5180), Some(36));
    assert_eq!(wifi_channel_number(5935), Some(2));
    assert_eq!(wifi_channel_number(5955), Some(1));
    assert_eq!(wifi_channel_number(7115), Some(233));
    assert_eq!(wifi_channel_number(6000), None);
    assert_eq!(WifiBand::Band24Ghz.as_str(), "2.4 GHz");
    assert_eq!(WifiBand::Band5Ghz.as_str(), "5 GHz");
    assert_eq!(WifiBand::Band6Ghz.as_str(), "6 GHz");
}

#[test]
fn test_packet_loss_tracker_semantics_and_rollover() {
    let mut tracker = PacketLossTracker::default();
    assert_eq!(tracker.loss_pct(), None);
    assert_eq!(tracker.health(), LatencyHealth::Unavailable);

    // 1. Successes only
    for _ in 0..10 {
        tracker.record(ProbeResult::Success {
            latency: Duration::from_millis(15),
        });
    }
    assert_eq!(tracker.loss_pct(), Some(0));
    assert_eq!(tracker.health(), LatencyHealth::Healthy);
    assert_eq!(tracker.counts(), (0, 10));

    // 2. Unavailable probes MUST NOT count towards packet loss
    for _ in 0..5 {
        tracker.record(ProbeResult::Unavailable);
    }
    // Total valid pings remains 10, lost remains 0, so 0% loss
    assert_eq!(tracker.loss_pct(), Some(0));
    assert_eq!(tracker.health(), LatencyHealth::Healthy);
    assert_eq!(tracker.counts(), (0, 10));

    // 3. One Timeout -> degraded
    tracker.record(ProbeResult::Timeout);
    assert_eq!(tracker.loss_pct(), Some(9)); // 1/11 = 9%
    assert_eq!(tracker.health(), LatencyHealth::Degraded);
    assert_eq!(tracker.counts(), (1, 11));

    // 4. Overwrite earlier samples with 19 successes so buffer is 19 success + 1 timeout
    for _ in 0..19 {
        tracker.record(ProbeResult::Success {
            latency: Duration::from_millis(18),
        });
    }
    assert_eq!(tracker.counts(), (1, 20));
    assert_eq!(tracker.loss_pct(), Some(5)); // 1/20 = 5%
    assert_eq!(tracker.health(), LatencyHealth::Degraded);

    // 5. Test rollover: push 20 timeouts
    for _ in 0..20 {
        tracker.record(ProbeResult::Timeout);
    }
    assert_eq!(tracker.counts(), (20, 20));
    assert_eq!(tracker.loss_pct(), Some(100));
    assert_eq!(tracker.health(), LatencyHealth::Timeout);
}

#[test]
fn test_physical_link_summary_formatting() {
    let wifi = PhysicalLinkInfo::Wifi(WifiPhyMetrics {
        ssid: "HomeMesh".to_string(),
        generation: WifiGeneration::Wifi6,
        band: WifiBand::Band5Ghz,
        channel: Some(36),
        signal_quality_pct: 84,
        rssi_dbm: Some(-58),
        tx_rate_mbps: Some(866),
        rx_rate_mbps: Some(866),
        is_mlo: false,
        link_count: 1,
    });
    assert_eq!(
        wifi.display_summary(),
        "Wi-Fi 6 · 5 GHz · -58 dBm · 866 Mbps"
    );

    let eth_1g = PhysicalLinkInfo::Ethernet(EthernetLinkMetrics {
        adapter_name: "Ethernet".to_string(),
        tx_speed_bps: 1_000_000_000,
        rx_speed_bps: 1_000_000_000,
    });
    assert_eq!(eth_1g.display_summary(), "Ethernet · 1 Gbps");

    let eth_2_5g = PhysicalLinkInfo::Ethernet(EthernetLinkMetrics {
        adapter_name: "Ethernet 2".to_string(),
        tx_speed_bps: 2_500_000_000,
        rx_speed_bps: 2_500_000_000,
    });
    assert_eq!(eth_2_5g.display_summary(), "Ethernet · 2.5 Gbps");

    let eth_100m = PhysicalLinkInfo::Ethernet(EthernetLinkMetrics {
        adapter_name: "Ethernet".to_string(),
        tx_speed_bps: 100_000_000,
        rx_speed_bps: 100_000_000,
    });
    assert_eq!(eth_100m.display_summary(), "Ethernet · 100 Mbps");
}

#[test]
fn test_active_adapter_selection_hierarchy() {
    let loopback = RawAdapterInfo {
        name: "{1111}".to_string(),
        description: "Software Loopback Interface 1".to_string(),
        friendly_name: "Loopback Pseudo-Interface 1".to_string(),
        interface_type: crate::export::InterfaceType::Loopback,
        is_physical: false,
        is_up: true,
        routing_metric: 75,
        link_speed_bps: 1_000_000_000,
        mac_address: "".to_string(),
        ipv4_addresses: vec!["127.0.0.1/8".to_string()],
        ipv6_addresses: vec!["::1/128".to_string()],
        default_gateways: vec![],
        dns_servers: vec![],
        dhcp_enabled: false,
        dhcp_server: None,
        mtu: 1500,
        wifi: None,
    };

    let hyperv = RawAdapterInfo {
        name: "{2222}".to_string(),
        description: "Hyper-V Virtual Ethernet Adapter".to_string(),
        friendly_name: "vEthernet (Default Switch)".to_string(),
        interface_type: crate::export::InterfaceType::Other,
        is_physical: false,
        is_up: true,
        routing_metric: 15,
        link_speed_bps: 10_000_000_000,
        mac_address: "00:15:5D:01:02:03".to_string(),
        ipv4_addresses: vec!["172.20.10.1/24".to_string()],
        ipv6_addresses: vec![],
        default_gateways: vec!["172.20.10.254".to_string()],
        dns_servers: vec!["172.20.10.254".to_string()],
        dhcp_enabled: false,
        dhcp_server: None,
        mtu: 1500,
        wifi: None,
    };

    let ethernet = RawAdapterInfo {
        name: "{3333}".to_string(),
        description: "Realtek Gaming 2.5GbE Family Controller".to_string(),
        friendly_name: "Ethernet".to_string(),
        interface_type: crate::export::InterfaceType::Ethernet,
        is_physical: true,
        is_up: true,
        routing_metric: 25,
        link_speed_bps: 2_500_000_000,
        mac_address: "04:D4:C4:01:02:03".to_string(),
        ipv4_addresses: vec!["192.168.1.150/24".to_string()],
        ipv6_addresses: vec!["2401:4900:1::50/64".to_string()],
        default_gateways: vec!["192.168.1.1".to_string()],
        dns_servers: vec!["1.1.1.1".to_string(), "1.0.0.1".to_string()],
        dhcp_enabled: true,
        dhcp_server: Some("192.168.1.1".to_string()),
        mtu: 1500,
        wifi: None,
    };

    let wifi = RawAdapterInfo {
        name: "{4444}".to_string(),
        description: "Intel(R) Wi-Fi 6E AX211 160MHz".to_string(),
        friendly_name: "Wi-Fi".to_string(),
        interface_type: crate::export::InterfaceType::Wifi,
        is_physical: true,
        is_up: true,
        routing_metric: 35,
        link_speed_bps: 1_200_000_000,
        mac_address: "50:EB:71:01:02:03".to_string(),
        ipv4_addresses: vec!["192.168.1.105/24".to_string()],
        ipv6_addresses: vec![],
        default_gateways: vec!["192.168.1.1".to_string()],
        dns_servers: vec!["1.1.1.1".to_string()],
        dhcp_enabled: true,
        dhcp_server: Some("192.168.1.1".to_string()),
        mtu: 1500,
        wifi: Some(crate::export::WifiDiagnostics {
            ssid: "HomeMesh".to_string(),
            bssid: "AA:BB:CC:DD:EE:FF".to_string(),
            generation: crate::export::WifiGeneration::Wifi6E,
            band_ghz: "5 GHz".to_string(),
            channel: 36,
            channel_width_mhz: 160,
            rssi_dbm: -54,
            transmit_rate_bps: 1_200_000_000,
            receive_rate_bps: 1_200_000_000,
        }),
    };

    let adapters = vec![loopback, hyperv, ethernet, wifi];
    let (active, others) = select_active_adapter(adapters);

    assert!(active.is_some());
    let active = active.unwrap();
    assert_eq!(active.name, "{3333}");
    assert_eq!(active.friendly_name, "Ethernet");
    assert_eq!(
        active.interface_type,
        crate::export::InterfaceType::Ethernet
    );
    assert!(active.wifi.is_none());

    assert_eq!(others.len(), 3);
    let other_names: Vec<String> = others.into_iter().map(|s| s.friendly_name).collect();
    assert!(other_names.contains(&"vEthernet (Default Switch)".to_string()));
    assert!(other_names.contains(&"Wi-Fi".to_string()));
    assert!(other_names.contains(&"Loopback Pseudo-Interface 1".to_string()));
}

#[test]
fn test_no_active_adapter() {
    let down_adapter = RawAdapterInfo {
        name: "{down}".to_string(),
        description: "Ethernet Controller".to_string(),
        friendly_name: "Ethernet".to_string(),
        interface_type: crate::export::InterfaceType::Ethernet,
        is_physical: true,
        is_up: false,
        routing_metric: 25,
        link_speed_bps: 0,
        mac_address: "AA:BB:CC:DD:EE:FF".to_string(),
        ipv4_addresses: vec!["192.168.1.50/24".to_string()],
        ipv6_addresses: vec![],
        default_gateways: vec!["192.168.1.1".to_string()],
        dns_servers: vec![],
        dhcp_enabled: false,
        dhcp_server: None,
        mtu: 1500,
        wifi: None,
    };

    let apipa_adapter = RawAdapterInfo {
        name: "{apipa}".to_string(),
        description: "Wi-Fi Adapter".to_string(),
        friendly_name: "Wi-Fi".to_string(),
        interface_type: crate::export::InterfaceType::Wifi,
        is_physical: true,
        is_up: true,
        routing_metric: 25,
        link_speed_bps: 54_000_000,
        mac_address: "AA:BB:CC:DD:EE:00".to_string(),
        ipv4_addresses: vec!["169.254.10.20/16".to_string()],
        ipv6_addresses: vec![],
        default_gateways: vec![],
        dns_servers: vec![],
        dhcp_enabled: true,
        dhcp_server: None,
        mtu: 1500,
        wifi: None,
    };

    let (active, others) = select_active_adapter(vec![down_adapter, apipa_adapter]);
    assert!(active.is_none());
    assert_eq!(others.len(), 2);
}

#[test]
fn test_packet_loss_tracker_latencies_and_jitter() {
    let mut tracker = PacketLossTracker::default();
    tracker.record(ProbeResult::Success {
        latency: Duration::from_millis(10),
    });
    tracker.record(ProbeResult::Success {
        latency: Duration::from_millis(15),
    });
    tracker.record(ProbeResult::Timeout);
    tracker.record(ProbeResult::Success {
        latency: Duration::from_millis(12),
    });

    let lats = tracker.latencies_ms();
    assert_eq!(lats, vec![10.0, 15.0, 12.0]);

    let jitter = crate::export::calculate_mean_absolute_rtt_difference(&lats);
    assert_eq!(jitter, 4.0);
}

#[test]
fn test_counter_rollover_and_discontinuity_matrix() {
    // 1. Normal increasing counter
    assert_eq!(counter_delta(100, 250), Some(150));
    assert_eq!(counter_delta(1_000_000, 1_050_000), Some(50_000));

    // 2. Identical reading (zero delta)
    assert_eq!(counter_delta(250, 250), Some(0));

    // 3. Legitimate 32-bit counter rollover
    let u32_max = u32::MAX as u64;
    assert_eq!(counter_delta(u32_max - 1000, 500), Some(1501));
    assert_eq!(counter_delta(u32_max, 0), Some(1));
    assert_eq!(counter_delta(u32_max - 50_000, 50_000), Some(100_001));

    // 4. Legitimate 64-bit counter rollover
    assert_eq!(counter_delta(u64::MAX - 2000, 1000), Some(3001));
    assert_eq!(counter_delta(u64::MAX, 0), Some(1));

    // 5. Counter reset to zero (e.g. adapter reset/driver reload)
    // Must return None so 0 bytes are credited instead of a fake multi-gigabyte spike!
    assert_eq!(counter_delta(500_000_000, 0), None);
    assert_eq!(counter_delta(10_000_000_000, 0), None);

    // 6. Interface restart / arbitrary negative jump
    assert_eq!(counter_delta(8_000_000, 100_000), None);
    assert_eq!(counter_delta(2_000_000_000, 500), None);

    // 7. Rollover delta exceeding plausible rate threshold (> 500 MB in 1 tick)
    assert_eq!(counter_delta(u32_max - 600_000_000, 500), None);
}

#[test]
fn test_72h_continuous_telemetry_stress() {
    let mut backend = NetworkBackend::new();
    let start_time = Instant::now();

    let mut expected_rx = 0u64;
    let mut expected_tx = 0u64;

    let mut iface_rx = 1_000_000u64;
    let mut iface_tx = 500_000u64;

    let total_ticks: usize = 259_200; // 72 hours at 1 Hz (72 * 3600)

    // Baseline sample at t = 0
    let ifaces = vec![mock_iface(
        1,
        InterfaceCategory::Physical,
        InterfaceMedium::Ethernet,
        1,
        iface_rx,
        iface_tx,
    )];
    backend.sample_from_interfaces(&ifaces, start_time);

    for tick in 1..=total_ticks {
        let t = start_time + Duration::from_secs(tick as u64);

        if tick == 50_000 {
            // Scenario 1: Legitimate 32-bit counter rollover
            let prev_rx = u32::MAX as u64 - 10_000;
            let curr_rx = 5_000;
            let wrap_delta_rx = (u32::MAX as u64 - prev_rx) + curr_rx + 1; // 15,001 bytes

            let delta_tx = 5_000u64;
            iface_tx += delta_tx;

            let ifaces = vec![mock_iface(
                1,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                curr_rx,
                iface_tx,
            )];
            backend.prev_counters.get_mut(&1).unwrap().rx_bytes = prev_rx;

            backend.sample_from_interfaces(&ifaces, t);
            expected_rx += wrap_delta_rx;
            expected_tx += delta_tx;
            iface_rx = curr_rx;
        } else if tick == 100_000 {
            // Scenario 2: Legitimate 64-bit counter rollover
            let prev_rx = u64::MAX - 20_000;
            let curr_rx = 10_000;
            let wrap_delta_rx = (u64::MAX - prev_rx) + curr_rx + 1; // 30,001 bytes

            let delta_tx = 5_000u64;
            iface_tx += delta_tx;

            let ifaces = vec![mock_iface(
                1,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                curr_rx,
                iface_tx,
            )];
            backend.prev_counters.get_mut(&1).unwrap().rx_bytes = prev_rx;

            backend.sample_from_interfaces(&ifaces, t);
            expected_rx += wrap_delta_rx;
            expected_tx += delta_tx;
            iface_rx = curr_rx;
        } else if tick == 150_000 {
            // Scenario 3: Counter reset to zero (e.g. driver reload)
            iface_rx = 0;
            iface_tx = 0;
            let ifaces = vec![mock_iface(
                1,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                iface_rx,
                iface_tx,
            )];
            backend.sample_from_interfaces(&ifaces, t);
            // expected totals unchanged, new baseline established
        } else if tick == 200_000 {
            // Scenario 4: Interface replacement (LUID 1 removed, LUID 2 added)
            let ifaces = vec![mock_iface(
                2,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                1_000_000,
                1_000_000,
            )];
            backend.sample_from_interfaces(&ifaces, t);
            iface_rx = 1_000_000;
            iface_tx = 1_000_000;
        } else if tick == 220_000 {
            // Scenario 5: Temporary invalid reading (0) on LUID 2
            let ifaces = vec![mock_iface(
                2,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                0,
                0,
            )];
            backend.sample_from_interfaces(&ifaces, t);
            iface_rx = 0;
            iface_tx = 0;
        } else {
            // Normal steady traffic
            let delta_rx = 10_000u64;
            let delta_tx = 5_000u64;
            iface_rx += delta_rx;
            iface_tx += delta_tx;

            let active_luid = if tick >= 200_000 { 2 } else { 1 };
            let ifaces = vec![mock_iface(
                active_luid,
                InterfaceCategory::Physical,
                InterfaceMedium::Ethernet,
                1,
                iface_rx,
                iface_tx,
            )];
            backend.sample_from_interfaces(&ifaces, t);
            expected_rx += delta_rx;
            expected_tx += delta_tx;
        }

        // Periodic invariant assertion every 1,000 samples
        if tick % 1_000 == 0 {
            assert!(
                backend.history.len() <= HISTORY_CAPACITY,
                "history capacity invariant violated at tick {}",
                tick
            );
            assert_eq!(
                backend.session_rx, expected_rx,
                "session_rx invariant violation at tick {}",
                tick
            );
            assert_eq!(
                backend.session_tx, expected_tx,
                "session_tx invariant violation at tick {}",
                tick
            );
        }
    }

    // Final steady-state assertions
    assert_eq!(backend.session_rx, expected_rx);
    assert_eq!(backend.session_tx, expected_tx);
    assert!(backend.history.len() <= HISTORY_CAPACITY);
}

#[test]
fn test_120_day_accounting_retention_simulation() {
    let mut store = crate::daily_usage::DailyUsageStore::new();

    let mut dates = Vec::new();
    let month_days = [(1, 31), (2, 28), (3, 31), (4, 30)];
    for (m, max_d) in month_days {
        for d in 1..=max_d {
            dates.push(format!("2026-{:02}-{:02}", m, d));
        }
    }
    assert_eq!(dates.len(), 120);

    for (day_idx, date) in dates.iter().enumerate() {
        let day_num = day_idx + 1;

        // Multiple intra-day deltas simulating morning, afternoon, evening
        store.record_usage_delta(date, 10_000_000, 5_000_000);
        store.record_usage_delta(date, 30_000_000, 15_000_000);
        store.record_usage_delta(date, 20_000_000, 10_000_000);

        if day_num <= 90 {
            assert_eq!(store.entries.len(), day_num);
        } else {
            // Retention boundary invariant: strictly capped at 90
            assert_eq!(store.entries.len(), crate::daily_usage::MAX_DAILY_ENTRIES);
            assert_eq!(store.entries.len(), 90);

            // Oldest day from (day_num - 90) must be the first entry
            let expected_oldest = &dates[day_num - 90];
            assert_eq!(&store.entries[0].date, expected_oldest);

            // Evicted dates must no longer be found
            let evicted_date = &dates[day_num - 91];
            assert!(store.get_entry(evicted_date).is_none());
        }

        // Verify current date has exact combined totals
        let entry = store
            .get_entry(date)
            .expect("current date must exist in store");
        assert_eq!(entry.rx_bytes, 60_000_000);
        assert_eq!(entry.tx_bytes, 30_000_000);

        // Verify strict ascending chronological ordering and uniqueness
        for window in store.entries.windows(2) {
            assert!(
                window[0].date < window[1].date,
                "Dates must be strictly ascending: {} vs {}",
                window[0].date,
                window[1].date
            );
        }
    }

    assert_eq!(store.entries.len(), 90);
    assert_eq!(store.entries.last().unwrap().date, "2026-04-30");
}

#[test]
fn test_icmp_pathological_sequences_finite_and_bounded() {
    // Sequence 1: Mixed dropouts and spikes
    let mut tracker = PacketLossTracker::default();
    let events = [
        ProbeResult::Success {
            latency: Duration::from_millis(10),
        },
        ProbeResult::Success {
            latency: Duration::from_millis(15),
        },
        ProbeResult::Timeout,
        ProbeResult::Success {
            latency: Duration::from_millis(20),
        },
        ProbeResult::Timeout,
        ProbeResult::Timeout,
        ProbeResult::Success {
            latency: Duration::from_millis(25),
        },
    ];
    for ev in events {
        tracker.record(ev);
    }
    let (loss, lost, total, health) = tracker.evaluate();
    assert_eq!(total, 7);
    assert_eq!(lost, 3);
    assert_eq!(loss, Some(43)); // 3/7 ~ 43%
    assert_eq!(health, LatencyHealth::Degraded);

    let lats = tracker.latencies_ms();
    assert_eq!(lats.len(), 4);
    let jitter = crate::export::calculate_mean_absolute_rtt_difference(&lats);
    assert!(!jitter.is_nan() && !jitter.is_infinite());
    assert!(jitter >= 0.0);

    // Sequence 2: 100% all success
    let mut all_ok = PacketLossTracker::default();
    for _ in 0..20 {
        all_ok.record(ProbeResult::Success {
            latency: Duration::from_millis(20),
        });
    }
    let (loss_ok, _, _, health_ok) = all_ok.evaluate();
    assert_eq!(loss_ok, Some(0));
    assert_eq!(health_ok, LatencyHealth::Healthy);
    let jitter_ok = crate::export::calculate_mean_absolute_rtt_difference(&all_ok.latencies_ms());
    assert_eq!(jitter_ok, 0.0);

    // Sequence 3: 100% all timeout
    let mut all_timeout = PacketLossTracker::default();
    for _ in 0..20 {
        all_timeout.record(ProbeResult::Timeout);
    }
    let (loss_to, _, _, health_to) = all_timeout.evaluate();
    assert_eq!(loss_to, Some(100));
    assert_eq!(health_to, LatencyHealth::Timeout);
    assert!(all_timeout.latencies_ms().is_empty());
    let jitter_to =
        crate::export::calculate_mean_absolute_rtt_difference(&all_timeout.latencies_ms());
    assert_eq!(jitter_to, 0.0);
    assert!(!jitter_to.is_nan());

    // Sequence 4: Single success surrounded by timeouts
    let mut single_ok = PacketLossTracker::default();
    for _ in 0..10 {
        single_ok.record(ProbeResult::Timeout);
    }
    single_ok.record(ProbeResult::Success {
        latency: Duration::from_millis(50),
    });
    for _ in 0..9 {
        single_ok.record(ProbeResult::Timeout);
    }
    let (loss_single, lost_single, total_single, health_single) = single_ok.evaluate();
    assert_eq!(total_single, 20);
    assert_eq!(lost_single, 19);
    assert_eq!(loss_single, Some(95));
    assert_eq!(health_single, LatencyHealth::Degraded);
    assert_eq!(single_ok.latencies_ms().len(), 1);
    let jitter_single =
        crate::export::calculate_mean_absolute_rtt_difference(&single_ok.latencies_ms());
    assert_eq!(jitter_single, 0.0);
    assert!(!jitter_single.is_nan() && !jitter_single.is_infinite());

    // Sequence 5: Massive latency spike
    let mut spike = PacketLossTracker::default();
    for _ in 0..10 {
        spike.record(ProbeResult::Success {
            latency: Duration::from_millis(5),
        });
    }
    spike.record(ProbeResult::Success {
        latency: Duration::from_millis(10_000),
    });
    for _ in 0..9 {
        spike.record(ProbeResult::Success {
            latency: Duration::from_millis(5),
        });
    }
    let lats_spike = spike.latencies_ms();
    let jitter_spike = crate::export::calculate_mean_absolute_rtt_difference(&lats_spike);
    assert!(jitter_spike > 0.0);
    assert!(!jitter_spike.is_nan() && !jitter_spike.is_infinite());

    // Sequence 6: Rolling buffer never exceeds 20 samples
    let mut bounded = PacketLossTracker::default();
    for _ in 0..100 {
        bounded.record(ProbeResult::Success {
            latency: Duration::from_millis(10),
        });
    }
    assert_eq!(bounded.count, 20);
    assert_eq!(bounded.latencies_ms().len(), 20);
}
