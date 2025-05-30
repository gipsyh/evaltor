pub mod filter;
pub mod fuzz;

use filter::BenchFilter;
use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};
use std::{
    collections::HashSet,
    fs::read_dir,
    path::{Path, PathBuf},
};

fn search_cases(path: &PathBuf, format: &str) -> Vec<PathBuf> {
    let mut cases = Vec::new();
    for entry in read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            if let Some(extension) = path.extension()
                && extension.eq_ignore_ascii_case(format)
            {
                cases.push(path);
            }
        } else if path.is_dir() {
            let sub_cases = search_cases(&path, format);
            cases.extend(sub_cases);
        }
    }
    cases.sort();
    let mut rng = StdRng::seed_from_u64(0);
    cases.shuffle(&mut rng);
    cases
        .into_iter()
        .map(|c| c.canonicalize().unwrap())
        .collect()
}

pub trait BenchIF {
    fn name(&self) -> String;

    fn cases(&self) -> Vec<PathBuf>;

    fn mount(&self) -> Vec<PathBuf>;
}

#[derive(Clone, Debug)]
pub struct Benchmark {
    name: String,
    path: PathBuf,
    format: String,
}

impl Benchmark {
    pub fn new(name: &str, path: &Path, format: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
            format: format.to_string(),
        }
    }
}

impl BenchIF for Benchmark {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn cases(&self) -> Vec<PathBuf> {
        search_cases(&self.path, &self.format)
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![self.path.clone()]
    }
}

#[derive(Default)]
pub struct MultiBenchmark {
    name: Option<String>,
    benchs: Vec<Box<dyn BenchIF>>,
    filter: Vec<Box<dyn BenchFilter>>,
}

impl MultiBenchmark {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn add_filter(mut self, filter: impl BenchFilter + 'static) -> Self {
        self.filter.push(Box::new(filter));
        self
    }

    pub fn add_bench(mut self, b: Box<dyn BenchIF>) -> Self {
        self.benchs.push(b);
        self
    }
}

impl BenchIF for MultiBenchmark {
    fn name(&self) -> String {
        if let Some(n) = &self.name {
            n.clone()
        } else {
            let name: Vec<_> = self.benchs.iter().map(|b| b.name()).collect();
            name.join("_")
        }
    }

    fn cases(&self) -> Vec<PathBuf> {
        let cases: Vec<PathBuf> = self.benchs.iter().flat_map(|b| b.cases()).collect();
        let mut seen_filenames = HashSet::new();
        let mut res = Vec::new();
        for case in cases {
            let filename = case.file_name().unwrap();
            if seen_filenames.insert(filename.to_owned()) {
                res.push(case);
            }
        }
        for f in self.filter.iter() {
            res = f.filter(res);
        }
        let mut rng = StdRng::seed_from_u64(0);
        res.shuffle(&mut rng);
        res
    }

    fn mount(&self) -> Vec<PathBuf> {
        let benchs: HashSet<PathBuf> = self.benchs.iter().flat_map(|b| b.mount()).collect();
        benchs.into_iter().collect()
    }
}
