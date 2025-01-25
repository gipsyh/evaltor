use std::{path::PathBuf, process::Command, time::Duration};

pub mod abc;
pub mod avr;
pub mod avy;
pub mod car;
pub mod cryptominisat;
pub mod ic3ref;
pub mod nuxmv;
pub mod pono;
pub mod ric3;

#[derive(Debug, Clone, Copy)]
pub enum EvaluationResult {
    Success(Duration),
    Timeout,
    Failed,
}

pub trait Evaluatee: Send + Sync {
    fn name(&self) -> String;

    fn version(&self) -> String {
        "v0".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command;

    fn result_analyse(&self, code: i64, time: Duration) -> EvaluationResult {
        match code {
            0 => EvaluationResult::Success(time),
            _ => EvaluationResult::Failed,
        }
    }

    fn parallelism(&self) -> usize {
        1
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
