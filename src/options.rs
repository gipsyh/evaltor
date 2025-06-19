use clap::Parser;
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

use crate::{
    bench::{BenchIF, Benchmark, MultiBenchmark},
    evaluatees::{Evaluatee, EvaluateeIF},
};

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

    /// evaluatee configuration file in toml format
    pub evaluatee_config: PathBuf,

    /// evaluatee
    #[arg(short, long, value_delimiter = ',', required = true)]
    pub evaluatee: Vec<String>,

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

impl Options {
    pub fn parse_bench(&self) -> Result<Box<dyn BenchIF>, Box<dyn std::error::Error>> {
        let bench_config: HashMap<String, HashMap<String, PathBuf>> =
            toml::de::from_str(&fs::read_to_string(&self.bench_config).unwrap())?;
        let mut benchs: Vec<Box<dyn BenchIF>> = Vec::new();
        for b in self.bench.iter() {
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
                bench = bench.add_bench(b);
            }
            Box::new(bench)
        };
        Ok(bench)
    }

    pub fn parse_evaluatee(&self) -> Result<Vec<Arc<dyn EvaluateeIF>>, Box<dyn std::error::Error>> {
        #[derive(serde::Deserialize, Debug)]
        struct TomlEvaluatee {
            cmd: PathBuf,
            args: Vec<String>,
            parallelism: Option<usize>,
        }
        #[derive(serde::Deserialize, Debug)]
        struct TomlEvaluatees {
            name: String,
            exit_code: HashMap<String, String>,
            #[serde(flatten)]
            evaluatees: HashMap<String, TomlEvaluatee>,
        }

        let config: TomlEvaluatees =
            toml::de::from_str(&fs::read_to_string(&self.evaluatee_config).unwrap())?;
        let exit_code: HashMap<i64, String> = config
            .exit_code
            .into_iter()
            .map(|(k, v)| (k.parse::<i64>().unwrap(), v))
            .collect();
        let mut evaluatees: Vec<Arc<dyn EvaluateeIF>> = Vec::new();
        for e in self.evaluatee.iter() {
            let te = config.evaluatees.get(e).unwrap();
            let mut evaluatee = Evaluatee::new(&config.name, &te.cmd, &te.args);
            if let Some(parallelism) = te.parallelism {
                evaluatee.parallelism = parallelism;
            }
            evaluatee.version = e.to_string();
            evaluatee.exit_code = exit_code.clone();
            evaluatees.push(Arc::new(evaluatee));
        }
        Ok(evaluatees)
    }
}
