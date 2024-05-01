use crate::config::ProcessesConfig;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tracing::{instrument, trace};

/// Creates a vector of all found target processes. Processes are searched for by process name from `ProcessesConfig`.
#[instrument(skip_all)]
pub fn get_names(config: &ProcessesConfig) -> Vec<String> {
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

    return active_target_processes;
}

/// Returns a tuple with the process text and process icon of the first active process found by `get_names()`.
#[instrument(skip_all)]
pub fn get_data(config: &ProcessesConfig, processes: &Vec<String>) -> (String, String) {
    for target_process in &config.process {
        if processes[0] == target_process.process {
            trace!("Process chosen:\n{target_process:#?}");
            return (
                target_process.text.to_owned(),
                target_process.icon.to_owned(),
            );
        }
    }
    trace!("No active target processes, using idle data");
    return (config.idle_text.to_owned(), config.idle_icon.to_owned());
}
