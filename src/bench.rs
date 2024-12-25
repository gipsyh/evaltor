use std::fs::read_dir;

fn search_cases(path: &str, format: &str) -> Vec<String> {
    dbg!(path);
    let mut cases = Vec::new();
    for entry in read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension.eq_ignore_ascii_case(&format) {
                    cases.push(path.to_str().unwrap().to_string());
                }
            }
        } else if path.is_dir() {
            let sub_cases = search_cases(path.to_str().unwrap(), format);
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

pub struct Benchmark {
    name: String,
    path: String, 
    format: String,
}

impl Benchmark {
    pub fn new(name: &str, path: &str, format: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            format: format.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cases(&self) -> Vec<String> {
        search_cases(&self.path, &self.format)
    }
}
