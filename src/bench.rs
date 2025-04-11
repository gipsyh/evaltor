use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::{
    collections::HashSet,
    fmt::Display,
    fs::read_dir,
    path::{Path, PathBuf},
};

fn search_cases(path: &PathBuf, format: Format) -> Vec<PathBuf> {
    let mut cases = Vec::new();
    for entry in read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension.eq_ignore_ascii_case(&format!("{}", format)) {
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
        .into_iter()
        .map(|c| c.canonicalize().unwrap())
        .collect()
}

#[derive(Clone, Copy, Debug)]
pub enum Format {
    Aig,
    Aag,
    Btor,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Aig => write!(f, "aig"),
            Format::Aag => write!(f, "aag"),
            Format::Btor => write!(f, "btor"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Benchmark {
    name: String,
    path: PathBuf,
    format: Format,
}

impl Benchmark {
    pub fn new(name: &str, path: &str, format: Format) -> Self {
        Self {
            name: name.to_string(),
            path: PathBuf::from(path),
            format,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cases(&self) -> Vec<PathBuf> {
        search_cases(&self.path, self.format)
    }

    pub fn mount(&self) -> PathBuf {
        self.path.clone()
    }
}

#[derive(Default)]
pub struct MultiBenchmark {
    name: Option<String>,
    benchs: Vec<Benchmark>,
    filter: Vec<Box<dyn BenchFilter>>,
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

    pub fn add_filter(mut self, filter: impl BenchFilter + 'static) -> Self {
        self.filter.push(Box::new(filter));
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
        for f in self.filter.iter() {
            res = f.filter(res);
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
            .filter(|f| {
                self.f
                    .contains(&f.file_stem().unwrap().to_str().unwrap().to_string())
            })
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
            .filter(|f| {
                !self
                    .f
                    .contains(&f.file_stem().unwrap().to_str().unwrap().to_string())
            })
            .collect()
    }
}
