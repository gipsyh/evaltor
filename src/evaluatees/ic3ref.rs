use crate::Evaluatee;
use std::{fs::File, process::Command};

pub struct Ic3Ref;

impl Evaluatee for Ic3Ref {
    fn name(&self) -> String {
        "ic3ref".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("../IC3ref/build/ic3refmain");
        // let file = File::open(path).unwrap();
        // command.stdin(file);
        command.arg("-b");
        command.arg(path);
        command
    }
}
