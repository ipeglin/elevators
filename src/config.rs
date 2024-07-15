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
