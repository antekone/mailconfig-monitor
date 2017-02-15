#![allow(dead_code, unused_variables, unused_attributes, unused_mut, unused_imports)]

use serde_hjson::{Map, Value};
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::error::Error;
use std::io;
use serde_hjson;
use serde_derive;

pub struct Server {
    settings: Map<String, String>,
}

pub struct Configuration {
    globals: Map<String, String>,
    servers: Map<String, Server>,
}

impl Configuration {
    fn new() -> Configuration {
        Configuration {
            globals: Map::new(),
            servers: Map::new(),
        }
    }

    pub fn dump(&self) {
        for (key, value) in &self.globals {
            info!("global: key='{}', value='{}'", key, value);
        }
    }
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

    let mut config = Configuration::new();
    let root_dict: Map<String, Value> = serde_hjson::from_reader(reader).unwrap();
    for (key, value) in root_dict.iter() {
        if key == "global" {
            iter_over(&value, "global", |key, value| {
                match value.as_str() {
                    Some(x) => {
                        config.globals.insert(key.to_string(), x.to_string());
                        Ok(())
                    },

                    None => Err("Entry '{}' in 'globals' is not a string".to_string())
                }
            })?;
        } else if key == "servers" {
            iter_over(&value, "servers", |key, server_obj| {
                iter_over(&server_obj, key, |server_key, server_value| {
                    info!("server_key={}, server_value={}", server_key, server_value.as_str().unwrap());
                    Ok(())
                })?;

                Ok(())
            })?;
        }
    }

    Ok(config)
}

fn iter_over<F>(dict: &Value, section_name: &str, mut create_object: F) -> Result<(), String>
    where F: FnMut(&str, &Value) -> Result<(), String> {

    let dict_obj = match dict.as_object() {
        Some(x) => x,
        None =>    return Err(format!("'{}' is not an object", section_name))
    };

    for (key, value) in dict_obj {
        create_object(key, value)?;
    }

    Ok(())
}
