use crate::Evaluatee;
use std::process::Command;

pub struct Pic3;

impl Evaluatee for Pic3 {
    fn name(&self) -> String {
        "pic3".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("/root/pic3/target/release/pic3-demo");
        command.arg(path);
        command
    }
}
