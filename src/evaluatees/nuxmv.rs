use std::process::Command;

use crate::Evaluatee;

pub struct IC3;

impl Evaluatee for IC3 {
    fn name(&self) -> String {
        "nuXmv".to_string()
    }

    fn version(&self) -> String {
        "r0".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("bash");
        command.arg("../nuXmv/ic3.sh");
        command.arg(path);
        command
    }
}
