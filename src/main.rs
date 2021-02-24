mod config;

use config::{SystemConfig, SYSTEM_CONFIG};

fn main() {
    println!("Hello, world!");
    let system_config_instance = SystemConfig::new();
    SYSTEM_CONFIG.set(system_config_instance).unwrap();

    let system_config = SystemConfig::global();

    println!("system config: {}", system_config.twitter.token)
}
