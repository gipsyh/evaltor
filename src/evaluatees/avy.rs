use super::{result_analyse, EvaluationResult};
use crate::Evaluatee;
use std::{path::PathBuf, process::Command};

pub struct Kavy;

impl Evaluatee for Kavy {
    fn name(&self) -> String {
        "Avy".to_string()
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("./rIC3-CAV25/")]
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("python3");
        command.arg("./rIC3-CAV25/Pavy/scripts/pavy.py");
        command.args(["-p", "kavy3"]);
        command.arg(path);
        command
    }

    fn result_analyse(&self, code: i64, time: std::time::Duration) -> EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 0 | 1))
    }
}

pub struct Pavy;

impl Evaluatee for Pavy {
    fn name(&self) -> String {
        "Pavy".to_string()
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("./rIC3-CAV25/")]
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("python3");
        command.arg("./rIC3-CAV25/Pavy/scripts/pavy.py");
        command.arg(path);
        command
    }

    fn parallelism(&self) -> usize {
        10
    }

    fn result_analyse(&self, code: i64, time: std::time::Duration) -> EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 0 | 1))
    }
}
