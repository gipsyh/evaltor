use std::fs::read_dir;

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
            if let Some(file_name) = entry.ok()?.file_name().to_str() {
                if file_name.ends_with(&self.suffix) {
                    return Some(file_name.to_owned());
                }
            }
            None
        })
    }
}

pub trait Evaluatee {
    fn name(&self) -> String;

    fn evaluate(&mut self, path: &str);
}

pub struct Evaluation {
    benchmark: Benchmark,
    evaluatees: Vec<Box<dyn Evaluatee>>,
}

impl Evaluation {
    pub fn new(benchmark: Benchmark) -> Self {
        Self {
            benchmark,
            evaluatees: Vec::new(),
        }
    }

    pub fn add_evaluatee(&mut self, evaluatee: impl Evaluatee + 'static) {
        self.evaluatees.push(Box::new(evaluatee));
    }

    pub fn evaluate(&mut self) {
        for case in self.benchmark.caces() {
            dbg!(case);
        }
    }
}

fn main() {
    let path = "/root/MC-Benchmark/hwmcc17/single";
    let suffix = ".aig";

    let benchmark = Benchmark::new(path, suffix);
    let mut evaluation = Evaluation::new(benchmark);
    evaluation.evaluate();
}
