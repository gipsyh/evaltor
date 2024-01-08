use crate::Evaluatee;
use std::process::Command;

pub struct EasySat;

impl Evaluatee for EasySat {
    fn name(&self) -> String {
        "easysat".to_string()
    }

    fn version(&self) -> String {
        "r0".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("../EasySAT/EasySAT");
        command.arg(path);
        command
    }
}
