use crate::Evaluatee;
use std::process::Command;

pub struct IC3;

impl Evaluatee for IC3 {
    fn name(&self) -> String {
        "iimc".to_string()
    }

    fn version(&self) -> String {
        "default".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("../iimc/iimc");
        command.args(["-t", "ic3"]);
        command.arg(path);
        command
    }
}
