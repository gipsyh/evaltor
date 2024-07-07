use crate::Evaluatee;
use std::process::Command;

pub struct Pdr;

impl Evaluatee for Pdr {
    fn name(&self) -> String {
        "abcpdr".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let path = format!("read {path}; logic; undc; strash; zero; pdr");
        let mut command = Command::new("../abc/build/abc");
        command.arg("-c").arg(path);
        command
    }
}

pub struct BMC;

impl Evaluatee for BMC {
    fn name(&self) -> String {
        "abcbmc".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let path = format!("read {path}; logic; undc; strash; zero; bmc3");
        let mut command = Command::new("/usr/local/bin/abc");
        command.arg("-c").arg(path);
        command
    }
}

pub struct IMC;

impl Evaluatee for IMC {
    fn name(&self) -> String {
        "abcimc".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let path = format!("read {path}; logic; undc; strash; zero; int");
        let mut command = Command::new("/usr/local/bin/abc");
        command.arg("-c").arg(path);
        command
    }
}

pub struct KIND;

impl Evaluatee for KIND {
    fn name(&self) -> String {
        "abckind".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let path = format!("read {path}; logic; undc; strash; zero; ind");
        let mut command = Command::new("/usr/local/bin/abc");
        command.arg("-c").arg(path);
        command
    }
}

pub struct Fraig;

impl Evaluatee for Fraig {
    fn name(&self) -> String {
        "abcfraig".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let name = &path[path.rfind('/').unwrap()..];
        let path = format!("read {path}; fraig; write ./fraig/{name}");
        let mut command = Command::new("/usr/local/bin/abc");
        command.arg("-c").arg(path);
        command
    }
}
