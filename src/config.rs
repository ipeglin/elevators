use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub hardware: HardwareConfig,
}

#[derive(Deserialize)]
pub struct HardwareConfig {
    NUM_FLOORS: u8,
    DRIVER_ADRESS: String,
    DRIVER_PORT: u32,
    DRIVER_CHANNEL_POLL_TIMEOUT_MILLISECONDS: u32,
}

pub fn load() -> Config {
    let config_string = fs::read_to_string("config.toml").expect("Failed to load config file");
    toml::from_str(&config_string).expect("Failed to parse configuration from file")
}
