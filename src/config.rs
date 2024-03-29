use std::fs::File;
use std::io::BufReader;

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TwitterConfig {
    pub token: String,
    pub endpoint: String,
    pub official_account_id: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MysqlConfig {
    pub endpoint: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemConfig {
    pub twitter: TwitterConfig,
    pub mysql: MysqlConfig,
}

impl SystemConfig {
    pub fn global() -> &'static SystemConfig {
        SYSTEM_CONFIG
            .get()
            .expect("system config is not initialized.")
    }

    pub fn new(path: &str) -> SystemConfig {
        let f = File::open(path).expect("can not read the config file");
        let reader = BufReader::new(f);

        let contents: SystemConfig =
            from_reader(reader).expect("the file doens't not match the type");

        contents
    }
}

pub static SYSTEM_CONFIG: OnceCell<SystemConfig> = OnceCell::new();
