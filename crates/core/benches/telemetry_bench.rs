use net_flow_core::{
    HISTORY_CAPACITY, InterfaceCategory, InterfaceInfo, NetworkBackend, SAMPLING_INTERVAL_MS,
    WidgetConfig, backend::InterfaceMedium, build_adaptive_card,
    chart::{render_idle_unified_chart_data_uri, render_unified_chart_data_uri, render_unified_dual_chart_png},
};
use std::time::{Duration, Instant};

fn percentile(sorted: &[f64], pct: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = ((sorted.len() - 1) as f64 * pct).round() as usize;
    sorted[idx]
}

fn stats(durations_us: &mut [f64]) -> (f64, f64, f64, f64, f64) {
    durations_us.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let min = durations_us.first().copied().unwrap_or(0.0);
    let p50 = percentile(durations_us, 0.50);
    let p99 = percentile(durations_us, 0.99);
    let max = durations_us.last().copied().unwrap_or(0.0);
    let sum: f64 = durations_us.iter().sum();
    let mean = sum / durations_us.len() as f64;
    (min, p50, mean, p99, max)
}

fn make_iface(in_octets: u64, out_octets: u64) -> InterfaceInfo {
    InterfaceInfo {
        luid: 1,
        index: 1,
        name: "eth1".to_string(),
        description: "Mock Adapter".to_string(),
        category: InterfaceCategory::Physical,
        medium: InterfaceMedium::Ethernet,
        oper_status: 1,
        in_octets,
        out_octets,
        speed: 1_000_000_000,
    }
}

fn main() {
    println!("=== Net-Flow Telemetry Engine & Chart Optimization Benchmark (Phase 4) ===\n");

    let mut backend = NetworkBackend::new();
    let mut idle_backend = NetworkBackend::new();
    let mut t = Instant::now();
    let dt = Duration::from_millis(SAMPLING_INTERVAL_MS);

    // 1. Warm-up backends to full 240-sample capacity
    let mut rx_bytes = 0u64;
    let mut tx_bytes = 0u64;
    for _ in 0..HISTORY_CAPACITY {
        t += dt;
        rx_bytes += 12_500; // 50 KB/s active traffic
        tx_bytes += 6_250;
        let ifaces = vec![make_iface(rx_bytes, tx_bytes)];
        backend.sample_from_interfaces(&ifaces, t);

        let idle_ifaces = vec![make_iface(0, 0)];
        idle_backend.sample_from_interfaces(&idle_ifaces, t);
    }
    println!(
        "Warmed up backends: history buffers at full capacity ({}) samples",
        HISTORY_CAPACITY
    );

    // 2. Benchmark sample_from_interfaces (10,000 iterations)
    const SAMPLE_ITERS: usize = 10_000;
    let mut sample_durations = Vec::with_capacity(SAMPLE_ITERS);
    let mut last_snap = None;

    for i in 0..SAMPLE_ITERS {
        t += dt;
        rx_bytes += 12_500 + (i as u64 % 500);
        tx_bytes += 6_250 + (i as u64 % 250);
        let ifaces = vec![make_iface(rx_bytes, tx_bytes)];

        let start = Instant::now();
        let snap = backend.sample_from_interfaces(&ifaces, t);
        let elapsed = start.elapsed();
        sample_durations.push(elapsed.as_nanos() as f64 / 1000.0);
        last_snap = Some(snap);
    }

    let active_snap = last_snap.expect("active snapshot");
    let idle_snap = idle_backend.sample_from_interfaces(&[make_iface(0, 0)], t);

    let (s_min, s_p50, s_mean, s_p99, s_max) = stats(&mut sample_durations);

    // 3. Benchmark snapshot history cloning in isolation (10,000 iterations)
    let mut clone_durations = Vec::with_capacity(SAMPLE_ITERS);
    for _ in 0..SAMPLE_ITERS {
        let start = Instant::now();
        let _cloned = active_snap.history.clone();
        let elapsed = start.elapsed();
        clone_durations.push(elapsed.as_nanos() as f64 / 1000.0);
    }
    let (c_min, c_p50, c_mean, c_p99, c_max) = stats(&mut clone_durations);

    // 4. Benchmark Chart Rendering (Active vs Idle) (1,000 iterations)
    const CHART_ITERS: usize = 1_000;
    let mut chart_raw_durations = Vec::with_capacity(CHART_ITERS);
    let mut chart_active_uri_durations = Vec::with_capacity(CHART_ITERS);
    let mut chart_idle_uri_durations = Vec::with_capacity(CHART_ITERS);
    let mut chart_direct_idle_durations = Vec::with_capacity(CHART_ITERS);

    // Raw rasterizer + PNG (Active)
    for _ in 0..CHART_ITERS {
        let start = Instant::now();
        let _png = render_unified_dual_chart_png(&active_snap.history, 240, 480, 80).expect("render png");
        let elapsed = start.elapsed();
        chart_raw_durations.push(elapsed.as_nanos() as f64 / 1000.0);
    }

    // Active Data URI (Full rasterizer + base64)
    for _ in 0..CHART_ITERS {
        let start = Instant::now();
        let _uri = render_unified_chart_data_uri(&active_snap.history, "Medium", 60);
        let elapsed = start.elapsed();
        chart_active_uri_durations.push(elapsed.as_nanos() as f64 / 1000.0);
    }

    // Idle Data URI via history slice
    for _ in 0..CHART_ITERS {
        let start = Instant::now();
        let _uri = render_unified_chart_data_uri(&idle_snap.history, "Medium", 60);
        let elapsed = start.elapsed();
        chart_idle_uri_durations.push(elapsed.as_nanos() as f64 / 1000.0);
    }

    // Idle Data URI direct cache lookup
    for _ in 0..CHART_ITERS {
        let start = Instant::now();
        let _uri = render_idle_unified_chart_data_uri("Medium", 60);
        let elapsed = start.elapsed();
        chart_direct_idle_durations.push(elapsed.as_nanos() as f64 / 1000.0);
    }

    let (r_min, r_p50, r_mean, r_p99, r_max) = stats(&mut chart_raw_durations);
    let (au_min, au_p50, au_mean, au_p99, au_max) = stats(&mut chart_active_uri_durations);
    let (iu_min, iu_p50, iu_mean, iu_p99, iu_max) = stats(&mut chart_idle_uri_durations);
    let (di_min, di_p50, di_mean, di_p99, di_max) = stats(&mut chart_direct_idle_durations);

    // 5. Benchmark Adaptive Card construction (Active vs Idle) (1,000 iterations)
    let config = WidgetConfig::default();
    let mut card_active_durations = Vec::with_capacity(CHART_ITERS);
    let mut card_idle_durations = Vec::with_capacity(CHART_ITERS);

    for _ in 0..CHART_ITERS {
        let start = Instant::now();
        let _card_json = build_adaptive_card(&active_snap, "Medium", &config);
        let elapsed = start.elapsed();
        card_active_durations.push(elapsed.as_nanos() as f64 / 1000.0);
    }

    for _ in 0..CHART_ITERS {
        let start = Instant::now();
        let _card_json = build_adaptive_card(&idle_snap, "Medium", &config);
        let elapsed = start.elapsed();
        card_idle_durations.push(elapsed.as_nanos() as f64 / 1000.0);
    }

    let (cda_min, cda_p50, cda_mean, cda_p99, cda_max) = stats(&mut card_active_durations);
    let (cdi_min, cdi_p50, cdi_mean, cdi_p99, cdi_max) = stats(&mut card_idle_durations);

    // Print summary table
    println!("\n### Micro-Benchmark Results (Phase 4 Optimized)");
    println!(
        "| Pipeline Component | State / Cadence | Min (us) | Median (us) | Mean (us) | p99 (us) | Max (us) |"
    );
    println!("|---|---|---|---|---|---|---|");
    println!(
        "| `sample_from_interfaces` (total) | 250ms (4 Hz) | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} |",
        s_min, s_p50, s_mean, s_p99, s_max
    );
    println!(
        "|   `history.clone()` (240 items) | 250ms (4 Hz) | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} |",
        c_min, c_p50, c_mean, c_p99, c_max
    );
    println!(
        "| `render_unified_dual_chart_png` | Active (2 Hz) | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} |",
        r_min, r_p50, r_mean, r_p99, r_max
    );
    println!(
        "| `render_unified_chart_data_uri` | Active (2 Hz) | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} |",
        au_min, au_p50, au_mean, au_p99, au_max
    );
    println!(
        "| `render_unified_chart_data_uri` | IDLE (cached) | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} |",
        iu_min, iu_p50, iu_mean, iu_p99, iu_max
    );
    println!(
        "| `render_idle_unified_chart_data_uri` | IDLE (direct) | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} |",
        di_min, di_p50, di_mean, di_p99, di_max
    );
    println!(
        "| `build_adaptive_card` | Active (2 Hz) | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} |",
        cda_min, cda_p50, cda_mean, cda_p99, cda_max
    );
    println!(
        "| `build_adaptive_card` | IDLE (O(1) bypass) | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} |",
        cdi_min, cdi_p50, cdi_mean, cdi_p99, cdi_max
    );

    println!("\n### CPU Core Utilization Impact (Active vs Idle)");
    let telemetry_hz = 4.0;
    let ui_hz = 2.0;

    let telemetry_cpu_us_per_sec = s_mean * telemetry_hz;
    let active_ui_us_per_sec = cda_mean * ui_hz;
    let idle_ui_us_per_sec = cdi_mean * ui_hz;

    println!(
        "- Telemetry Sampling (4 Hz):    {:.2} us/sec ({:.5}% of 1 core)",
        telemetry_cpu_us_per_sec,
        telemetry_cpu_us_per_sec / 10_000.0
    );
    println!(
        "- UI Push ACTIVE (2 Hz):        {:.2} us/sec ({:.4}% of 1 core)",
        active_ui_us_per_sec,
        active_ui_us_per_sec / 10_000.0
    );
    println!(
        "- UI Push IDLE (2 Hz):          {:.2} us/sec ({:.5}% of 1 core)",
        idle_ui_us_per_sec,
        idle_ui_us_per_sec / 10_000.0
    );
    println!(
        "- **Total Active Core Overhead**: {:.2} us/sec ({:.4}% of 1 core)",
        telemetry_cpu_us_per_sec + active_ui_us_per_sec,
        (telemetry_cpu_us_per_sec + active_ui_us_per_sec) / 10_000.0
    );
    println!(
        "- **Total IDLE Core Overhead**:   {:.2} us/sec ({:.5}% of 1 core)",
        telemetry_cpu_us_per_sec + idle_ui_us_per_sec,
        (telemetry_cpu_us_per_sec + idle_ui_us_per_sec) / 10_000.0
    );
}
