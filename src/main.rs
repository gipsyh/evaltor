use evaltor::{bench::*, evaluatees, Evaluation};
use std::time::Duration;

fn main() {
    let hwmcc_appr = Benchmark::new("hwmcc_appr", "../mc-benchmark/hwmcc-appr", "aig");
    let hwmcc19 = Benchmark::new("hwmcc19", "../mc-benchmark/hwmcc19/aig", "aig");
    let hwmcc20 = Benchmark::new("hwmcc20", "../mc-benchmark/hwmcc20/aig1.8", "aig");
    let hwmcc24 = Benchmark::new("hwmcc24", "../mc-benchmark/hwmcc24/aig1.8", "aig");
    let hwmcc20_unsafe = Benchmark::new(
        "hwmcc20-unsafe",
        "../mc-benchmark/hwmcc20/aig-unsafe",
        "aig",
    );

    let bench = MultiBenchmark::new()
        .set_name("hwmcc2024")
        .add(hwmcc20)
        .add(hwmcc24);
    Evaluation::new(bench)
        .set_timeout(Duration::from_secs(3600))
        .set_memory_limit(1024 * 1024 * 1024 * 16)
        .add_evaluatee(evaluatees::abc::Pdr)
        .evaluate();
}
