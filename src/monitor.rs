#![allow(unused_variables, dead_code)]

use args::ProgramMode;
use config::Configuration;
use processes::{get_process_list};

pub fn run_monitor(_: &ProgramMode, config: &Configuration) -> bool {
    trace!("running monitor...");
    let procs = get_process_list();

    for process in &procs {
        println!("proc: {}", process.proc_dir());
    }

    false
}
