use crate::Evaluatee;
use std::process::Command;

pub struct Avy;

impl Evaluatee for Avy {
    fn name(&self) -> String {
        "avy".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("../extavy/build/avy/src/avy");
        command.arg(path);
        command
    }
}

pub struct Pavy;

impl Evaluatee for Pavy {
    fn name(&self) -> String {
        "pavy".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("python3");
        command.arg("/root/pavy/scripts/pavy.py");
        command.arg(path);
        command
    }

    fn parallelism(&self) -> usize {
        10
    }
}
