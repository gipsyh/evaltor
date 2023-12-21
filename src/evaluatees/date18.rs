use crate::Evaluatee;
use std::process::Command;

pub struct CombinedPdr;

impl Evaluatee for CombinedPdr {
    fn name(&self) -> String {
        "date18".to_string()
    }

    fn version(&self) -> String {
        "r0".to_string()
    }

    fn evaluate(&self, path: &str) -> Command {
        let mut command = Command::new("../combinedPDR_DATE18/CombinedPDR_DATE18");
        command.arg(path);
        command
    }
}
