#![allow(unused_variables, dead_code)]

use args::ProgramMode;
use config::Configuration;
use processes::{get_process_list, has_process_pid};
use std::io::{BufReader, BufRead};
use std::io;
use std::fs::File;

pub fn run_monitor(_: &ProgramMode, config: &Configuration) -> Result<usize, String> {
    let procs = get_process_list();
    let mut number_of_errors = 0usize;

    for (account_name, account_settings) in &config.accounts {
        let pidfile = match account_settings.settings.get("pidfile") {
            Some(x) =>
                x,
            None =>
                return Err(format!("No 'pidfile' setting in account '{}'", account_name))
        };

        let pid = match get_pid_from_pidfile(pidfile) {
            Ok(x) =>
                x,
            Err(errstr) =>
                return Err(format!("Can't parse pidfile: '{}', error: {}", pidfile, errstr))
        };

        if !has_process_pid(&procs, pid) {
            error!("missing process, pid {}, account {}", pid, account_name);
            number_of_errors += 1;
        }
    }

    Ok(number_of_errors)
}

fn create_reader(file_name: &String) -> io::Result<BufReader<File>> {
    Ok(BufReader::new(File::open(file_name)?))
}

fn do_read_pid(file_name: &String) -> io::Result<String> {
    let mut reader = create_reader(file_name)?;

    let mut line = String::new();
    reader.read_line(&mut line)?;

    Ok(line)
}

fn get_pid_from_pidfile(pidfile: &String) -> Result<i32, String> {
    let pid_str = match do_read_pid(pidfile) {
        Ok(pidstr) =>
            pidstr,
        Err(e) =>
            return Err(format!("I/O error while reading pid from pidfile {}, error description: {}", pidfile, e))
    };

    match pid_str.trim().parse::<i32>() {
        Ok(pid) => Ok(pid),
        Err(_) => Err(format!("Expected number in pidfile: {}, got '{}'", pidfile, pid_str))
    }
}
