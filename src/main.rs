use clap::Parser;
use evaltor::{Evaltor, options::Options};
use std::{fs, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("/tmp/evaltor/").unwrap();
    let options = Options::parse();
    let mut evaltor = Evaltor::new(options.parse_bench()?);
    evaltor
        .set_timeout(Duration::from_secs(options.timeout))
        .set_memory_limit(1024 * 1024 * 1024 * options.memout)
        .set_certify(options.certify);
    for e in options.parse_evaluatee()? {
        evaltor.add_evaluatee(e);
    }
    if let Some(w) = options.num_worker {
        evaltor.set_num_worker(w);
    }
    evaltor.evaluate();
    Ok(())
}
