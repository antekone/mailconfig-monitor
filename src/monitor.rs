use args::ProgramMode;
use processes::{get_process_list, has_process_pid};

pub fn run_monitor(_: &ProgramMode) -> bool {
    trace!("running monitor...");
    let procs = get_process_list();

    for process in &procs {
        println!("proc: {}", process.proc_dir());
    }

    false
}
