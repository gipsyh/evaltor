use crate::Evaluatee;
use std::process::Command;

pub struct RIC3;

impl Evaluatee for RIC3 {
    fn name(&self) -> String {
        "rIC3".to_string()
    }

    fn version(&self) -> String {
        "t1".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("../rIC3/target/release/rIC3");
        command.arg(path);
        command.arg("--ic3");
        command
    }
}

pub struct BMC;

impl Evaluatee for BMC {
    fn name(&self) -> String {
        "rIC3bmc".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("../rIC3/target/release/rIC3");
        command.args(&["--bmc", "--kissat", "--step", "70"]);
        command.arg(path);
        command
    }
}

pub struct Kind;

impl Evaluatee for Kind {
    fn name(&self) -> String {
        "rIC3kind".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("../rIC3/target/release/rIC3");
        command.arg("--kind");
        command.arg("-v");
        command.arg(path);
        command
    }
}

pub struct Portfolio;

impl Evaluatee for Portfolio {
    fn name(&self) -> String {
        "rIC3portfolio".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("../rIC3/target/release/rIC3");
        command.arg("--portfolio");
        command.arg(path);
        command
    }

    fn parallelism(&self) -> usize {
        8
    }
}
