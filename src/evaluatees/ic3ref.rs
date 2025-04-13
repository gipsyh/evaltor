use crate::Evaluatee;
use std::{path::PathBuf, process::Command};

pub struct Ic3Ref;

impl Evaluatee for Ic3Ref {
    fn name(&self) -> String {
        "IC3ref".to_string()
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("./rIC3-CAV25/")]
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("./rIC3-CAV25/bin/ic3refmain");
        command.arg(path);
        command
    }
}
