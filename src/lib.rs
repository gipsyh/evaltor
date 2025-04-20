#![feature(thread_id_value)]

pub mod bench;
pub mod evaluatees;
pub mod options;
mod worker;

use bench::BenchIF;
use chrono::Local;
use evaluatees::Evaluatee;
use regex::Regex;
use std::{sync::Arc, thread::spawn, time::Duration};
use worker::{Share, Worker};

pub struct Evaltor {
    benchmark: Box<dyn BenchIF>,
    evaluatees: Vec<Arc<dyn Evaluatee>>,
    timeout: Duration,
    memory_limit: usize,
    num_worker: usize,
    certify: bool,
    exclude: Vec<Regex>,
}

impl Evaltor {
    pub fn new(benchmark: Box<dyn BenchIF>) -> Self {
        Self {
            benchmark,
            evaluatees: Vec::new(),
            timeout: Duration::from_secs(1000),
            num_worker: num_cpus::get(),
            memory_limit: 1024 * 1024 * 1024,
            certify: false,
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

    pub fn set_num_worker(mut self, num_worker: usize) -> Self {
        self.num_worker = num_worker;
        self
    }

    pub fn set_certify(mut self, certify: bool) -> Self {
        self.certify = certify;
        self
    }

    pub fn add_evaluatee(mut self, evaluatee: impl Evaluatee + 'static) -> Self {
        self.evaluatees.push(Arc::new(evaluatee));
        self
    }

    pub fn evaluate(self) {
        for evaluatee in self.evaluatees.iter() {
            let num_worker = self.num_worker / evaluatee.parallelism();
            let file = if let Some(version) = evaluatee.version() {
                format!(
                    "result/{}-{}-{}-{}",
                    evaluatee.name(),
                    version,
                    self.benchmark.name(),
                    Local::now().format("%m%d%H%M"),
                )
            } else {
                format!(
                    "result/{}-{}-{}",
                    evaluatee.name(),
                    self.benchmark.name(),
                    Local::now().format("%m%d%H%M"),
                )
            };
            // let mut cases = self.benchmark.cases();
            // cases.retain(|f| {
            //     self.exclude
            //         .iter()
            //         .all(|r| !r.is_match(f.to_str().unwrap()))
            // });
            let share = Share::new(
                &*self.benchmark,
                file,
                self.timeout,
                self.memory_limit,
                self.certify,
            );
            let mut joins = Vec::new();
            for _ in 0..num_worker {
                let worker = Worker::new(evaluatee.clone(), share.clone());
                joins.push(spawn(|| worker.start()));
            }
            for join in joins {
                let _ = join.join();
            }
        }
    }
}
