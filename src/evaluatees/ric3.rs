use super::{result_analyse, EvaluationResult};
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

    fn version(&self) -> Option<String> {
        Some("dynamic".to_string())
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("/root/rIC3")]
    }

    fn evaluate(&self, model: &Path) -> Command {
        let mut command = Command::new("/root/rIC3/target/release/rIC3");
        command.arg("-e");
        command.arg("ic3");
        command.arg("--ic3-ctg");
        command.arg("--ic3-dynamic");
        command.arg("-v0");
        // command.arg("--ic3-inn");
        // command.arg("--ic3-no-dynamic");
        // command.arg("--ic3-ctg-limit");
        // command.arg("5");
        // command.arg("--certify");
        // command.arg("--ic3-inn");
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
        result_analyse(code, time, |c| matches!(c, 10 | 20))
    }
}

pub struct BMC;

impl EvaluateeIF for BMC {
    fn name(&self) -> String {
        "rIC3bmc".to_string()
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("/root/rIC3")]
    }

    fn evaluate(&self, path: &Path) -> Command {
        let mut command = Command::new("/root/rIC3/target/release/rIC3");
        command.args(["-e", "bmc"]);
        command.arg(path);
        command
    }

    fn result_analyse(&self, code: i64, time: std::time::Duration) -> EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 10 | 20))
    }
}

pub struct Portfolio;

impl EvaluateeIF for Portfolio {
    fn name(&self) -> String {
        "rIC3portfolio".to_string()
    }


    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("/root/rIC3")]
    }

    fn evaluate(&self, model: &Path) -> Command {
        let mut command = Command::new("/root/rIC3/target/release/rIC3");
        command.arg("-e");
        command.arg("portfolio");
        command.arg(model);
        command
    }

    fn parallelism(&self) -> usize {
        16
    }

    fn result_analyse(&self, code: i64, time: std::time::Duration) -> EvaluationResult {
        result_analyse(code, time, |c| matches!(c, 10 | 20))
    }
}
