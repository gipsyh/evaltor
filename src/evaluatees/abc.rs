use crate::Evaluatee;
use std::{path::PathBuf, process::Command};

pub struct Pdr;

impl Evaluatee for Pdr {
    fn name(&self) -> String {
        "abc".to_string()
    }

    fn version(&self) -> Option<String> {
        Some("pdr".to_string())
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("./rIC3-CAV25/")]
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let path = path.as_path().to_str().unwrap();
        let path = format!("read {path}; logic; undc; strash; zero; pdr -nct");
        let mut command = Command::new("./rIC3-CAV25/bin/abc");
        command.arg("-c").arg(path);
        command
    }
}

pub struct SuperProve;

impl Evaluatee for SuperProve {
    fn name(&self) -> String {
        "abc".to_string()
    }

    fn version(&self) -> Option<String> {
        Some("superprove".to_string())
    }

    fn mount(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("./rIC3-CAV25/")]
    }

    fn evaluate(&self, path: &PathBuf) -> Command {
        let conda_prefix = "/usr/local/miniconda3/envs/py2";
        let full_cmd = format!("LD_LIBRARY_PATH={0}/lib:$LD_LIBRARY_PATH PYTHONHOME={0} PYTHONPATH={0}/lib/python2.7/site-packages ./rIC3-CAV25/bin/super_prove/bin/super_prove.sh {1}",
            conda_prefix, path.display()
        );
        let mut command = Command::new("sh");
        command.arg("-c");
        command.arg(full_cmd);
        command
    }

    fn parallelism(&self) -> usize {
        16
    }
}
