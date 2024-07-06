use crate::Evaluatee;
use std::{process::Command, thread};

pub struct IC3;

impl Evaluatee for IC3 {
    fn name(&self) -> String {
        "avr".to_string()
    }

    fn version(&self) -> String {
        "r0".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("python3");
        command.current_dir("/root/avr");
        let out = format!("/tmp/evaluator/{}", thread::current().id().as_u64());
        command.args(["/root/avr/avr.py", "--memout", "16010", "-o", &out, path]);
        command
    }
}
