use crate::{command_evaluate, Evaluatee, EvaluationResult};
use std::{fs::File, process::Command, time::Duration};

pub struct Ic3Ref;

impl Evaluatee for Ic3Ref {
    fn name(&self) -> String {
        "ic3ref".to_string()
    }

    fn evaluate(&self, path: &str, timeout: Duration) -> EvaluationResult {
        let mut command = Command::new("/root/IC3ref/build/ic3refmain");
        command.arg("-s");
        let file = File::open(path).unwrap();
        command.stdin(file);
        command_evaluate(command, timeout)
    }
}
