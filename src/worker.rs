use crate::{Evaluatee, EvaluationResult};
use process_control::{ChildExt, Control};
use std::{
    fs::File,
    io::{self, Read, Write},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

pub struct Share {
    pub cases: Mutex<Vec<String>>,
    pub res_file: Mutex<File>,
    pub log_file: Mutex<File>,
    pub timeout: Duration,
    pub memory_limit: usize,
}

impl Share {
    fn get_case(&self) -> Option<String> {
        self.cases.lock().unwrap().pop()
    }

    fn submit_result(&self, case: String, res: EvaluationResult) {
        println!("{:?}", res);
        let out_time = match res {
            EvaluationResult::Success(time) => format!("{:.2}", time.as_secs_f32()).to_string(),
            EvaluationResult::Timeout => "Timeout".to_string(),
            EvaluationResult::Failed => "Failed".to_string(),
        };
        let out = format!("{} {}\n", case, out_time);
        self.res_file
            .lock()
            .unwrap()
            .write_all(out.as_bytes())
            .unwrap();
    }

    fn submit_log<R: Read>(&self, mut log: R) {
        let _ = io::copy(&mut log, &mut *self.log_file.lock().unwrap());
    }
}

pub struct Worker {
    evaluatee: Arc<dyn Evaluatee>,
    share: Arc<Share>,
}

impl Worker {
    pub fn new(evaluatee: Arc<dyn Evaluatee>, share: Arc<Share>) -> Self {
        Self { evaluatee, share }
    }

    fn evaluate(&self, case: String, mut command: Command) {
        command.stdout(Stdio::piped());
        let mut child = command.spawn().unwrap();
        let start = Instant::now();
        let output = child
            .controlled()
            .time_limit(self.share.timeout)
            .memory_limit(self.share.memory_limit)
            .wait()
            .unwrap();
        let time = start.elapsed();
        let res = if let Some(status) = output {
            if status.success() {
                EvaluationResult::Success(time)
            } else {
                EvaluationResult::Failed
            }
        } else {
            nix::sys::signal::kill(
                nix::unistd::Pid::from_raw(child.id() as i32),
                nix::sys::signal::Signal::SIGINT,
            )
            .unwrap();
            EvaluationResult::Timeout
        };
        self.share.submit_log(child.stdout.take().unwrap());
        self.share.submit_result(case, res);
    }

    pub fn start(self) {
        while let Some(case) = self.share.get_case() {
            let command = self.evaluatee.evaluate(&case);
            self.evaluate(case, command);
        }
    }
}
