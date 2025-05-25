use crate::EvaluateeIF;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub struct Ic3Ref;

impl EvaluateeIF for Ic3Ref {
    fn name(&self) -> String {
        "IC3ref".to_string()
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("./IC3ref/")]
    }

    fn evaluate(&self, model: &Path) -> Command {
        let mut command = Command::new("./IC3ref/build/ic3refmain");
        command.arg(model);
        command
    }
}
