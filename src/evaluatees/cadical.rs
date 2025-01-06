use crate::Evaluatee;
use std::{path::PathBuf, process::Command};

pub struct Cadical;

impl Evaluatee for Cadical {
    fn name(&self) -> String {
        "cadical".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("../cadical/build/cadical");
        command.arg("-n");
        command.arg(path);
        command
    }
}
