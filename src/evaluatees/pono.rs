use super::{result_analyse, EvaluationResult};
use crate::Evaluatee;
use std::{path::PathBuf, process::Command, time::Duration};

pub struct IC3sa;

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
        command.args(["-e", "ic3ia", "-k", "100000", "--pseudo-init-prop"]);
        command.arg(path);
        command
    }

    fn result_analyse(&self, code: i64, time: Duration) -> EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 0 | 1))
    }

    fn parallelism(&self) -> usize {
        2
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
        result_analyse(code, time, |c| matches!(c, 0 | 1))
    }
}
