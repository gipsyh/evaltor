use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

pub trait BenchFilter {
    fn filter(&self, cases: Vec<PathBuf>) -> Vec<PathBuf>;
}

pub struct BenchIncludeFilter {
    f: HashSet<String>,
}

impl BenchIncludeFilter {
    pub fn new<P: AsRef<Path>>(f: P) -> Self {
        let file = std::fs::File::open(f).unwrap();
        let f: HashSet<String> = serde_json::from_reader(file).unwrap();
        Self { f }
    }
}

impl BenchFilter for BenchIncludeFilter {
    fn filter(&self, cases: Vec<PathBuf>) -> Vec<PathBuf> {
        cases
            .into_iter()
            .filter(|f| self.f.contains(f.file_stem().unwrap().to_str().unwrap()))
            .collect()
    }
}

pub struct BenchExcludeFilter {
    f: HashSet<String>,
}

impl BenchExcludeFilter {
    pub fn new<P: AsRef<Path>>(f: P) -> Self {
        let file = std::fs::File::open(f).unwrap();
        let f: HashSet<String> = serde_json::from_reader(file).unwrap();
        Self { f }
    }
}

impl BenchFilter for BenchExcludeFilter {
    fn filter(&self, cases: Vec<PathBuf>) -> Vec<PathBuf> {
        cases
            .into_iter()
            .filter(|f| !self.f.contains(f.file_stem().unwrap().to_str().unwrap()))
            .collect()
    }
}
