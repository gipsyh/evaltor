use crate::EvaluateeIF;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub struct Pdr;

impl EvaluateeIF for Pdr {
    fn name(&self) -> String {
        "abcpdr".to_string()
    }

    fn version(&self) -> Option<String> {
        Some("nct".to_string())
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("../abc/build")]
    }

    fn evaluate(&self, model: &Path) -> Command {
        let model = model.to_str().unwrap();
        let model = format!("read {model}; logic; undc; strash; zero; pdr -nct");
        let mut command = Command::new("../abc/build/abc");
        command.arg("-c").arg(model);
        command
    }
}

pub struct SuperProve;

impl EvaluateeIF for SuperProve {
    fn name(&self) -> String {
        "abc".to_string()
    }

    fn version(&self) -> Option<String> {
        Some("superprove".to_string())
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("../super-prove-build")]
    }

    fn evaluate(&self, model: &Path) -> Command {
        let mut command = Command::new("../super-prove-build/build/super_prove/bin/super_prove.sh");
        command.arg(model);
        command
    }

    fn parallelism(&self) -> usize {
        16
    }
}
