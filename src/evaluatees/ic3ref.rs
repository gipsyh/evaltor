use crate::Evaluatee;
use std::{path::PathBuf, process::Command};

pub struct Ic3Ref;

impl Evaluatee for Ic3Ref {
    fn name(&self) -> String {
        "ic3ref".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("../IC3ref/build/ic3refmain");
        let file = std::fs::File::open(path).unwrap();
        command.stdin(file);
        // command.arg("-b");
        // command.arg(path);
        command
    }
}

pub struct Ic3RefCaDiCal;

impl Evaluatee for Ic3RefCaDiCal {
    fn name(&self) -> String {
        "ic3ref-cadical".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("/root/fm24/FM2024/IC3ref/build/ic3refmain");
        let file = std::fs::File::open(path).unwrap();
        command.stdin(file);
        // command.arg("-b");
        // command.arg(path);
        command
    }
}

pub struct Ic3RefCryptominisat;

impl Evaluatee for Ic3RefCryptominisat {
    fn name(&self) -> String {
        "ic3ref-cryptominisat".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("/root/fm24/FM2024/IC3ref-cry/build/ic3refmain");
        let file = std::fs::File::open(path).unwrap();
        command.stdin(file);
        // command.arg("-b");
        // command.arg(path);
        command
    }
}
