use super::{result_analyse, EvaluationResult};
use crate::Evaluatee;
use std::{path::PathBuf, process::Command};

pub struct RIC3;

impl Evaluatee for RIC3 {
    fn name(&self) -> String {
        "rIC3".to_string()
    }

    fn version(&self) -> Option<String> {
        Some("ic3".to_string())
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("./rIC3-CAV25/")]
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("./rIC3-CAV25/bin/rIC3");
        command.arg(path);
        command.arg("-e");
        command.arg("ic3");
        command.arg("--ic3-ctg");
        command.arg("--ic3-dynamic");
        command
    }

    fn result_analyse(&self, code: i64, time: std::time::Duration) -> EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 10 | 20))
    }
}

pub struct Portfolio;

impl Evaluatee for Portfolio {
    fn name(&self) -> String {
        "rIC3".to_string()
    }

    fn version(&self) -> Option<String> {
        Some("portfolio".to_string())
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("./rIC3-CAV25/")]
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("./rIC3-CAV25/bin/rIC3");
        command.arg("-e");
        command.arg("portfolio");
        command.arg(path);
        command
    }

    fn parallelism(&self) -> usize {
        16
    }

    fn result_analyse(&self, code: i64, time: std::time::Duration) -> EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 10 | 20))
    }
}
