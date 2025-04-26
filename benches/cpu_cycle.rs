// Benchmark for measuring CPU cycle usage
// Uses criterion for performance benchmarking

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mavis_core::monitor::cpu::CpuMonitor;
use std::time::Duration;

fn cpu_monitor_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("CPU Monitoring");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(20);
    
    // Benchmark CPU usage measurement
    group.bench_function("get_usage", |b| {
        let mut monitor = CpuMonitor::new().expect("Failed to create CPU monitor");
        b.iter(|| {
            black_box(monitor.get_usage().expect("Failed to get CPU usage"));
        });
    });
    
    group.finish();
}

criterion_group!(benches, cpu_monitor_benchmark);
criterion_main!(benches);
