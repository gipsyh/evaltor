use crate::Evaluatee;
use std::process::Command;

pub struct Cryptominisat;

impl Evaluatee for Cryptominisat {
    fn name(&self) -> String {
        "cryptominisat".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("../cryptominisat-rs/cryptominisat/build/cryptominisat5");
        command.arg("--verb=0");
        command.arg(path);
        command
    }
}
