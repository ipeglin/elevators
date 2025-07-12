use serde::Deserialize;
use std::fmt;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub hardware: HardwareConfig,
    pub network: NetworkConfig,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config:\n{}\n{}", self.hardware, self.network)
    }
}

#[derive(Debug, Deserialize)]
pub struct HardwareConfig {
    pub num_floors: u8,
    pub driver_address: String,
    pub driver_port: u32,
    pub driver_channel_poll_timeout_milliseconds: u64,
}

impl fmt::Display for HardwareConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Hardware Config:\n  Floors: {}\n  Driver: {}:{}\n  Poll Timeout: {}ms",
            self.num_floors,
            self.driver_address,
            self.driver_port,
            self.driver_channel_poll_timeout_milliseconds
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    pub address: String,
    pub port: u32,
}

impl fmt::Display for NetworkConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Network Config:\n  Address: {}:{}",
            self.address, self.port
        )
    }
}

pub fn load() -> Config {
    let config_string = fs::read_to_string("config.toml").expect("Failed to load config file");
    toml::from_str(&config_string).expect("Failed to parse configuration from file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_display() {
        let config = Config {
            hardware: HardwareConfig {
                num_floors: 10,
                driver_address: "localhost".to_string(),
                driver_port: 15657,
                driver_channel_poll_timeout_milliseconds: 25,
            },
            network: NetworkConfig {
                address: "192.168.1.100".to_string(),
                port: 8080,
            },
        };

        println!("Debug: {:#?}", config);
        println!("Display:\n{}", config);

        // Test individual components
        println!("Hardware only: {}", config.hardware);
        println!("Network only: {}", config.network);
    }

    #[test]
    fn test_config_debug() {
        let config = Config {
            hardware: HardwareConfig {
                num_floors: 5,
                driver_address: "127.0.0.1".to_string(),
                driver_port: 9999,
                driver_channel_poll_timeout_milliseconds: 50,
            },
            network: NetworkConfig {
                address: "0.0.0.0".to_string(),
                port: 3000,
            },
        };

        // Debug output
        let debug_output = format!("{:?}", config);
        assert!(debug_output.contains("Config"));
        assert!(debug_output.contains("num_floors: 5"));

        // Pretty debug output
        let pretty_debug = format!("{:#?}", config);
        println!("Pretty Debug:\n{}", pretty_debug);
    }
}
