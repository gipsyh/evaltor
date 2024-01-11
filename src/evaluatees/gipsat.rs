use crate::Evaluatee;
use std::process::Command;

pub struct Gipsat;

impl Evaluatee for Gipsat {
    fn name(&self) -> String {
        "gipsat".to_string()
    }

    fn version(&self) -> String {
        "r0".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("../gipsat/target/release/gipsat");
        command.arg(path);
        command
    }
}
