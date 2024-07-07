use crate::Evaluatee;
use std::process::Command;

pub struct Camical;

impl Evaluatee for Camical {
    fn name(&self) -> String {
        "camical".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("../camical/src/build/camical/camical");
        command.arg("-n");
        command.arg(path);
        command
    }
}
