use crate::{bench::MultiBenchmark, evaluatees::EvaluationResult, Evaluatee};
use bollard::{container, secret::HostConfig, Docker};
use bytes::Bytes;
use futures::{StreamExt, TryStreamExt};
use indicatif::ProgressBar;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::time::timeout;

struct RaceShare {
    cases: Vec<PathBuf>,
    res_file: File,
    log_file: BufWriter<File>,
    pb: ProgressBar,
}

pub struct Share {
    race: Mutex<RaceShare>,
    bench: MultiBenchmark,
    timeout: Duration,
    memory_limit: usize,
}

impl Share {
    pub fn new(
        bench: MultiBenchmark,
        file: String,
        timeout: Duration,
        memory_limit: usize,
    ) -> Self {
        let cases = bench.cases();
        let result_file = format!("{}.txt", file);
        let log_file = format!("{}.log", file);
        let res_file = File::create(Path::new(&result_file)).unwrap();
        let log_file = BufWriter::new(File::create(Path::new(&log_file)).unwrap());
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
            bench,
            timeout,
            memory_limit,
        }
    }

    fn get_case(&self) -> Option<PathBuf> {
        self.race.lock().unwrap().cases.pop()
    }

    fn submit_result(
        &self,
        case: PathBuf,
        res: EvaluationResult,
        log: impl Iterator<Item = Bytes>,
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
        for l in log {
            race.log_file.write_all(&l).unwrap();
        }
        race.log_file.flush().unwrap();
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
    docker: Docker,
}

impl Worker {
    pub fn new(evaluatee: Arc<dyn Evaluatee>, share: Arc<Share>) -> Self {
        let docker = Docker::connect_with_local_defaults().unwrap();
        Self {
            evaluatee,
            share,
            docker,
        }
    }

    async fn evaluate(&self, case: PathBuf, command: Command) {
        let binds = self
            .share
            .bench
            .mount()
            .iter()
            .chain(self.evaluatee.mount().iter())
            .map(|m| m.canonicalize().unwrap())
            .map(|b| format!("{}:{}:ro", b.display(), b.display()))
            .collect();
        let host_config = HostConfig {
            memory: Some(self.share.memory_limit as i64),
            cpu_count: Some(self.evaluatee.parallelism() as i64),
            binds: Some(binds),
            ..Default::default()
        };
        let wdir = command
            .get_current_dir()
            .map(|d| d.as_os_str().to_str().unwrap())
            .unwrap_or("/root");
        let mut cmd = vec![command.get_program().to_str().unwrap()];
        cmd.extend(command.get_args().map(|a| a.to_str().unwrap()));
        let config = container::Config {
            image: Some("evaluator:latest"),
            working_dir: Some(wdir),
            cmd: Some(cmd),
            tty: Some(true),
            stop_signal: Some("SIGINT"),
            host_config: Some(host_config),
            ..Default::default()
        };
        let create = self
            .docker
            .create_container(None::<container::CreateContainerOptions<&str>>, config)
            .await
            .unwrap();
        self.docker
            .start_container(&create.id, None::<container::StartContainerOptions<String>>)
            .await
            .unwrap();
        let wait_options = container::WaitContainerOptions {
            condition: "not-running",
        };
        let mut wait = self.docker.wait_container(&create.id, Some(wait_options));
        let start = Instant::now();
        let res = match timeout(self.share.timeout, wait.next()).await {
            Ok(wait_result) => match wait_result.unwrap() {
                Ok(wait_result) => self
                    .evaluatee
                    .result_analyse(wait_result.status_code, start.elapsed()),
                Err(_) => EvaluationResult::Failed,
            },
            Err(_) => {
                self.docker.stop_container(&create.id, None).await.unwrap();
                EvaluationResult::Timeout
            }
        };
        let options = Some(container::LogsOptions::<String> {
            stdout: true,
            stderr: true,
            ..Default::default()
        });
        let log = self
            .docker
            .logs(&create.id, options)
            .try_collect::<Vec<_>>()
            .await
            .unwrap();
        let log = log.into_iter().into_iter().map(|l| l.into_bytes());
        self.docker
            .remove_container(&create.id, Default::default())
            .await
            .unwrap();
        self.share.submit_result(case, res, log);
    }

    pub fn start(self) {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        while let Some(case) = self.share.get_case() {
            let command = self.evaluatee.evaluate(&case);
            rt.block_on(self.evaluate(case, command));
        }
    }
}
