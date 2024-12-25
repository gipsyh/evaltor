use std::time::Duration;
use evaluator::{bench::Benchmark, evaluatees, Evaluation};



#[allow(unused)]
fn main() {
    let hwmcc_appr = Benchmark::new("hwmcc_appr", "../mc-benchmark/hwmcc-appr", "aig");
    let hwmcc20 = Benchmark::new("hwmcc20", "../mc-benchmark/hwmcc20/aig", "aig");
    let hwmcc20_unsafe = Benchmark::new("hwmcc20-unsafe", "../mc-benchmark/hwmcc20/aig-unsafe", "aig");

    let mut evaluation = Evaluation::new(hwmcc20_unsafe);
    // evaluation.exclude(r"mul[123]\.aig$");
    // evaluation.exclude(r"cal(?:2|156|192|201|206|209|210|220|224|227|234)\.aig$");
    evaluation.set_timeout(Duration::from_secs(3600));
    evaluation.set_memory_limit(1024 * 1024 * 1024 * 16);
    evaluation.add_evaluatee(evaluatees::ric3::BMC);
    evaluation.evaluate();
}
