use crate::Evaluatee;
use std::{path::PathBuf, process::Command};

pub struct Minisat;

impl Evaluatee for Minisat {
    fn name(&self) -> String {
        "minisat".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("../minisat/build/minisat");
        command.arg(path);
        command
    }
}
