use crate::{Evaluatee, EvaluationResult};
use std::{path::PathBuf, process::Command, time::Duration};

fn result_analyse(code: i64, time: Duration) -> EvaluationResult {
    match code {
        0 | 1 => EvaluationResult::Success(time),
        _ => EvaluationResult::Failed,
    }
}

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

    fn result_analyse(&self, code: i64, time: std::time::Duration) -> crate::EvaluationResult {
        result_analyse(code, time)
    }
}
