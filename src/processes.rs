use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tracing::trace;

use crate::config::ProcessesConfig;

pub fn get_processes(config: &ProcessesConfig) -> Vec<String> {
    let sys = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
    );

    let mut active_target_processes: Vec<String> = Vec::new();

    for process in &config.process {
        let mut p = sys.processes_by_exact_name(&process.process);
        if let Some(found_process) = p.next() {
            active_target_processes.push(found_process.name().to_owned());
        }
    }

    trace!("Found target processes \n{active_target_processes:#?}");

    active_target_processes
}

pub fn replace_asset(config: &ProcessesConfig, processes: &Vec<String>) -> (String, String) {
    for target_process in &config.process {
        if processes[0] == target_process.process {
            return (
                target_process.text.to_owned(),
                target_process.icon.to_owned(),
            );
        }
    }
    (config.idle_text.to_owned(), config.idle_icon.to_owned())
}
