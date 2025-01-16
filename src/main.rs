use evaluator::{
    bench::{Benchmark, MultiBenchmark},
    evaluatees, Evaluation,
};
use std::time::Duration;

#[allow(unused)]
fn main() {
    let hwmcc_appr = Benchmark::new("hwmcc_appr", "../mc-benchmark/hwmcc-appr", "aig");
    let hwmcc19 = Benchmark::new("hwmcc19", "../mc-benchmark/hwmcc19/aig", "aig");
    let hwmcc20 = Benchmark::new("hwmcc20", "../mc-benchmark/hwmcc20/aig", "aig");
    let hwmcc24 = Benchmark::new("hwmcc24", "../mc-benchmark/hwmcc24/aig", "aig");
    let hwmcc20_unsafe = Benchmark::new(
        "hwmcc20-unsafe",
        "../mc-benchmark/hwmcc20/aig-unsafe",
        "aig",
    );

    let bench = MultiBenchmark::new()
        .set_name("hwmcc2024")
        // .add(hwmcc19)
        .add(hwmcc20)
        .add(hwmcc24);
    Evaluation::new(bench)
        .set_timeout(Duration::from_secs(3600))
        .set_memory_limit(1024 * 1024 * 1024 * 16)
        .add_evaluatee(evaluatees::ric3::IC3Minisat)
        .evaluate();
    // evaluation.exclude(r"cal(?:2|156|192|201|206|209|210|220|224|227|234)\.aig$");
    // evaluation.exclude(r"mul[123]\.aig$");
}
