use crate::EvaluateeIF;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub struct IC3;

impl EvaluateeIF for IC3 {
    fn name(&self) -> String {
        "avr".to_string()
    }

    fn version(&self) -> Option<String> {
        Some("ic3sa".to_string())
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("./avr/")]
    }

    fn evaluate(&self, model: &Path) -> Command {
        let mut command = Command::new("python3");
        command.current_dir("../avr");
        let out = format!("/tmp/evaltor/{}", std::thread::current().id().as_u64());
        command.args([
            "avr.py",
            "--memout",
            "30000",
            "--timeout",
            "10000",
            "-o",
            &out,
        ]);
        command.arg(std::fs::canonicalize(model).unwrap());
        command
    }
}

pub struct Portfolio;

impl EvaluateeIF for Portfolio {
    fn name(&self) -> String {
        "avr".to_string()
    }

    fn version(&self) -> Option<String> {
        Some("portfolio".to_string())
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("../avr")]
    }

    fn evaluate(&self, model: &Path) -> Command {
        let mut command = Command::new("python3");
        command.current_dir("../avr");
        command.args([
            "avr_pr.py",
            "--memout",
            "133120",
            "--timeout",
            "10000",
            "-o",
            "/root/avr_out/",
        ]);
        command.arg(model);
        command
    }

    fn parallelism(&self) -> usize {
        16
    }
}
