#![feature(thread_id_value)]

pub mod bench;
pub mod evaluatees;
pub mod options;
mod worker;

use bench::MultiBenchmark;
use chrono::Local;
use evaluatees::Evaluatee;
use regex::Regex;
use std::{sync::Arc, thread::spawn, time::Duration};
use worker::{Share, Worker};

pub struct Evaluation {
    benchmark: MultiBenchmark,
    pub evaluatees: Vec<Arc<dyn Evaluatee>>,
    timeout: Duration,
    memory_limit: usize,
    test_cores: usize,
    exclude: Vec<Regex>,
}

impl Evaluation {
    pub fn new(benchmark: MultiBenchmark) -> Self {
        Self {
            benchmark,
            evaluatees: Vec::new(),
            timeout: Duration::from_secs(1000),
            test_cores: num_cpus::get(),
            memory_limit: 1024 * 1024 * 1024,
            exclude: Vec::new(),
        }
    }

    pub fn exclude(mut self, e: &str) -> Self {
        self.exclude.push(Regex::new(e).unwrap());
        self
    }

    pub fn set_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn set_memory_limit(mut self, memory_limit: usize) -> Self {
        self.memory_limit = memory_limit;
        self
    }

    pub fn set_test_cores(mut self, test_cores: usize) -> Self {
        self.test_cores = test_cores;
        self
    }

    pub fn add_evaluatee(mut self, evaluatee: impl Evaluatee + 'static) -> Self {
        self.evaluatees.push(Arc::new(evaluatee));
        self
    }

    pub fn evaluate(self) {
        for evaluatee in self.evaluatees.iter() {
            let test_cores = self.test_cores / evaluatee.parallelism();
            let file = format!(
                "result/{}-{}-{}-{}",
                evaluatee.name(),
                self.benchmark.name(),
                Local::now().format("%m%d%H%M"),
                evaluatee.version(),
            );
            // let mut cases = self.benchmark.cases();
            // cases.retain(|f| {
            //     self.exclude
            //         .iter()
            //         .all(|r| !r.is_match(f.to_str().unwrap()))
            // });
            let share = Arc::new(Share::new(
                &self.benchmark,
                file,
                self.timeout,
                self.memory_limit,
            ));
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
