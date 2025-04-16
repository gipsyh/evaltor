use crate::Evaluatee;
use std::{path::PathBuf, process::Command, thread};

use super::result_analyse;

pub struct IC3;

impl Evaluatee for IC3 {
    fn name(&self) -> String {
        "nuXmv".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let path = path.as_path().to_str().unwrap();
        let stdin = format!(
            "read_aiger_model -i {path}
            encode_variables
            build_boolean_model
            check_invar_ic3 -d -g
            quit"
        );
        let file = format!("/tmp/evaltor/{}", thread::current().id().as_u64());
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

pub struct IGoodLemma;

impl Evaluatee for IGoodLemma {
    fn name(&self) -> String {
        "nuXmv".to_string()
    }
    fn version(&self) -> Option<String> {
        Some("cav23".to_string())
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("../i-Good_Lemmas_MC")]
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let path = path.as_path().to_str().unwrap();
        let args =
            "-a ic3 -s cadical -m 1 -u 4 -I 1 -D 0 -g 1 -X 0 -c 0 -p 1 -d 2 -G 1 -P 1 -A 100 -O 3"
                .split(' ');
        let mut command = Command::new("../i-Good_Lemmas_MC/nuXmv/nuXmv-ic3");
        command.args(args);
        command.arg(path);
        command
    }

    fn result_analyse(&self, code: i64, time: std::time::Duration) -> super::EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 0 | 1))
    }
}
