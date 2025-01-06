use crate::Evaluatee;
use std::{path::PathBuf, process::Command};

pub struct Cryptominisat;

impl Evaluatee for Cryptominisat {
    fn name(&self) -> String {
        "cryptominisat".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("../cryptominisat-rs/cryptominisat/build/cryptominisat5");
        command.arg("--verb=0");
        command.arg(path);
        command
    }
}
