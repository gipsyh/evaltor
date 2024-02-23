use crate::Evaluatee;
use std::process::Command;

pub struct AbcPdr;

impl Evaluatee for AbcPdr {
    fn name(&self) -> String {
        "abcpdr".to_string()
    }

    fn version(&self) -> String {
        "r0".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let path = format!("read {path}; logic; undc; strash; zero; pdr");
        let mut command = Command::new("../abc/build/abc");
        command.arg("-c").arg(path);
        command
    }
}
