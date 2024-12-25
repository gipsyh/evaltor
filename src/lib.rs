#![feature(thread_id_value)]

pub mod bench;
pub mod evaluatees;
mod worker;

use bench::Benchmark;
use chrono::Local;
use regex::Regex;
use std::{process::Command, sync::Arc, thread::spawn, time::Duration};
use worker::{Share, Worker};

#[derive(Debug, Clone, Copy)]
pub enum EvaluationResult {
    Success(Duration),
    Timeout,
    Failed,
}

pub trait Evaluatee: Send + Sync {
    fn name(&self) -> String;

    fn version(&self) -> String {
        "v0".to_string()
    }

    fn evaluate(&self, path: &str) -> Command;

    fn parallelism(&self) -> usize {
        1
    }
}

pub struct Evaluation {
    benchmark: Benchmark,
    evaluatees: Vec<Arc<dyn Evaluatee>>,
    timeout: Duration,
    memory_limit: usize,
    test_cores: usize,
    exclude: Vec<Regex>,
}

impl Evaluation {
    pub fn new(benchmark: Benchmark) -> Self {
        Self {
            benchmark,
            evaluatees: Vec::new(),
            timeout: Duration::from_secs(1000),
            test_cores: num_cpus::get(),
            memory_limit: 1024 * 1024 * 1024,
            exclude: Vec::new(),
        }
    }

    pub fn exclude(&mut self, e: &str) {
        self.exclude.push(Regex::new(e).unwrap())
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout
    }

    pub fn set_memory_limit(&mut self, memory_limit: usize) {
        self.memory_limit = memory_limit
    }

    pub fn set_test_cores(&mut self, test_cores: usize) {
        self.test_cores = test_cores
    }

    pub fn add_evaluatee(&mut self, evaluatee: impl Evaluatee + 'static) {
        self.evaluatees.push(Arc::new(evaluatee));
    }

    pub fn evaluate(&mut self) {
        for evaluatee in self.evaluatees.iter() {
            let test_cores = self.test_cores / evaluatee.parallelism();
            let file = format!(
                "result/{}-{}-{}-{}",
                evaluatee.name(),
                self.benchmark.name(),
                Local::now().format("%m%d%H%M"),
                evaluatee.version(),
            );
            let mut cases = self.benchmark.cases();
            cases.retain(|f| self.exclude.iter().all(|r| !r.is_match(f)));
            let share = Arc::new(Share::new(cases, file, self.timeout, self.memory_limit));
            let mut joins = Vec::new();
            for _ in 0..test_cores {
                let worker = Worker::new(evaluatee.clone(), share.clone());
                joins.push(spawn(|| worker.start()));
            }
            for join in joins {
                let _ = join.join();
            }
        }
    }
}
