use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

struct CountingAlloc;

static TRACK_ALLOCS: AtomicBool = AtomicBool::new(false);
static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if TRACK_ALLOCS.load(Ordering::Relaxed) {
            ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if TRACK_ALLOCS.load(Ordering::Relaxed) {
            ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL: CountingAlloc = CountingAlloc;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

const SPARKLINE_SAMPLES: usize = net_flow_core::FLYOUT_SPARKLINE_SAMPLES;

#[test]
fn test_steady_state_sparkline_zero_heap_allocation() {
    use net_flow_core::backend::HistorySample;

    // 1. Prepare simulated rolling history
    let mut history = Vec::new();
    for i in 0..SPARKLINE_SAMPLES {
        let rx = (10_000 + (i % 7) * 5_000) as u64;
        let tx = (5_000 + (i % 5) * 3_000) as u64;
        history.push(HistorySample::new(rx, tx, 250_000_000));
    }

    // Stack storage for steady-state pipeline
    let mut sparkline_history = [(0.0_f32, 0.0_f32); SPARKLINE_SAMPLES];
    let mut rx_pts = [Point::default(); SPARKLINE_SAMPLES];
    let mut tx_pts = [Point::default(); SPARKLINE_SAMPLES];
    let mut smoothed_peak = 1024.0_f32;

    // 2. Warmup passes
    for _ in 0..10 {
        net_flow_core::update_flyout_sparkline(
            &history,
            &mut sparkline_history,
            &mut smoothed_peak,
            0.25,
        );

        // Simulate GDI point mapping
        let chart_l = 10;
        let chart_w = 300;
        let chart_b = 80;
        let chart_ch = 40;
        let n = SPARKLINE_SAMPLES;
        for (i, &(rx_norm, tx_norm)) in sparkline_history[..n].iter().enumerate() {
            let px = chart_l + ((i as i32 * chart_w) / (n.max(2) - 1) as i32);
            let py_rx = chart_b - (rx_norm * chart_ch as f32).round() as i32;
            let py_tx = chart_b - (tx_norm * chart_ch as f32).round() as i32;
            rx_pts[i] = Point { x: px, y: py_rx };
            tx_pts[i] = Point { x: px, y: py_tx };
        }
    }

    // 3. Reset allocation counter and activate tracking
    ALLOC_COUNT.store(0, Ordering::SeqCst);
    TRACK_ALLOCS.store(true, Ordering::SeqCst);

    // 4. Run 1,000 iterations of steady-state sparkline processing
    for _ in 0..1000 {
        net_flow_core::update_flyout_sparkline(
            &history,
            &mut sparkline_history,
            &mut smoothed_peak,
            0.25,
        );

        let chart_l = 10;
        let chart_w = 300;
        let chart_b = 80;
        let chart_ch = 40;
        let n = SPARKLINE_SAMPLES;
        for (i, &(rx_norm, tx_norm)) in sparkline_history[..n].iter().enumerate() {
            let px = chart_l + ((i as i32 * chart_w) / (n.max(2) - 1) as i32);
            let py_rx = chart_b - (rx_norm * chart_ch as f32).round() as i32;
            let py_tx = chart_b - (tx_norm * chart_ch as f32).round() as i32;
            rx_pts[i] = Point { x: px, y: py_rx };
            tx_pts[i] = Point { x: px, y: py_tx };
        }

        // Black box prevents compiler dead-code elimination
        std::hint::black_box(&rx_pts[..n]);
        std::hint::black_box(&tx_pts[..n]);
    }

    // 5. Deactivate tracking and verify ZERO allocations
    TRACK_ALLOCS.store(false, Ordering::SeqCst);
    let total_allocations = ALLOC_COUNT.load(Ordering::SeqCst);

    assert_eq!(
        total_allocations, 0,
        "Steady-state sparkline processing violated zero-heap-allocation contract: {} allocations detected across 1,000 iterations",
        total_allocations
    );
}
