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
        Configuration::rIC3 => ("aig", Format::Aig),
        Configuration::nuXmv => ("aig", Format::Aig),
        Configuration::ABC_pdr => ("aig1.8", Format::Aig),
        Configuration::Avy => ("aig1.8", Format::Aig),
        Configuration::IC3ref => ("aig1.8", Format::Aig),
        Configuration::AVR_ic3sa => ("bv", Format::Btor),
        Configuration::Pono_ic3ia => ("bv", Format::Btor),
        Configuration::Pono_ic3sa => ("bv", Format::Btor),
        Configuration::rIC3_portfolio => ("aig", Format::Aig),
        Configuration::ABC_superprove => ("aig", Format::Aig),
        Configuration::Pavy => ("aig1.8", Format::Aig),
        Configuration::AVR_portfolio => ("bv", Format::Btor),
        Configuration::Pono_portfolio => ("bv", Format::Btor),
    };

    let hwmcc19 = Benchmark::new("hwmcc19", &format!("../mc-benchmark/hwmcc19/{suf}"), format);
    let hwmcc20 = Benchmark::new("hwmcc20", &format!("../mc-benchmark/hwmcc20/{suf}"), format);
    let hwmcc24 = Benchmark::new("hwmcc24", &format!("../mc-benchmark/hwmcc24/{suf}"), format);
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
        Configuration::rIC3 => Arc::new(evaluatees::ric3::RIC3),
        Configuration::nuXmv => Arc::new(evaluatees::nuxmv::IGoodLemma),
        Configuration::ABC_pdr => Arc::new(evaluatees::abc::Pdr),
        Configuration::Avy => Arc::new(evaluatees::avy::Avy),
        Configuration::IC3ref => Arc::new(evaluatees::ic3ref::Ic3Ref),
        Configuration::AVR_ic3sa => Arc::new(evaluatees::avr::IC3),
        Configuration::Pono_ic3ia => Arc::new(evaluatees::pono::IC3ia),
        Configuration::Pono_ic3sa => Arc::new(evaluatees::pono::IC3sa),
        Configuration::rIC3_portfolio => Arc::new(evaluatees::ric3::Portfolio),
        Configuration::ABC_superprove => Arc::new(evaluatees::abc::SuperProve),
        Configuration::Pavy => Arc::new(evaluatees::avy::Pavy),
        Configuration::AVR_portfolio => Arc::new(evaluatees::avr::Portfolio),
        Configuration::Pono_portfolio => Arc::new(evaluatees::pono::Portfolio),
    };

    let mut evaltor = Evaluation::new(bench)
        .set_timeout(Duration::from_secs(options.timeout))
        .set_memory_limit(1024 * 1024 * 1024 * options.memout);
    evaltor.evaluatees.push(config);
    evaltor.evaluate();
}
