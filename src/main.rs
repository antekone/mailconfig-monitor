extern crate argparse;
extern crate ansi_term;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate serde_hjson;
extern crate hyper;

mod args;
mod alog;
mod monitor;
mod processes;
mod config;

use std::process::exit;
use std::io::{BufRead, BufReader, Read};
use monitor::run_monitor;
use config::{load_config};
use hyper::{Client};
use std::fs::File;

const ARBITER_PING_URL_CONF: &'static str = "/etc/arbiter_ping_url.conf";

fn main() {
    alog::init().unwrap();

    // Make sure this will panic on the beginning of runtime, not at the end.
    early_panic_if_needed();

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
            Ok(number_of_errors) =>
                if number_of_errors > 0 {
                    handle_errors();
                    exit(1);
                } else {
                    handle_no_errors();
                    exit(0);
                },
            Err(string) => {
                error!("Monitor run failed, error: {}", string);
                exit(1);
            }
        }
    }

    error!("No action specified.");
    exit(1);
}

fn get_file_contents(file_name: &str) -> String {
    let file = File::open(file_name).expect("can't open hard settings file");
    let mut reader = BufReader::new(file);

    let mut data = String::new();
    reader.read_to_string(&mut data).expect("can't read hard settings file");

    data.trim().to_string()
}

fn send_arbiter_ping(msg: &str, nextcheck: i32) {
    // Sorry, since code is in github, I don't want to share all of my URLs ;)
    let ping_url = get_file_contents(ARBITER_PING_URL_CONF);

    let http = Client::new();
    let mut res = http.post(ping_url.as_str())
        .body(format!("{{\"machine\": \"mailgate\", \"name\": \"mailconfig-pid-watcher\", \"status\": \"{}\", \"nextcheck\": {}}}", msg, nextcheck).as_str())
        .send()
        .expect("Can't send Arbiter HTTP request");

    let mut body = String::new();
    res.read_to_string(&mut body).expect("Can't read answer of Arbiter HTTP request");
}

fn handle_errors() {
    // send dead heartbeat
    send_arbiter_ping("DEAD", 60);
}

fn handle_no_errors() {
    // send alive heartbeat
    send_arbiter_ping("HEARTBEAT", 60);
}

fn early_panic_if_needed() {
    get_file_contents(ARBITER_PING_URL_CONF);
}
