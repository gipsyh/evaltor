use crate::Evaluatee;
use std::process::Command;

pub struct IC3;

impl Evaluatee for IC3 {
    fn name(&self) -> String {
        "avr".to_string()
    }

    fn version(&self) -> String {
        "r0".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("cd");
        command.args(["../avr", "&&", "python3"]);
        command
    }
}
