use clap::Parser;
use evaltor::{
    bench::{fuzz::FuzzBench, *},
    evaluatees,
    options::Options,
    Evaltor,
};
use std::{fs, time::Duration};

fn main() {
    fs::create_dir_all("/tmp/evaltor/").unwrap();
    let options = Options::parse();

    let hwmcc19: Box<dyn BenchIF> = Box::new(Benchmark::new(
        "hwmcc19",
        "../mc-benchmark/hwmcc19/aig",
        Format::Aig,
    ));
    let hwmcc20: Box<dyn BenchIF> = Box::new(Benchmark::new(
        "hwmcc20",
        "../mc-benchmark/hwmcc20/aig",
        Format::Aig,
    ));
    let hwmcc24: Box<dyn BenchIF> = Box::new(Benchmark::new(
        "hwmcc24",
        "../mc-benchmark/hwmcc24/aig",
        Format::Aig,
    ));
    let bench: Box<dyn BenchIF> = match options.bench {
        evaltor::options::Bench::Hwmcc19 => hwmcc19,
        evaltor::options::Bench::Hwmcc20 => hwmcc20,
        evaltor::options::Bench::Hwmcc24 => hwmcc24,
        evaltor::options::Bench::Hwmcc2024 => Box::new(
            MultiBenchmark::new()
                .set_name("hwmcc2024")
                .add(hwmcc20)
                .add(hwmcc24),
        ),
        evaltor::options::Bench::Hwmcc192024 => Box::new(
            MultiBenchmark::new()
                .set_name("hwmcc2024")
                .add(hwmcc19)
                .add(hwmcc20)
                .add(hwmcc24),
        ),
        evaltor::options::Bench::Fuzz => Box::new(FuzzBench::new(10000)),
    };
    let mut evaltor = Evaltor::new(bench)
        .set_timeout(Duration::from_secs(options.timeout))
        .set_memory_limit(1024 * 1024 * 1024 * options.memout)
        .set_certify(options.certify)
        .add_evaluatee(evaluatees::ric3::IC3);
    if let Some(w) = options.num_worker {
        evaltor = evaltor.set_num_worker(w);
    }
    evaltor.evaluate();
}
