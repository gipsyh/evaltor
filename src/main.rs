use clap::Parser;
use evaltor::{
    bench::*,
    evaluatees::{self, Evaluatee},
    options::{Configuration, HWMCCBench, Options},
    Evaluation,
};
use std::{sync::Arc, time::Duration};

fn main() {
    let options = Options::parse();
    let (suf, format) = match options.config {
        Configuration::ric3 => ("aig", Format::Aig),
        Configuration::nuxmv => ("aig", Format::Aig),
        Configuration::abc_pdr => ("aig1.8", Format::Aig),
        Configuration::avy => ("aig1.8", Format::Aig),
        Configuration::ic3ref => ("aig1.8", Format::Aig),
        Configuration::avr_ic3sa => ("bv", Format::Btor),
        Configuration::pono_ic3ia => ("bv", Format::Btor),
        Configuration::pono_ic3sa => ("bv", Format::Btor),
        Configuration::ric3_portfolio => ("aig", Format::Aig),
        Configuration::abc_superprove => ("aig", Format::Aig),
        Configuration::pavy => ("aig1.8", Format::Aig),
        Configuration::avr_portfolio => ("bv", Format::Btor),
        Configuration::pono_portfolio => ("bv", Format::Btor),
    };

    let hwmcc19 = Benchmark::new("hwmcc19", &format!("./benchmark/hwmcc19/{suf}"), format);
    let hwmcc20 = Benchmark::new("hwmcc20", &format!("./benchmark/hwmcc20/{suf}"), format);
    let hwmcc24 = Benchmark::new("hwmcc24", &format!("./benchmark/hwmcc24/{suf}"), format);
    let bench = match options.bench {
        HWMCCBench::HWMCC19 => MultiBenchmark::new().add(hwmcc19),
        HWMCCBench::HWMCC20 => MultiBenchmark::new().add(hwmcc20),
        HWMCCBench::HWMCC24 => MultiBenchmark::new().add(hwmcc24),
        HWMCCBench::HWMCC192024 => MultiBenchmark::new()
            .set_name("hwmcc192024")
            .add(hwmcc19)
            .add(hwmcc20)
            .add(hwmcc24),
        HWMCCBench::Subset => todo!(),
    };

    let config: Arc<dyn Evaluatee> = match options.config {
        Configuration::ric3 => Arc::new(evaluatees::ric3::RIC3),
        Configuration::nuxmv => Arc::new(evaluatees::nuxmv::IGoodLemma),
        Configuration::abc_pdr => Arc::new(evaluatees::abc::Pdr),
        Configuration::avy => Arc::new(evaluatees::avy::Avy),
        Configuration::ic3ref => Arc::new(evaluatees::ic3ref::Ic3Ref),
        Configuration::avr_ic3sa => Arc::new(evaluatees::avr::IC3),
        Configuration::pono_ic3ia => Arc::new(evaluatees::pono::IC3ia),
        Configuration::pono_ic3sa => Arc::new(evaluatees::pono::IC3sa),
        Configuration::ric3_portfolio => Arc::new(evaluatees::ric3::Portfolio),
        Configuration::abc_superprove => Arc::new(evaluatees::abc::SuperProve),
        Configuration::pavy => Arc::new(evaluatees::avy::Pavy),
        Configuration::avr_portfolio => Arc::new(evaluatees::avr::Portfolio),
        Configuration::pono_portfolio => Arc::new(evaluatees::pono::Portfolio),
    };

    let mut evaltor = Evaluation::new(bench)
        .set_timeout(Duration::from_secs(options.timeout))
        .set_memory_limit(1024 * 1024 * 1024 * options.memout);
    evaltor.evaluatees.push(config);
    evaltor.evaluate();
}
