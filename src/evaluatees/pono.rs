use crate::{Evaluatee, EvaluationResult};
use std::{path::PathBuf, process::Command, time::Duration};
pub struct IC3sa;

fn result_analyse(code: i64, time: Duration) -> EvaluationResult {
    match code {
        0 | 1 => EvaluationResult::Success(time),
        _ => EvaluationResult::Failed,
    }
}

impl Evaluatee for IC3sa {
    fn name(&self) -> String {
        "pono-sa".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("pono");
        command.args(["-e", "ic3sa", "-k", "100000000", "--static-coi"]);
        command.arg(path);
        command
    }
}

pub struct IC3ia;

impl Evaluatee for IC3ia {
    fn name(&self) -> String {
        "pono-ic3ia".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("pono");
        command.args(["-e", "ic3ia", "-k", "100000000", "--pseudo-init-prop"]);
        command.arg(path);
        command
    }

    fn result_analyse(&self, code: i64, time: Duration) -> EvaluationResult {
        result_analyse(code, time)
    }
}

pub struct IC3bits;

impl Evaluatee for IC3bits {
    fn name(&self) -> String {
        "pono-ic3bits".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("pono");
        command.args(["-e", "ic3bits", "-k", "100000000"]);
        command.arg(path);
        command
    }

    fn result_analyse(&self, code: i64, time: Duration) -> EvaluationResult {
        result_analyse(code, time)
    }
}
