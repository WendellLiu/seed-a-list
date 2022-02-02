use std::env;

use dotenv::dotenv;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TwitterConfig {
    pub token: String,
    pub endpoint: String,
    pub official_account_id: u64,
}

impl TwitterConfig {
    pub fn new() -> TwitterConfig {
        TwitterConfig {
            token: env::var("TWITTER_TOKEN").unwrap(),
            endpoint: env::var("TWITTER_ENDPOINT").unwrap(),
            official_account_id: env::var("TWITTER_OFFICIAL_ACCOUNT_ID")
                .unwrap()
                .parse::<u64>()
                .unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MysqlConfig {
    pub endpoint: String,
}

impl MysqlConfig {
    pub fn new() -> MysqlConfig {
        MysqlConfig {
            endpoint: env::var("DATABASE_URL").unwrap(),
        }
    }
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

    pub fn new() -> SystemConfig {
        dotenv().expect("Failed to read .env file");

        SystemConfig {
            twitter: TwitterConfig::new(),
            mysql: MysqlConfig::new(),
        }
    }
}

pub static SYSTEM_CONFIG: OnceCell<SystemConfig> = OnceCell::new();
