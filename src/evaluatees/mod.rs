use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

// pub mod abc;
// pub mod avr;
// pub mod avy;
// pub mod ic3ref;
// pub mod pono;
// pub mod ric3;

#[derive(Debug, Clone)]
pub enum EvaluationResult {
    Success(String, Duration),
    Timeout,
    Failed,
    CertifyFailed,
}

pub trait EvaluateeIF: Send + Sync {
    fn name(&self) -> String;

    fn version(&self) -> String {
        "".to_string()
    }

    fn evaluate(&self, model: &Path) -> Command;

    fn evaluate_with_certify(&self, _model: &Path, _certificate: &Path) -> Command {
        panic!("evaluation with certify is not supported")
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![]
    }

    fn result_analyse(&self, code: i64, time: Duration) -> EvaluationResult {
        match code {
            0 => EvaluationResult::Success("Success".to_string(), time),
            _ => EvaluationResult::Failed,
        }
    }

    fn parallelism(&self) -> usize {
        1
    }

    fn certify(&self, _model: &Path, _certificate: &Path) -> bool {
        true
    }
}

pub struct Evaluatee {
    pub name: String,
    pub version: String,
    pub cmd: PathBuf,
    pub args: Vec<String>,
    pub exit_code: HashMap<i64, String>,
}

impl Evaluatee {
    pub fn new(name: &str, cmd: &Path, args: &[String]) -> Self {
        Self {
            name: name.to_string(),
            version: "".to_string(),
            cmd: cmd.to_path_buf(),
            args: args.to_vec(),
            exit_code: Default::default(),
        }
    }
}

impl EvaluateeIF for Evaluatee {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn version(&self) -> String {
        self.version.to_string()
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![self.cmd.clone()]
    }

    fn evaluate(&self, model: &Path) -> Command {
        let mut cmd = Command::new(&self.cmd);
        let args: Vec<String> = self
            .args
            .iter()
            .map(|arg| {
                if arg == "${case}" {
                    model.to_str().unwrap().to_string()
                } else {
                    arg.clone()
                }
            })
            .collect();
        cmd.args(args);
        cmd
    }

    fn result_analyse(&self, code: i64, time: Duration) -> EvaluationResult {
        if let Some(res) = self.exit_code.get(&code) {
            EvaluationResult::Success(res.clone(), time)
        } else if code == 0 {
            EvaluationResult::Success("Success".to_string(), time)
        } else {
            EvaluationResult::Failed
        }
    }
}
