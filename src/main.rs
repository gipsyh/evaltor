use clap::Parser;
use evaltor::{bench::*, evaluatees, options::Options, Evaltor};
use std::{collections::HashMap, fs, path::PathBuf, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("/tmp/evaltor/").unwrap();
    let options = Options::parse();
    let bench_config: HashMap<String, HashMap<String, PathBuf>> =
        toml::de::from_str(&fs::read_to_string(options.bench_config).unwrap())?;
    let mut benchs: Vec<Box<dyn BenchIF>> = Vec::new();
    for b in options.bench.iter() {
        let parts: Vec<&str> = b.split('.').collect();
        assert!(parts.len() == 2);
        let name = parts[0];
        let format = parts[1];
        let path = &bench_config[name][format];
        benchs.push(Box::new(Benchmark::new(name, path, format)));
    }
    let bench: Box<dyn BenchIF> = if benchs.len() == 1 {
        benchs.into_iter().next().unwrap()
    } else {
        let mut bench = MultiBenchmark::new();
        for b in benchs {
            bench = bench.add(b);
        }
        Box::new(bench)
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
    Ok(())
}
