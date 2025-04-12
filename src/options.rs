use clap::{Parser, ValueEnum};

/// Evaltor
#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about,
    after_help = "Copyright (C) 2023 - Present, Yuheng Su <gipsyh.icu@gmail.com>. All rights reserved."
)]
pub struct Options {
    /// config
    #[arg(short, long, value_enum)]
    pub config: Configuration,

    /// benchmark
    #[arg(short, long, value_enum, default_value_t = HWMCCBench::Subset)]
    pub bench: HWMCCBench,

    /// time limit
    #[arg(short, long, default_value_t = 3600)]
    pub timeout: u64,

    /// memory limit
    #[arg(short, long, default_value_t = 16)]
    pub memout: usize,
}

#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum HWMCCBench {
    HWMCC19,
    HWMCC20,
    HWMCC24,
    HWMCC192024,
    Subset,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum Configuration {
    ric3,
    nuxmv,
    abc_pdr,
    avy,
    ic3ref,
    avr_ic3sa,
    pono_ic3ia,
    pono_ic3sa,
    ric3_portfolio,
    abc_superprove,
    pavy,
    avr_portfolio,
    pono_portfolio,
}
