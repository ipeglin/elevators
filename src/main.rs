use crossbeam_channel as channel;
use elevator::ElevatorDriver;

mod elevator;

fn main() {
    // hardware
    let (_hw_terminate_tx, hw_terminate_rx) = channel::unbounded::<()>();
    let (hw_motor_direction_tx, hw_motor_direction_rx) = channel::unbounded::<u8>();
    let (hw_button_light_tx, hw_button_light_rx) = channel::unbounded::<(u8, u8, bool)>();
    let (hw_requests_tx, hw_requests_rx) = channel::unbounded::<(u8, u8)>();
    let (hw_floor_sensor_tx, hw_floor_sensor_rx) = channel::unbounded::<u8>();
    let (hw_floor_indicator_tx, hw_floor_indicator_rx) = channel::unbounded::<u8>();
    let (hw_door_light_tx, hw_door_light_rx) = channel::unbounded::<bool>();
    let (hw_emergency_halt_tx, hw_emergency_halt_rx) = channel::unbounded::<bool>();
    let (hw_obstruction_tx, hw_obstruction_rx) = channel::unbounded::<bool>();

    let elevator_driver = ElevatorDriver::new(
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
}
