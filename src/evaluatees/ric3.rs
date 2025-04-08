use super::{result_analyse, EvaluationResult};
use crate::Evaluatee;
use std::{path::PathBuf, process::Command};

pub struct RIC3;

impl Evaluatee for RIC3 {
    fn name(&self) -> String {
        "rIC3".to_string()
    }

    fn version(&self) -> String {
        "dynamic-dev2".to_string()
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("/root/rIC3")]
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("/root/rIC3/target/release/rIC3");
        command.arg(path);
        command.arg("-e");
        command.arg("ic3");
        command.arg("--ic3-ctg");
        command.arg("--ic3-dynamic");
        command.arg("--ic3-inn");
        // command.arg("--ic3-no-dynamic");
        // command.arg("--ic3-ctg-limit");
        // command.arg("5");
        // command.arg("--certify");
        // command.arg("--ic3-inn");
        command
    }

    fn result_analyse(&self, code: i64, time: std::time::Duration) -> EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 10 | 20))
    }
}

pub struct BMC;

impl Evaluatee for BMC {
    fn name(&self) -> String {
        "rIC3bmc".to_string()
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("/root/rIC3")]
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("/root/rIC3/target/release/rIC3");
        command.args(&["-e", "bmc"]);
        command.arg(path);
        command
    }

    fn result_analyse(&self, code: i64, time: std::time::Duration) -> EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 10 | 20))
    }
}

pub struct Deep;

impl Evaluatee for Deep {
    fn name(&self) -> String {
        "rIC3Deep".to_string()
    }

    fn version(&self) -> String {
        "v0".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("../rIC3/target/release/rIC3");
        command.arg(path);
        command.arg("-e");
        command.arg("deep");
        command
    }
}

pub struct Portfolio;

impl Evaluatee for Portfolio {
    fn name(&self) -> String {
        "rIC3portfolio".to_string()
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("/root/rIC3")]
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("/root/rIC3/target/release/rIC3");
        command.arg("-e");
        command.arg("portfolio");
        // command.arg("--certify");
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
