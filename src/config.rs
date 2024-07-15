use serde::Deserialize;

#[destruct(Deserialize)]
struct Config {
    hardware: HardwareConfig,
}

#[destruct(Deserialize)]
struct HardwareConfig {
    NUM_FLOORS: u8,
    DRIVER_ADRESS: String,
    DRIVER_PORT: u32,
    DRIVER_CHANNEL_POLL_TIMEOUT_MILLISECONDS: u32,
}

pub fn load() -> Config {
  let config_string = fs::read_to_string("config.toml").expect("Failed to load config file");
  toml::from_string(config_string).expect("Failed to parse configuration from file")
}