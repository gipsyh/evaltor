#![feature(thread_id_value)]

mod evaluatees;
mod worker;

use chrono::Local;
use std::{fs::read_dir, process::Command, sync::Arc, thread::spawn, time::Duration};
use worker::{Share, Worker};

pub struct Benchmark {
    name: String,
    path: String,
    suffix: String,
}

impl Benchmark {
    pub fn new(name: &str, path: &str, suffix: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            suffix: suffix.to_string(),
        }
    }

    fn inner_cases(&self, path: &str) -> Vec<String> {
        let mut cases = Vec::new();
        for entry in read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension.eq_ignore_ascii_case(&self.suffix) {
                        cases.push(path.to_str().unwrap().to_string());
                    }
                }
            } else if path.is_dir() {
                let sub_cases = self.inner_cases(path.to_str().unwrap());
                cases.extend(sub_cases);
            }
        }
        cases.sort();
        cases
    }

    pub fn caces(&self) -> Vec<String> {
        self.inner_cases(&self.path)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EvaluationResult {
    Success(Duration),
    Timeout,
    Failed,
}

pub trait Evaluatee: Send + Sync {
    fn name(&self) -> String;

    fn version(&self) -> String;

    fn evaluate(&self, path: &str) -> Command;
}

pub struct Evaluation {
    benchmark: Benchmark,
    evaluatees: Vec<Arc<dyn Evaluatee>>,
    timeout: Duration,
    memory_limit: usize,
    test_cores: usize,
}

impl Evaluation {
    pub fn new(benchmark: Benchmark) -> Self {
        Self {
            benchmark,
            evaluatees: Vec::new(),
            timeout: Duration::from_secs(1000),
            test_cores: num_cpus::get(),
            memory_limit: 1024 * 1024 * 1024,
        }
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
            let file = format!(
                "result/{}-{}-{}-{}",
                evaluatee.name(),
                self.benchmark.name,
                Local::now().format("%m%d%H%M"),
                evaluatee.version(),
            );
            let share = Arc::new(Share::new(
                self.benchmark.caces(),
                file,
                self.timeout,
                self.memory_limit,
            ));
            let mut joins = Vec::new();
            for _ in 0..self.test_cores {
                let worker = Worker::new(evaluatee.clone(), share.clone());
                joins.push(spawn(|| worker.start()));
            }
            for join in joins {
                let _ = join.join();
            }
        }
    }
}

#[allow(unused)]
fn main() {
    let hwmcc_appr = Benchmark::new("hwmcc_appr", "../mc-benchmark/hwmcc-appr", "aag");
    let hwmcc1517 = Benchmark::new("hwmcc1517", "../mc-benchmark/hwmcc1517", "aag");
    let hwmcc1920 = Benchmark::new("hwmcc1920", "../mc-benchmark/hwmcc1920/btor2", "btor2");
    let xepic = Benchmark::new("xepic", "/root/mc-benchmark/x-epic-2024/btor2", "btor2");

    let mut evaluation = Evaluation::new(hwmcc1920);
    evaluation.set_timeout(Duration::from_secs(10));
    evaluation.set_memory_limit(1024 * 1024 * 1024 * 64);
    evaluation.add_evaluatee(evaluatees::avr::IC3);
    evaluation.evaluate();
}
