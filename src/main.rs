extern crate argparse;
extern crate ansi_term;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate serde_hjson;

mod args;
mod alog;
mod monitor;
mod processes;
mod config;

use std::process::exit;
use monitor::run_monitor;
use config::{load_config};

fn main() {
    alog::init().unwrap();

    let mut pmode = args::ProgramMode {
        monitor_mode: false,
        config_file: String::new()
    };

    if !args::parse_args(&mut pmode) {
        return;
    }

    if !pmode.monitor_mode {
        error!("No mode selected, nothing to do. Use `-h` option to get some help.");
        return;
    }

    if pmode.config_file.len() == 0 {
        error!("No config file specified, aborting. Use `-c` option to specify the config file.");
        return;
    }

    let config = match load_config(&pmode.config_file) {
        Ok(x) => x,
        Err(e) => {
            error!("Can't parse config file: {}", pmode.config_file);
            error!("Error was: {}", e);
            exit(1);
        }
    };

    if pmode.monitor_mode {
        match run_monitor(&pmode, &config) {
            Ok(_) =>
                exit(0),
            Err(string) => {
                error!("Monitor run failed, error: {}", string);
                exit(1);
            }
        }
    }

    error!("No action specified.");
    exit(1);
}
