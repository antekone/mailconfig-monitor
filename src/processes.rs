#![allow(dead_code)]

use std::fs;

pub struct Process {
    proc_dir_: String,
    id_: i32,
}

impl Process {
    pub fn proc_dir(&self) -> &String { &self.proc_dir_ }
    pub fn id(&self) -> i32           { self.id_ }
}

pub fn get_process_list() -> Vec<Process> {
    fs::read_dir("/proc").unwrap().filter_map(|path| {
        let file_name = path.unwrap().file_name();
        let file_name_str = file_name.to_str().unwrap();
        if let Ok(n) = file_name_str.parse::<i32>() {
            Some(Process {
                proc_dir_: format!("/proc/{}", file_name_str),
                id_:       n
            })
        } else {
            None
        }
    }).collect()
}

pub fn has_process_pid(v: &Vec<Process>, pid: i32) -> bool {
    v.iter().any(|ref x| x.id() == pid)
}
