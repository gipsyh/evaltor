use crate::{EvaluateeIF, bench::BenchIF, evaluatees::EvaluationResult};
use bollard::{Docker, container, secret::HostConfig};
use bytes::Bytes;
use crossbeam::queue::SegQueue;
use futures::{StreamExt, TryStreamExt};
use indicatif::ProgressBar;
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::time::timeout;

struct RaceShare {
    res_file: File,
    log_file: BufWriter<File>,
    pb: ProgressBar,
}

impl Drop for RaceShare {
    fn drop(&mut self) {
        self.pb.finish();
    }
}

#[derive(Clone)]
pub struct Share {
    race_get: Arc<SegQueue<PathBuf>>,
    race_put: Arc<Mutex<RaceShare>>,
    bench_mount: Vec<PathBuf>,
    timeout: Duration,
    memory_limit: usize,
    certify: bool,
}

impl Share {
    pub fn new(
        bench: &dyn BenchIF,
        file: String,
        timeout: Duration,
        memory_limit: usize,
        certify: bool,
    ) -> Self {
        let cases = SegQueue::new();
        for b in bench.cases() {
            cases.push(b);
        }
        let result_file = format!("{}.txt", file);
        let log_file = format!("{}.log", file);
        let result_file = Path::new(&result_file);
        if let Some(parent) = Path::new(result_file).parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let res_file = File::create(result_file).unwrap();
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
            race_get: Arc::new(cases),
            race_put: Arc::new(Mutex::new(RaceShare {
                res_file,
                log_file,
                pb,
            })),
            bench_mount: bench.mount(),
            timeout,
            memory_limit,
            certify,
        }
    }

    #[inline]
    fn get_case(&self) -> Option<PathBuf> {
        self.race_get.pop()
    }

    fn submit_result(&self, case: &Path, res: EvaluationResult, log: Vec<Bytes>) {
        let out_time = match res {
            EvaluationResult::Success(r, time) => {
                format!("{r}({:.2})", time.as_secs_f32()).to_string()
            }
            EvaluationResult::Timeout => "Timeout".to_string(),
            EvaluationResult::Failed => "Failed".to_string(),
            EvaluationResult::CertifyFailed => "CertifyFailed".to_string(),
        };
        let out = format!("{} {}\n", case.display(), out_time);
        let mut race = self.race_put.lock().unwrap();
        race.res_file.write_all(out.as_bytes()).unwrap();
        race.pb.inc(1);
        if !log.is_empty() {
            for l in log {
                race.log_file.write_all(&l).unwrap();
            }
            race.log_file.flush().unwrap();
        }
    }
}

pub struct Worker {
    evaluatee: Arc<dyn EvaluateeIF>,
    share: Share,
    docker: Docker,
}

impl Worker {
    pub fn new(evaluatee: Arc<dyn EvaluateeIF>, share: Share) -> Self {
        let docker = Docker::connect_with_local_defaults().unwrap();
        Self {
            evaluatee,
            share,
            docker,
        }
    }

    async fn evaluate(
        &self,
        command: Command,
        bind: Vec<PathBuf>,
    ) -> (EvaluationResult, Vec<Bytes>) {
        let mut binds: Vec<String> = self
            .share
            .bench_mount
            .iter()
            .chain(self.evaluatee.mount().iter())
            .map(|m| m.canonicalize().unwrap())
            .map(|b| format!("{}:{}:ro", b.display(), b.display()))
            .collect();
        binds.extend(
            bind.iter()
                .map(|m| m.canonicalize().unwrap())
                .map(|b| format!("{}:{}", b.display(), b.display())),
        );
        let host_config = HostConfig {
            memory: Some(self.share.memory_limit as i64),
            cpu_count: Some(self.evaluatee.parallelism() as i64),
            binds: Some(binds),
            init: Some(true),
            ..Default::default()
        };
        let wdir = std::env::current_dir().unwrap();
        let wdir = command
            .get_current_dir()
            .map(|d| d.canonicalize().unwrap())
            .unwrap_or(wdir);
        let mut cmd = vec![command.get_program().to_str().unwrap()];
        cmd.extend(command.get_args().map(|a| a.to_str().unwrap()));
        let config = container::Config {
            image: Some("evaltor_box:latest"),
            working_dir: Some(wdir.to_str().unwrap()),
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
                Err(bollard::errors::Error::DockerContainerWaitError { error: _, code: c }) => {
                    self.evaluatee.result_analyse(c, start.elapsed())
                }
                _ => EvaluationResult::Failed,
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
        let log = log.into_iter().map(|l| l.into_bytes());
        self.docker
            .remove_container(&create.id, Default::default())
            .await
            .unwrap();
        (res, log.collect())
    }

    pub fn start(self) {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        while let Some(case) = self.share.get_case() {
            if self.share.certify {
                let certificate = tempfile::NamedTempFile::new_in("/tmp/evaltor/").unwrap();
                let certificate_path = certificate.path();
                let command = self
                    .evaluatee
                    .evaluate_with_certify(&case, certificate_path);
                let (mut res, log) =
                    rt.block_on(self.evaluate(command, vec![PathBuf::from(certificate_path)]));
                if let EvaluationResult::Success(..) = res {
                    if !self.evaluatee.certify(case.as_path(), certificate_path) {
                        println!("certify {} failed", case.display());
                        res = EvaluationResult::CertifyFailed;
                    }
                }
                self.share.submit_result(&case, res, log);
            } else {
                let command = self.evaluatee.evaluate(&case);
                let (res, log) = rt.block_on(self.evaluate(command, vec![]));
                self.share.submit_result(&case, res, log);
            }
        }
    }
}
