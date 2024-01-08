use crate::Evaluatee;
use std::process::Command;

pub struct Minisat;

impl Evaluatee for Minisat {
    fn name(&self) -> String {
        "minisat".to_string()
    }

    fn version(&self) -> String {
        "r0".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("../minisat/build/minisat");
        command.arg(path);
        command
    }
}
