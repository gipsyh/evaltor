mod evaluatees;
mod worker;

use chrono::Local;
use process_control::{ChildExt, Control};
use std::{
    fs::{read_dir, File},
    path::Path,
    process::Command,
    sync::{Arc, Mutex},
    thread::spawn,
    time::{Duration, Instant},
};
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

    fn evaluate(&self, path: &str, timeout: Duration, memory_limit: usize) -> EvaluationResult;
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
            test_cores: 1,
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
            let result_file = format!(
                "result/{}-{}-{}",
                evaluatee.name(),
                self.benchmark.name,
                Local::now().format("%m-%d-%H-%M")
            );
            let res_file = File::create(Path::new(&result_file)).unwrap();
            let share = Arc::new(Share {
                cases: Mutex::new(self.benchmark.caces()),
                res_file: Mutex::new(res_file),
                timeout: self.timeout,
                memory_limit: self.memory_limit,
            });
            let mut joins = Vec::new();
            for _ in 0..self.test_cores {
                let worker = Worker::new(evaluatee.clone(), share.clone());
                joins.push(spawn(|| worker.start()));
            }
            for join in joins {
                join.join().unwrap();
            }
        }
    }
}

fn command_evaluate(
    mut command: Command,
    timeout: Duration,
    memory_limit: usize,
) -> EvaluationResult {
    let child = command.spawn().unwrap();
    let start = Instant::now();
    let output = child
        .controlled_with_output()
        .time_limit(timeout)
        .terminate_for_timeout()
        .memory_limit(memory_limit)
        .wait()
        .unwrap();
    if let Some(output) = output {
        if output.status.success() {
            EvaluationResult::Success(start.elapsed())
        } else {
            EvaluationResult::Failed
        }
    } else {
        EvaluationResult::Timeout
    }
}

fn main() {
    let name = "hwmcc15";
    let path = "/root/MC-Benchmark/hwmcc15";
    // let path = "/root/MC-Benchmark/hwmcc17/single";
    // let path = "/root/MC-Benchmark/hwmcc20/aig/2019/goel/";
    // let path = "/root/MC-Benchmark/hwmcc-appr";
    let suffix = "aag";

    let benchmark = Benchmark::new(name, path, suffix);
    let mut evaluation = Evaluation::new(benchmark);
    evaluation.set_timeout(Duration::from_secs(2000));
    evaluation.set_memory_limit(2 * 1024 * 1024 * 1024);
    evaluation.add_evaluatee(evaluatees::myic3::MyIc3);
    evaluation.set_test_cores(16);
    evaluation.evaluate();
}
