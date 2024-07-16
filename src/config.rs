use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub hardware: HardwareConfig,
}

#[derive(Deserialize)]
pub struct HardwareConfig {
    pub num_floors: u8,
    pub driver_address: String,
    pub driver_port: u32,
    pub driver_channel_poll_timeout_milliseconds: u64,
}

pub fn load() -> Config {
    let config_string = fs::read_to_string("config.toml").expect("Failed to load config file");
    toml::from_str(&config_string).expect("Failed to parse configuration from file")
}
