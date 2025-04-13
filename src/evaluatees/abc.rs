use crate::Evaluatee;
use std::{path::PathBuf, process::Command};

pub struct Pdr;

impl Evaluatee for Pdr {
    fn name(&self) -> String {
        "abc".to_string()
    }

    fn version(&self) -> Option<String> {
        Some("pdr".to_string())
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("./rIC3-CAV25/")]
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let path = path.as_path().to_str().unwrap();
        let path = format!("read {path}; logic; undc; strash; zero; pdr -nct");
        let mut command = Command::new("./rIC3-CAV25/bin/abc");
        command.arg("-c").arg(path);
        command
    }
}

pub struct SuperProve;

impl Evaluatee for SuperProve {
    fn name(&self) -> String {
        "abc".to_string()
    }

    fn version(&self) -> Option<String> {
        Some("superprove".to_string())
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command =
            Command::new("/root/rIC3-CAV25/super-prove-build/build/super_prove/bin/super_prove.sh");
        command.arg(path);
        command
    }

    fn parallelism(&self) -> usize {
        16
    }
}
