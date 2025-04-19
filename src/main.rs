use clap::Parser;
use evaltor::{bench::*, evaluatees, options::Options, Evaluation};
use std::{fs, time::Duration};

fn main() {
    fs::create_dir_all("/tmp/evaltor/").unwrap();
    let options = Options::parse();

    let hwmcc_appr = Benchmark::new("hwmcc_appr", "../mc-benchmark/hwmcc-appr", Format::Aig);
    let hwmcc19 = Benchmark::new("hwmcc19", "../mc-benchmark/hwmcc19/aig", Format::Aig);
    let hwmcc20 = Benchmark::new("hwmcc20", "../mc-benchmark/hwmcc20/aig", Format::Aig);
    let hwmcc24 = Benchmark::new("hwmcc24", "../mc-benchmark/hwmcc24/aig", Format::Aig);

    let bench = MultiBenchmark::new()
        .set_name("hwmcc2024")
        .add(hwmcc20)
        .add(hwmcc24);
    Evaluation::new(bench)
        .set_timeout(Duration::from_secs(options.timeout))
        .set_memory_limit(1024 * 1024 * 1024 * options.memout)
        .set_certify(options.certify)
        .add_evaluatee(evaluatees::ric3::IC3)
        .evaluate();
}
