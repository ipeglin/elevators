use crossbeam_channel as channel;
use elevator::ElevatorDriver;
use std::thread;

mod config;
mod elevator;

fn main() {
    let config = config::load();

    // hardware
    let (_hw_terminate_tx, hw_terminate_rx) = channel::unbounded::<()>();
    let (_, hw_motor_direction_rx) = channel::unbounded::<u8>();
    let (_, hw_button_light_rx) = channel::unbounded::<(u8, u8, bool)>();
    let (hw_requests_tx, _) = channel::unbounded::<(u8, u8)>();
    let (hw_floor_sensor_tx, _) = channel::unbounded::<u8>();
    let (_, hw_floor_indicator_rx) = channel::unbounded::<u8>();
    let (_, hw_door_light_rx) = channel::unbounded::<bool>();
    let (hw_emergency_halt_tx, _) = channel::unbounded::<bool>();
    let (hw_obstruction_tx, _) = channel::unbounded::<bool>();

    let elevator_driver = ElevatorDriver::new(
        &config.hardware,
        hw_motor_direction_rx,
        hw_button_light_rx,
        hw_requests_tx,
        hw_floor_sensor_tx,
        hw_floor_indicator_rx,
        hw_door_light_rx,
        hw_emergency_halt_tx,
        hw_obstruction_tx,
        hw_terminate_rx,
    );

    let driver_thread = thread::spawn(move || elevator_driver.run());

    // TODO: spawn pass instance to thread and run
}
