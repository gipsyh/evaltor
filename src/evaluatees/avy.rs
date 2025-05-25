use super::{result_analyse, EvaluationResult};
use crate::EvaluateeIF;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub struct Kavy;

impl EvaluateeIF for Kavy {
    fn name(&self) -> String {
        "Avy".to_string()
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("../Pavy")]
    }

    fn evaluate(&self, model: &Path) -> Command {
        let mut command = Command::new("python3");
        command.arg("../Pavy/scripts/pavy.py");
        command.args(["-p", "kavy3"]);
        command.arg(model);
        command
    }

    fn result_analyse(&self, code: i64, time: std::time::Duration) -> EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 0 | 1))
    }
}

pub struct Pavy;

impl EvaluateeIF for Pavy {
    fn name(&self) -> String {
        "Pavy".to_string()
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("../Pavy/")]
    }

    fn evaluate(&self, model: &Path) -> Command {
        let mut command = Command::new("python3");
        command.arg("../Pavy/scripts/pavy.py");
        command.arg(model);
        command
    }

    fn parallelism(&self) -> usize {
        16
    }

    fn result_analyse(&self, code: i64, time: std::time::Duration) -> EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 0 | 1))
    }
}
