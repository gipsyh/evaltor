use crate::Evaluatee;
use std::{path::PathBuf, process::Command, thread};

pub struct IC3;

impl Evaluatee for IC3 {
    fn name(&self) -> String {
        "avr".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("python3");
        command.current_dir("/root/avr");
        let out = format!("/tmp/evaluator/{}", thread::current().id().as_u64());
        command.args(["/root/avr/avr.py", "--memout", "16010", "-o", &out]);
        command.arg(path);
        command
    }
}
