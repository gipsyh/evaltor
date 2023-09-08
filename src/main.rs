use chrono::Local;
use std::{
    fs::{read_dir, File},
    io::Write,
    path::Path,
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
            let result_file = format!(
                "result/{}-{}",
                evaluatee.name(),
                Local::now().format("%m-%d-%H-%M")
            );
            let mut result = File::create(Path::new(&result_file)).unwrap();
            for case in self.benchmark.caces() {
                let evaluatee = evaluatee.clone();
                let timeout = self.timeout.clone();
                let case_clone = case.clone();
                let join = spawn(move || evaluatee.evaluate(&case_clone, timeout));
                let time = join.join().unwrap();
                let out_time = match time {
                    Some(time) => format!("{:.2}", time.as_secs_f32()).to_string(),
                    None => "None".to_string(),
                };
                let out = format!("{} {}\n", case, out_time);
                result.write_all(out.as_bytes()).unwrap();
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

struct AbcPdrCtp;

impl Evaluatee for AbcPdrCtp {
    fn name(&self) -> String {
        "abc-pdr-ctp".to_string()
    }

    fn evaluate(&self, path: &str, timeout: Duration) -> Option<Duration> {
        let path = format!("read {path}; pdr -s -v");
        let mut child = Command::new("/root/abc/abc").arg("-c").arg(path).spawn().unwrap();
        let start = Instant::now();
        if let Ok(Some(_)) = child.wait_timeout(timeout) {
            Some(start.elapsed())
        } else {
            child.kill().unwrap();
            None
        }
    }
}

struct Pic3;

impl Evaluatee for Pic3 {
    fn name(&self) -> String {
        "pic3".to_string()
    }

    fn evaluate(&self, path: &str, timeout: Duration) -> Option<Duration> {
        let mut child = Command::new("/root/pic3/target/release/pic3-demo")
            .arg(path)
            .spawn()
            .unwrap();
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
    evaluation.set_timeout(Duration::from_secs(300));
    evaluation.add_evaluatee(AbcPdrCtp);
    evaluation.evaluate();
}
