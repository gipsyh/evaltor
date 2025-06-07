use super::EvaluationResult;
use crate::EvaluateeIF;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub struct IC3;

impl EvaluateeIF for IC3 {
    fn name(&self) -> String {
        "rIC3".to_string()
    }

    fn version(&self) -> String {
        "rlive".to_string()
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("/root/rIC3")]
    }

    fn evaluate(&self, model: &Path) -> Command {
        let mut command = Command::new("/root/rIC3/target/release/rIC3");
        command.arg("-e");
        command.arg("rlive");
        command.arg(model);
        command
    }

    fn evaluate_with_certify(&self, model: &Path, certificate: &Path) -> Command {
        let mut cmd = self.evaluate(model);
        cmd.arg(certificate);
        cmd
    }

    fn certify(&self, model: &Path, certificate: &Path) -> bool {
        let output = Command::new("docker")
            .args([
                "run",
                "--rm",
                "-v",
                &format!("{}:{}", model.display(), model.display()),
                "-v",
                &format!("{}:{}", certificate.display(), certificate.display()),
                "ghcr.io/gipsyh/certifaiger",
            ])
            .arg(model)
            .arg(certificate)
            .output()
            .unwrap();
        if output.status.success() {
            true
        } else if let Some(1) = output.status.code() {
            false
        } else {
            panic!(
                "certifaiger maybe not avaliable, please build docker image from https://github.com/Froleyks/certifaiger"
            );
        }
    }

    fn result_analyse(&self, code: i64, time: std::time::Duration) -> EvaluationResult {
        match code {
            10 => EvaluationResult::Success("Unsafe".to_string(), time),
            20 => EvaluationResult::Success("Safe".to_string(), time),
            _ => EvaluationResult::Failed,
        }
    }
}
