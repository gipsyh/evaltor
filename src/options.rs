use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Evaltor
#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about,
    after_help = "Copyright (C) 2023 - Present, Yuheng Su <gipsyh.icu@gmail.com>. All rights reserved."
)]
pub struct Options {
    /// benchmark configuration file in toml format
    pub bench_config: PathBuf,

    /// benchmark
    #[arg(short, long, value_delimiter = ',', required = true)]
    pub bench: Vec<String>,

    /// engine configuration file in toml format
    pub engine_config: PathBuf,

    /// time limit
    #[arg(short, long, default_value_t = 3600)]
    pub timeout: u64,

    /// memory limit
    #[arg(short, long, default_value_t = 16)]
    pub memout: usize,

    /// certify
    #[arg(short, long, default_value_t = false)]
    pub certify: bool,

    /// number of workers
    #[arg(short = 'w', long)]
    pub num_worker: Option<usize>,

    /// number of fuzz tests
    #[arg(short = 'f', long, default_value_t = 1000)]
    pub num_fuzz: usize,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Bench {
    Hwmcc19,
    Hwmcc20,
    Hwmcc24,
    Hwmcc2024,
    Hwmcc192024,
    Fuzz,
}
