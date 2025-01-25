use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::{collections::HashSet, fs::read_dir, path::PathBuf};

fn search_cases(path: &PathBuf, format: &str) -> Vec<PathBuf> {
    let mut cases = Vec::new();
    for entry in read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension.eq_ignore_ascii_case(&format) {
                    cases.push(path);
                }
            }
        } else if path.is_dir() {
            let sub_cases = search_cases(&path, format);
            cases.extend(sub_cases);
        }
    }
    cases.sort();
    cases
}

pub enum Format {
    Aig,
    Aag,
    Btor,
}

#[derive(Clone, Debug)]
pub struct Benchmark {
    name: String,
    path: PathBuf,
    format: String,
}

impl Benchmark {
    pub fn new(name: &str, path: &str, format: &str) -> Self {
        Self {
            name: name.to_string(),
            path: PathBuf::from(path),
            format: format.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cases(&self) -> Vec<PathBuf> {
        search_cases(&self.path, &self.format)
    }

    pub fn mount(&self) -> PathBuf {
        self.path.clone()
    }
}

#[derive(Default, Clone)]
pub struct MultiBenchmark {
    name: Option<String>,
    benchs: Vec<Benchmark>,
}

impl MultiBenchmark {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(&self) -> &str {
        if let Some(n) = &self.name {
            &n
        } else {
            &self.benchs[0].name
        }
    }

    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn add(mut self, b: Benchmark) -> Self {
        self.benchs.push(b);
        self
    }

    pub fn cases(&self) -> Vec<PathBuf> {
        let cases: Vec<PathBuf> = self.benchs.iter().map(|b| b.cases()).flatten().collect();
        let mut seen_filenames = HashSet::new();
        let mut res = Vec::new();
        for case in cases {
            let filename = case.file_name().unwrap();
            if seen_filenames.insert(filename.to_owned()) {
                res.push(case);
            }
        }
        let mut rng = StdRng::seed_from_u64(0);
        res.shuffle(&mut rng);
        res
    }

    pub fn mount(&self) -> Vec<PathBuf> {
        let benchs: HashSet<PathBuf> = self
            .benchs
            .iter()
            .map(|b| b.mount().canonicalize().unwrap())
            .collect();
        benchs.into_iter().collect()
    }
}
