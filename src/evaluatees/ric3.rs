use crate::Evaluatee;
use std::process::Command;

pub struct RIC3;

impl Evaluatee for RIC3 {
    fn name(&self) -> String {
        "rIC3".to_string()
    }

    fn version(&self) -> String {
        "minisat".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("../rIC3/target/release/ic3");
        command.arg(path);
        command
    }
}
