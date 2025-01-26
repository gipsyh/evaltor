use crate::Evaluatee;
use std::{path::PathBuf, process::Command};

pub struct IC3;

impl Evaluatee for IC3 {
    fn name(&self) -> String {
        "avr".to_string()
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("python3");
        command.current_dir("/root/rIC3-CAV25/avr");
        let out = format!("/tmp/evaluator/{}", std::thread::current().id().as_u64());
        command.args([
            "/root/rIC3-CAV25/avr/avr.py",
            "--memout",
            "30000",
            "--timeout",
            "10000",
            "-o",
            &out,
        ]);
        command.arg(std::fs::canonicalize(&path).unwrap());
        // let mut command = Command::new("docker");
        // command.args(&[
        //     "run",
        //     "-t",
        //     "-v",
        //     "/root/mc-benchmark:/root/mc-benchmark",
        //     "--memory=16g",
        //     "--rm",
        //     "avr",
        // ]);
        // command.arg(path);
        // command.args(&["--memout", "30000", "--timeout", "10000"]);
        command
    }
}

pub struct Portfolio;

impl Evaluatee for Portfolio {
    fn name(&self) -> String {
        "avr-portfolio".to_string()
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("/root/rIC3-CAV25/avr")]
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("python3");
        command.current_dir("/root/rIC3-CAV25/avr");
        command.args([
            "/root/rIC3-CAV25/avr/avr_pr.py",
            "--memout",
            "133120",
            "--timeout",
            "10000",
            "-o",
            "/root/avr_out/",
        ]);
        command.arg(&path);
        command
    }

    fn parallelism(&self) -> usize {
        16
    }
}
