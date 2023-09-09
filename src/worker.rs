use crate::{Evaluatee, EvaluationResult};
use std::{
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
    time::Duration,
};

pub struct Share {
    pub cases: Mutex<Vec<String>>,
    pub res_file: Mutex<File>,
    pub timeout: Duration,
}

impl Share {
    fn get_case(&self) -> Option<String> {
        self.cases.lock().unwrap().pop()
    }

    fn submit_result(&self, case: String, res: EvaluationResult) {
        dbg!(res);
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
}

pub struct Worker {
    evaluatee: Arc<dyn Evaluatee>,
    share: Arc<Share>,
}

impl Worker {
    pub fn new(evaluatee: Arc<dyn Evaluatee>, share: Arc<Share>) -> Self {
        Self { evaluatee, share }
    }

    pub fn start(self) {
        while let Some(case) = self.share.get_case() {
            let res = self.evaluatee.evaluate(&case, self.share.timeout);
            self.share.submit_result(case, res);
        }
    }
}
