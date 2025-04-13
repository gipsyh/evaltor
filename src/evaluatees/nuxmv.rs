use super::result_analyse;
use crate::Evaluatee;
use std::{path::PathBuf, process::Command};

pub struct IGoodLemma;

impl Evaluatee for IGoodLemma {
    fn name(&self) -> String {
        "nuXmv".to_string()
    }

    fn version(&self) -> Option<String> {
        Some("cav23".to_string())
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("./rIC3-CAV25/")]
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let path = path.as_path().to_str().unwrap();
        let args =
            "-a ic3 -s cadical -m 1 -u 4 -I 1 -D 0 -g 1 -X 0 -c 0 -p 1 -d 2 -G 1 -P 1 -A 100 -O 3"
                .split(' ');
        let mut command = Command::new("./rIC3-CAV25/bin/nuXmv-ic3");
        command.args(args);
        command.arg(path);
        command
    }

    fn result_analyse(&self, code: i64, time: std::time::Duration) -> super::EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 0 | 1))
    }
}
