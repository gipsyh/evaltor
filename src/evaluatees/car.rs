use crate::Evaluatee;
use std::{path::PathBuf, process::Command};

pub struct Car;

impl Evaluatee for Car {
    fn name(&self) -> String {
        "car".to_string()
    }

    fn version(&self) -> String {
        "backward".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("../simplecar/simplecar");
        command.arg("-b");
        command.arg(path);
        command.arg("./test");
        command
    }
}
