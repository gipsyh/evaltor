use crate::Evaluatee;
use std::process::Command;

pub struct MyIc3;

impl Evaluatee for MyIc3 {
    fn name(&self) -> String {
        "myic3".to_string()
    }

    fn version(&self) -> String {
        "t26".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("../ic3/target/release/ic3");
        command.arg(path);
        command
    }
}
