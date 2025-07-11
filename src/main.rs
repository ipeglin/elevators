mod clock;

use clock::init_clock;
use crossbeam_channel as channel;
use elevator::ElevatorDriver;
use std::thread;
use uhlc::ID;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load();
    let node_id = ID::try_from([0x01, 0x02, 0x03])?;
    init_clock(node_id)?;

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

    // TODO: spawn pass instance to thread and run
    let driver_thread = thread::Builder::new().name("driver".into());
    driver_thread
        .spawn(move || {
            elevator_driver
                .expect("Failed to initiate ElevatorDriver")
                .run();
        })
        .unwrap();

    loop {
        thread::sleep(std::time::Duration::from_secs(1))
    }
}
