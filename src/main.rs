mod evaluatees;
mod worker;

use chrono::Local;
use evaluatees::myic3::MyIc3;
use std::{
    fs::{read_dir, File},
    path::Path,
    process::Command,
    sync::{Arc, Mutex},
    thread::spawn,
    time::{Duration, Instant},
};
use wait_timeout::ChildExt;
use worker::{Share, Worker};

pub struct Benchmark {
    path: String,
    suffix: String,
}

impl Benchmark {
    pub fn new(path: &str, suffix: &str) -> Self {
        Self {
            path: path.to_string(),
            suffix: suffix.to_string(),
        }
    }

    pub fn caces(&self) -> impl Iterator<Item = String> + '_ {
        read_dir(&self.path).unwrap().filter_map(|entry| {
            if let Some(file_name) = entry.ok()?.path().to_str() {
                if file_name.ends_with(&self.suffix) {
                    return Some(file_name.to_owned());
                }
            }
            None
        })
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

    fn evaluate(&self, path: &str, timeout: Duration) -> EvaluationResult;
}

pub struct Evaluation {
    benchmark: Benchmark,
    evaluatees: Vec<Arc<dyn Evaluatee>>,
    timeout: Duration,
    test_cores: usize,
}

impl Evaluation {
    pub fn new(benchmark: Benchmark) -> Self {
        Self {
            benchmark,
            evaluatees: Vec::new(),
            timeout: Duration::from_secs(1000),
            test_cores: 1,
        }
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout
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
                "result/{}-{}",
                evaluatee.name(),
                Local::now().format("%m-%d-%H-%M")
            );
            let res_file = File::create(Path::new(&result_file)).unwrap();
            let share = Arc::new(Share {
                cases: Mutex::new(self.benchmark.caces().collect()),
                res_file: Mutex::new(res_file),
                timeout: self.timeout,
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

fn command_evaluate(mut command: Command, timeout: Duration) -> EvaluationResult {
    let mut child = command.spawn().unwrap();
    let start = Instant::now();
    if let Ok(Some(status)) = child.wait_timeout(timeout) {
        if status.success() {
            EvaluationResult::Success(start.elapsed())
        } else {
            EvaluationResult::Failed
        }
    } else {
        child.kill().unwrap();
        EvaluationResult::Timeout
    }
}

fn main() {
    let path = "/root/MC-Benchmark/hwmcc17/single";
    let suffix = ".aag";

    let benchmark = Benchmark::new(path, suffix);
    let mut evaluation = Evaluation::new(benchmark);
    evaluation.set_timeout(Duration::from_secs(1000));
    evaluation.add_evaluatee(MyIc3);
    evaluation.set_test_cores(16);
    evaluation.evaluate();
}
