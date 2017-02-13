extern crate serde_hjson;
extern crate argparse;
extern crate ansi_term;
#[macro_use] extern crate log;

mod args;
mod alog;
mod monitor;

use std::process::exit;
use monitor::run_monitor;

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

    info!("Using config file: {}", pmode.config_file);

    if pmode.monitor_mode {
        let r = run_monitor(&pmode);
        exit(match r { true => 0, false => 1 });
    }

    error!("No action specified.");
    exit(1);
}
