use crate::Evaluatee;
use std::process::Command;

pub struct Pono;

impl Evaluatee for Pono {
    fn name(&self) -> String {
        "pono".to_string()
    }

    fn version(&self) -> String {
        "ic3sa".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("../pono/build/pono");
        command.args(["-e", "ic3sa", "-k", "1000000000"]);
        command.arg(path);
        command
    }
}
