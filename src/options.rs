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
    rIC3,
    nuXmv,
    ABC_pdr,
    Avy,
    IC3ref,
    AVR_ic3sa,
    Pono_ic3ia,
    Pono_ic3sa,
    rIC3_portfolio,
    ABC_superprove,
    Pavy,
    AVR_portfolio,
    Pono_portfolio,
}
