use crate::Evaluatee;
use std::{path::PathBuf, process::Command};

pub struct Pono;

impl Evaluatee for Pono {
    fn name(&self) -> String {
        "pono".to_string()
    }

    fn version(&self) -> String {
        "ic3sa".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("../pono/build/pono");
        command.args(["-e", "ic3sa", "-k", "100000"]);
        command.arg(path);
        command
    }
}
