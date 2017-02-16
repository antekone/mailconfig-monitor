#![allow(unused_variables, dead_code)]

use args::ProgramMode;
use config::Configuration;
use processes::{get_process_list};

fn ok() -> Result<(), String> { Ok(()) }

pub fn run_monitor(_: &ProgramMode, config: &Configuration) -> Result<(), String> {
    trace!("running monitor...");
    let procs = get_process_list();

    for (account_name, account_settings) in &config.accounts {
        let pidfile = match account_settings.settings.get("pidfile") {
            Some(x) =>
                x,
            None =>
                return Err(format!("No 'pidfile' setting in account '{}'", account_name))
        };

        info!("account {}, checking pidfile: {}", account_name, pidfile);
    }

    ok()
}
