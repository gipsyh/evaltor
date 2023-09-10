use crate::{command_evaluate, Evaluatee, EvaluationResult};
use std::{process::Command, time::Duration};

pub struct MyIc3;

impl Evaluatee for MyIc3 {
    fn name(&self) -> String {
        "myic3".to_string()
    }

    fn evaluate(&self, path: &str, timeout: Duration) -> EvaluationResult {
        let mut command = Command::new("/root/ic3/target/release/ic3");
        command.arg(path);
        command_evaluate(command, timeout)
    }
}
