use crate::{command_evaluate, Evaluatee, EvaluationResult};
use std::{process::Command, time::Duration};

pub struct AbcPdr;

impl Evaluatee for AbcPdr {
    fn name(&self) -> String {
        "abc-pdr".to_string()
    }

    fn evaluate(&self, path: &str, timeout: Duration, memory_limit: usize) -> EvaluationResult {
        let path = format!("read {path}; pdr");
        let mut command = Command::new("/root/abc/build/abc");
        command.arg("-c").arg(path);
        command_evaluate(command, timeout, memory_limit)
    }
}
