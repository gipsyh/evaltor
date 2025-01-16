use crate::Evaluatee;
use std::{path::PathBuf, process::Command, time::Duration};

use super::{result_analyse, EvaluationResult};

pub struct Avy;

impl Evaluatee for Avy {
    fn name(&self) -> String {
        "avy".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("/root/rIC3-CAV25/Pavy/executables/avy");
        command.arg(path);
        command
    }
}

pub struct Pavy;

impl Evaluatee for Pavy {
    fn name(&self) -> String {
        "pavy".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("python3");
        command.arg("/root/rIC3-CAV25/Pavy/scripts/pavy.py");
        command.arg(path);
        command
    }

    fn parallelism(&self) -> usize {
        16
    }

    fn result_analyse(&self, code: i64, time: std::time::Duration) -> EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 0 | 1))
    }
}
