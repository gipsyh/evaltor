use clap::Parser;
use evaltor::{bench::*, evaluatees, options::Options, Evaluation};
use std::time::Duration;

fn main() {
    let options = Options::parse();

    let hwmcc_appr = Benchmark::new("hwmcc_appr", "../mc-benchmark/hwmcc-appr", Format::Aig);
    let hwmcc19 = Benchmark::new("hwmcc19", "../mc-benchmark/hwmcc19/aig", Format::Aig);
    let hwmcc20 = Benchmark::new("hwmcc20", "../mc-benchmark/hwmcc20/aig1.8", Format::Aig);
    let hwmcc24 = Benchmark::new("hwmcc24", "../mc-benchmark/hwmcc24/aig1.8", Format::Aig);

    let bench = MultiBenchmark::new()
        .set_name("hwmcc2024")
        .add(hwmcc20)
        .add(hwmcc24);
    Evaluation::new(bench)
        .set_timeout(Duration::from_secs(options.timeout))
        .set_memory_limit(1024 * 1024 * 1024 * options.memout)
        .add_evaluatee(evaluatees::abc::Pdr)
        .evaluate();
}
