#![feature(thread_id_value)]

mod evaluatees;
mod worker;

use chrono::Local;
use regex::Regex;
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

    fn version(&self) -> String {
        "r0".to_string()
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
                self.benchmark.name,
                Local::now().format("%m%d%H%M"),
                evaluatee.version(),
            );
            let mut cases = self.benchmark.caces();
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

#[allow(unused)]
fn main() {
    let hwmcc_appr = Benchmark::new("hwmcc_appr", "../mc-benchmark/hwmcc-appr", "aig");
    let hwmcc1517 = Benchmark::new("hwmcc1517", "../mc-benchmark/hwmcc1517", "aig");
    let hwmcc1920 = Benchmark::new("hwmcc1920", "../mc-benchmark/hwmcc1920/aig", "aig");
    let hwmcc1920sat = Benchmark::new("hwmcc1920sat", "../mc-benchmark/hwmcc1920sat", "aig");
    let hwmcc1920uns = Benchmark::new("hwmcc1920uns", "../mc-benchmark/hwmcc1920uns", "aig");
    let hwmcc1920cal = Benchmark::new("hwmcc1920cal", "../mc-benchmark/hwmcc1920cal", "aig");
    let hwmcc1920ns = Benchmark::new("hwmcc1920ns", "../mc-benchmark/hwmcc1920ns", "aig");
    let hwmcc20 = Benchmark::new("hwmcc20", "../mc-benchmark/hwmcc20/aig", "aig");
    let avr = Benchmark::new("avr", "../mc-benchmark/avr", "aig");
    let cal = Benchmark::new("cal", "../mc-benchmark/avr/industry", "aig");
    let xepic = Benchmark::new("xepic", "/root/mc-benchmark/x-epic-2024/btor2", "btor2");
    let others = Benchmark::new("others", "/root/mc-benchmark/others/atxfifo/aig", "aig");
    let sat23 = Benchmark::new("sat23", "/root/sat23", "cnf");
    let ic3inn = Benchmark::new("ic3inn", "/root/innard-benchmarks", "aig");

    let mut evaluation = Evaluation::new(hwmcc20);
    evaluation.exclude(r"mul[123]\.aig$");
    evaluation.exclude(r"cal(?:2|156|192|201|206|209|210|220|224|227|234)\.aig$");
    evaluation.set_timeout(Duration::from_secs(3600));
    evaluation.set_memory_limit(1024 * 1024 * 1024 * 64);
    evaluation.add_evaluatee(evaluatees::ric3::Portfolio);
    evaluation.evaluate();
}
