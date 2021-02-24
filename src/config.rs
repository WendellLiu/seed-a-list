use std::fs::File;
use std::io::BufReader;

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TwitterConfig {
    pub token: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemConfig {
    pub twitter: TwitterConfig,
}

impl SystemConfig {
    pub fn global() -> &'static SystemConfig {
        SYSTEM_CONFIG
            .get()
            .expect("system config is not initialized.")
    }

    pub fn new() -> SystemConfig {
        let f = File::open("./config.yml").expect("can not read the config file");
        let reader = BufReader::new(f);

        let contents: SystemConfig =
            from_reader(reader).expect("the file doens't not match the type");
        contents
    }
}

pub static SYSTEM_CONFIG: OnceCell<SystemConfig> = OnceCell::new();
