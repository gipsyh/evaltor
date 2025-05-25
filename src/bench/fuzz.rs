use super::BenchIF;
use std::{
    fs,
    mem::{forget, take},
    path::PathBuf,
    process::Command,
};
use tempfile::NamedTempFile;

pub struct FuzzBench {
    cases: Vec<NamedTempFile>,
}

impl FuzzBench {
    pub fn new(num: usize) -> Self {
        fs::create_dir_all("/tmp/evaltor/fuzz/").unwrap();
        if Command::new("which")
            .arg("aigfuzz")
            .output()
            .map(|output| !output.status.success())
            .unwrap_or(true)
        {
            panic!("Error: aigfuzz binary not found");
        }
        let mut cases = Vec::new();
        for _ in 0..num {
            let fuzz =
                tempfile::NamedTempFile::with_suffix_in(".aig", "/tmp/evaltor/fuzz").unwrap();
            assert!(
                Command::new("aigfuzz")
                    .args(["-2", "-m", "-s", "-S", "-o"])
                    .arg(fuzz.path())
                    .output()
                    .unwrap()
                    .status
                    .success()
            );
            cases.push(fuzz);
        }
        Self { cases }
    }
}

impl BenchIF for FuzzBench {
    fn name(&self) -> String {
        "fuzz".to_string()
    }

    fn cases(&self) -> Vec<PathBuf> {
        self.cases.iter().map(|c| c.path().to_path_buf()).collect()
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("/tmp/evaltor/fuzz")]
    }
}

impl Drop for FuzzBench {
    fn drop(&mut self) {
        let cases = take(&mut self.cases);
        forget(cases);
    }
}
