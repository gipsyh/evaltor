use super::{result_analyse, EvaluationResult};
use crate::Evaluatee;
use std::{
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

pub struct IC3sa;

impl Evaluatee for IC3sa {
    fn name(&self) -> String {
        "pono".to_string()
    }

    fn version(&self) -> Option<String> {
        Some("ic3sa".to_string())
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("../pono")]
    }

    fn evaluate(&self, model: &Path) -> Command {
        let mut command = Command::new("../pono/build/pono");
        command.args(["-e", "ic3sa", "-k", "100000000", "--static-coi"]);
        command.arg(model);
        command
    }

    fn result_analyse(&self, code: i64, time: Duration) -> EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 0 | 1))
    }
}

pub struct IC3ia;

impl Evaluatee for IC3ia {
    fn name(&self) -> String {
        "pono".to_string()
    }

    fn version(&self) -> Option<String> {
        Some("ic3ia".to_string())
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("../pono")]
    }

    fn evaluate(&self, model: &Path) -> Command {
        let mut command = Command::new("../pono/build/pono");
        command.args(["-e", "ic3ia", "-k", "100000000", "--pseudo-init-prop"]);
        command.arg(model);
        command
    }

    fn result_analyse(&self, code: i64, time: Duration) -> EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 0 | 1))
    }
}

pub struct Portfolio;

impl Evaluatee for Portfolio {
    fn name(&self) -> String {
        "pono".to_string()
    }

    fn version(&self) -> Option<String> {
        Some("portfolio".to_string())
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("../pono")]
    }

    fn evaluate(&self, model: &Path) -> Command {
        let mut command = Command::new("python3");
        command.current_dir("../pono");
        command.arg("./scripts/parallel_pono.py");
        command.args(["-k", "1000000"]);
        command.arg(model);
        command
    }

    fn result_analyse(&self, code: i64, time: Duration) -> EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 0 | 1))
    }

    fn parallelism(&self) -> usize {
        16
    }
}
