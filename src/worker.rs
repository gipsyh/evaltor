use crate::{Evaluatee, EvaluationResult};
use indicatif::ProgressBar;
use process_control::{ChildExt, Control};
use std::{
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

struct RaceShare {
    cases: Vec<PathBuf>,
    res_file: File,
    log_file: File,
    pb: ProgressBar,
}

pub struct Share {
    race: Mutex<RaceShare>,
    timeout: Duration,
    memory_limit: usize,
}

impl Share {
    pub fn new(cases: Vec<PathBuf>, file: String, timeout: Duration, memory_limit: usize) -> Self {
        let result_file = format!("{}.txt", file);
        let log_file = format!("{}.log", file);
        let res_file = File::create(Path::new(&result_file)).unwrap();
        let log_file = File::create(Path::new(&log_file)).unwrap();
        let pb = indicatif::ProgressBar::new(cases.len() as _);
        pb.set_style(
            indicatif::ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7} / {len:7}",
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        Self {
            race: Mutex::new(RaceShare {
                cases,
                res_file,
                log_file,
                pb,
            }),
            timeout,
            memory_limit,
        }
    }

    fn get_case(&self) -> Option<PathBuf> {
        self.race.lock().unwrap().cases.pop()
    }

    fn submit_result<R: Read, E: Read>(
        &self,
        case: PathBuf,
        res: EvaluationResult,
        mut log: R,
        mut stderr: E,
    ) {
        let mut race = self.race.lock().unwrap();
        let out_time = match res {
            EvaluationResult::Success(time) => format!("{:.2}", time.as_secs_f32()).to_string(),
            EvaluationResult::Timeout => "Timeout".to_string(),
            EvaluationResult::Failed => "Failed".to_string(),
        };
        let out = format!("{} {}\n", case.as_path().to_str().unwrap(), out_time);
        race.res_file.write_all(out.as_bytes()).unwrap();
        race.pb.inc(1);
        let _ = io::copy(&mut log, &mut race.log_file);
        let _ = io::copy(&mut stderr, &mut race.log_file);
    }
}

impl Drop for Share {
    fn drop(&mut self) {
        self.race.lock().unwrap().pb.finish();
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

    fn evaluate(&self, case: PathBuf, mut command: Command) {
        let mut child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let start = Instant::now();
        let output = child
            .controlled()
            .time_limit(self.share.timeout)
            .memory_limit(self.share.memory_limit)
            .wait()
            .unwrap();
        let time = start.elapsed();
        let res = if let Some(status) = output {
            if let Some(0 | 1 | 10 | 20) = status.code() {
                EvaluationResult::Success(time)
            } else {
                EvaluationResult::Failed
            }
        } else {
            let cmd = format!(
                r#"pstree -p {} | grep -oP '\(\K\d+' | sort -u | xargs -n 1 kill -9"#,
                child.id()
            );
            Command::new("sh").args(["-c", &cmd]).output().unwrap();
            // nix::sys::signal::kill(
            //     nix::unistd::Pid::from_raw(child.id() as i32),
            //     nix::sys::signal::Signal::SIGINT,
            // )
            // .unwrap();
            EvaluationResult::Timeout
        };
        self.share.submit_result(
            case,
            res,
            child.stdout.take().unwrap(),
            child.stderr.take().unwrap(),
        );
    }

    pub fn start(self) {
        while let Some(case) = self.share.get_case() {
            let command = self.evaluatee.evaluate(&case);
            self.evaluate(case, command);
        }
    }
}
