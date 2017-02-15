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

pub struct OptionSet {
    settings: Map<String, String>,
}

pub struct Account {
    settings: Map<String, String>,
}

pub struct Configuration {
    globals: Map<String, String>,
    servers: Map<String, Server>,
    optionsets: Map<String, OptionSet>,
    accounts: Map<String, Account>,
}

impl Configuration {
    fn new() -> Configuration {
        Configuration {
            globals: Map::new(),
            servers: Map::new(),
            optionsets: Map::new(),
            accounts: Map::new(),
        }
    }

    pub fn get_optionset_by_name(&self, name: &str) -> Option<&OptionSet> {
        self.optionsets.get(name)
    }

    pub fn dump(&self) {
        for (key, value) in &self.globals {
            info!("global: key='{}', value='{}'", key, value);
        }

        for (key, obj) in &self.servers {
            for (name, value) in &obj.settings {
                info!("server: name='{}', key='{}', value='{}'", key, name, value);
            }
        }

        for (key, obj) in &self.optionsets {
            for (name, value) in &obj.settings {
                info!("optionset: name='{}', key='{}', value='{}'", key, name, value);
            }
        }

        for (key, obj) in &self.accounts {
            for (name, value) in &obj.settings {
                info!("account: name='{}', key='{}', value='{}'", key, name, value);
            }
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
            iter_over_stringlist(&value, "servers", |server_name, server_map| {
                config.servers.insert(server_name.to_string(), Server { settings: server_map });
            })?;
        } else if key == "option sets" {
            iter_over_stringlist(&value, "option sets", |optionset_name, optionset_map| {
                config.optionsets.insert(optionset_name.to_string(), OptionSet { settings: optionset_map });
            })?;
        } else if key == "accounts" {
            iter_over(&value, "accounts", |account_name, account_obj| {
                let mut settings = Map::new();

                iter_over(&account_obj, account_name, |setting_name, setting_value| {
                    if setting_name == "templates" {
                        match setting_value.as_array() {
                            Some(x) => process_templates(&mut settings, &x, &config)?,
                            None =>    return Err(format!("setting '{}' is not an array", setting_name)),
                        }
                    } else {
                        if let Some(x) = setting_value.as_str() {
                            settings.insert(setting_name.to_string(), x.to_string());
                        } else {
                            return Err(format!("setting '{}' is not a string", setting_name));
                        }
                    }

                    Ok(())
                })?;

                config.accounts.insert(account_name.to_string(), Account { settings: settings });
                Ok(())
            })?;
        }
    }

    Ok(config)
}

fn process_templates(settings: &mut Map<String, String>, template_array: &Vec<Value>, config: &Configuration) -> Result<(), String> {
    for ref value in template_array {
        match value.as_str() {
            Some(template_name) => process_template(template_name, settings, config)?,
            None => return Err("expecting string in template".to_string()),
        };
    }

    Ok(())
}

fn process_template(name: &str, settings: &mut Map<String, String>, config: &Configuration) -> Result<(), String> {
    let optionset = match config.get_optionset_by_name(name) {
        Some(x) => x,
        None => return Err(format!("Referenced optionset '{}', but it hasn't been defined", name)),
    };

    for (key, value) in &optionset.settings {
        settings.insert(key.clone(), value.clone());
    }

    Ok(())
}

fn iter_over_stringlist<F>(dict: &Value, section_name: &str, mut closure: F) -> Result<(), String>
    where F: FnMut(&str, Map<String, String>) {

    iter_over(&dict, section_name, |set_name, set_obj| {
        let mut dest_map = Map::new();

        iter_over(&set_obj, set_name, |arr_key, arr_value| {
            dest_map.insert(arr_key.to_string(), arr_value.to_string());
            Ok(())
        })?;

        closure(set_name, dest_map);
        Ok(())
    })?;

    Ok(())
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
