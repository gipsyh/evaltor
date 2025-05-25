use std::{
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

pub mod abc;
pub mod avr;
pub mod avy;
pub mod ic3ref;
pub mod pono;
pub mod ric3;

#[derive(Debug, Clone, Copy)]
pub enum EvaluationResult {
    Success(Duration),
    Timeout,
    Failed,
    CertifyFailed,
}

pub trait EvaluateeIF: Send + Sync {
    fn name(&self) -> String;

    fn version(&self) -> Option<String> {
        None
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
            0 => EvaluationResult::Success(time),
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

fn result_analyse<F>(code: i64, time: Duration, success: F) -> EvaluationResult
where
    F: Fn(i64) -> bool,
{
    if success(code) {
        EvaluationResult::Success(time)
    } else {
        EvaluationResult::Failed
    }
}

pub struct Evaluatee {
    pub name: String,
    pub cmd: PathBuf,
    pub args: Vec<String>,
}

impl Evaluatee {
    pub fn new(name: &str, cmd: &PathBuf, args: &[String]) -> Self {
        Self {
            name: name.to_string(),
            cmd: cmd.clone(),
            args: args.to_vec(),
        }
    }
}

impl EvaluateeIF for Evaluatee {
    fn name(&self) -> String {
        self.name.clone()
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
}
