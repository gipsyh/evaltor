use crate::{Evaluatee, EvaluationResult};
use indicatif::ProgressBar;
use process_control::{ChildExt, Control};
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

pub struct Share {
    cases: Mutex<Vec<String>>,
    res_file: Mutex<File>,
    log_file: Mutex<File>,
    pub pb: Mutex<ProgressBar>,
    timeout: Duration,
    memory_limit: usize,
}

impl Share {
    pub fn new(cases: Vec<String>, file: String, timeout: Duration, memory_limit: usize) -> Self {
        let result_file = format!("{}.txt", file);
        let log_file = format!("{}.log", file);
        let res_file = Mutex::new(File::create(Path::new(&result_file)).unwrap());
        let log_file = Mutex::new(File::create(Path::new(&log_file)).unwrap());
        let pb = indicatif::ProgressBar::new(cases.len() as _);
        pb.set_style(
            indicatif::ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7} / {len:7}",
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        Self {
            cases: Mutex::new(cases),
            res_file,
            log_file,
            pb: Mutex::new(pb),
            timeout,
            memory_limit,
        }
    }

    fn get_case(&self) -> Option<String> {
        self.cases.lock().unwrap().pop()
    }

    fn submit_result(&self, case: String, res: EvaluationResult) {
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
        self.pb.lock().unwrap().inc(1);
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
