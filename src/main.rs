use std::{
    fs::read_dir,
    process::Command,
    sync::Arc,
    thread::spawn,
    time::{Duration, Instant},
};

use wait_timeout::ChildExt;

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

pub trait Evaluatee: Send + Sync {
    fn name(&self) -> String;

    fn evaluate(&self, path: &str, timeout: Duration) -> Option<Duration>;
}

pub struct Evaluation {
    benchmark: Benchmark,
    evaluatees: Vec<Arc<dyn Evaluatee>>,
    timeout: Duration,
}

impl Evaluation {
    pub fn new(benchmark: Benchmark) -> Self {
        Self {
            benchmark,
            evaluatees: Vec::new(),
            timeout: Duration::from_secs(1000),
        }
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout
    }

    pub fn add_evaluatee(&mut self, evaluatee: impl Evaluatee + 'static) {
        self.evaluatees.push(Arc::new(evaluatee));
    }

    pub fn evaluate(&mut self) {
        for evaluatee in self.evaluatees.iter() {
            for case in self.benchmark.caces() {
                let evaluatee = evaluatee.clone();
                let timeout = self.timeout.clone();
                let join = spawn(move || evaluatee.evaluate(&case, timeout));
                let time = join.join().unwrap();
                dbg!(time);
            }
        }
    }
}

struct AbcPdr;

impl Evaluatee for AbcPdr {
    fn name(&self) -> String {
        "abc-pdr".to_string()
    }

    fn evaluate(&self, path: &str, timeout: Duration) -> Option<Duration> {
        let path = format!("read {path}; pdr -v");
        let mut child = Command::new("abc").arg("-c").arg(path).spawn().unwrap();
        let start = Instant::now();
        if let Ok(Some(_)) = child.wait_timeout(timeout) {
            Some(start.elapsed())
        } else {
            child.kill().unwrap();
            None
        }
    }
}

fn main() {
    let path = "/root/MC-Benchmark/hwmcc17/single";
    let suffix = ".aig";

    let benchmark = Benchmark::new(path, suffix);
    let mut evaluation = Evaluation::new(benchmark);
    evaluation.add_evaluatee(AbcPdr);
    evaluation.evaluate();
}
