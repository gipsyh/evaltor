use crate::Evaluatee;
use std::{process::Command, thread};

pub struct IC3;

impl Evaluatee for IC3 {
    fn name(&self) -> String {
        "nuXmv".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let stdin = format!(
            "read_aiger_model -i {path}
            encode_variables
            build_boolean_model
            check_invar_ic3 -d
            quit"
        );
        let file = format!("/tmp/evaluator/{}", thread::current().id().as_u64());
        let mut command = Command::new("sh");
        command
            .arg("-c")
            .arg(format!("echo '{}' > {}", stdin, file));
        command.output().unwrap();
        let mut command = Command::new("nuXmv");
        command.arg("-source");
        command.arg(file);
        command
    }
}
