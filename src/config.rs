#![allow(dead_code, unused_variables, unused_attributes, unused_mut)]

//use serde_hjson::{Map, Value};
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::error::Error;
use std::io;
use serde_hjson;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    globals: Option<Vec<Map<String, Value>>>,
}

fn create_reader(file_name: &String) -> io::Result<BufReader<File>> {
    Ok(BufReader::new(File::open(file_name)?))
}

pub fn load_config(file_name: &String) -> Result<Configuration, String> {
    let mut reader = match create_reader(file_name) {
        Ok(x) => x,
        Err(e) => {
            return Err(format!("Config loading failed: {}", e.description()));
        }
    };

    // let data: Configuration = serde_hjson::from_reader(reader).unwrap();

    Err("ok".to_string())
}
