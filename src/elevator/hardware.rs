use crossbeam_channel as channel;

use driver_rust::elevio::elev::Elevator;
use driver_rust::elevio::elev::{CAB, HALL_DOWN, HALL_UP};

const NUM_CALL_VARIANTS: usize = 3;
const NUM_FLOORS: u8 = 4;

pub struct ElevatorDriver {
    elevator: Elevator,
    current_floor: u8,
    is_halted: bool,
    is_obstructed: bool,
    requests: Vec<Vec<bool>>,
    hw_motor_direction_rx: channel::Receiver<u8>,
    hw_button_light_rx: channel::Receiver<(u8, u8, bool)>,
    hw_request_tx: channel::Sender<(u8, u8)>,
    hw_floor_sensor_tx: channel::Sender<u8>,
    hw_floor_indicator_rx: channel::Receiver<u8>,
    hw_door_light_rx: channel::Receiver<bool>,
    hw_emergency_halt_tx: channel::Sender<bool>,
    hw_obstruction_tx: channel::Sender<bool>,
    terminate_rx: channel::Receiver<()>,
}

impl ElevatorDriver {
    pub fn new(
        hw_motor_direction_rx: channel::Receiver<u8>,
        hw_button_light_rx: channel::Receiver<(u8, u8, bool)>,
        hw_request_tx: channel::Sender<(u8, u8)>,
        hw_floor_sensor_tx: channel::Sender<u8>,
        hw_floor_indicator_rx: channel::Receiver<u8>,
        hw_door_light_rx: channel::Receiver<bool>,
        hw_emergency_halt_tx: channel::Sender<bool>,
        hw_obstruction_tx: channel::Sender<bool>,
        terminate_rx: channel::Receiver<()>,
    ) -> ElevatorDriver {
        ElevatorDriver {
            elevator: Elevator::init("localhost:15657", NUM_FLOORS).unwrap(),
            current_floor: u8::MAX, // because unknown starting position
            is_halted: false,
            is_obstructed: false,
            requests: vec![vec![false; NUM_CALL_VARIANTS]; NUM_FLOORS as usize],
            hw_motor_direction_rx,
            hw_button_light_rx,
            hw_request_tx,
            hw_floor_sensor_tx,
            hw_floor_indicator_rx,
            hw_door_light_rx,
            hw_emergency_halt_tx,
            hw_obstruction_tx,
            terminate_rx,
        }
    }

    pub fn run(mut self) -> ! {
        self.is_obstructed = self.elevator.obstruction();

        // reset light on init
        for floor in 0..self.elevator.num_floors {
            self.elevator.call_button_light(floor, HALL_UP, false);
            self.elevator.call_button_light(floor, HALL_DOWN, false);
            self.elevator.call_button_light(floor, CAB, false);
        }

        loop {
            if self.elevator.stop_button() != self.is_halted {
                self.is_halted = !self.is_halted;
                let _ = self.hw_emergency_halt_tx.send(self.is_halted);
            }

            if self.elevator.obstruction() != self.is_obstructed {
                self.is_obstructed = !self.is_obstructed;
                let _ = self.hw_obstruction_tx.send(self.is_obstructed);
            }

            // ref. The Rust Programming Language - Concise Control Flow
            if let Some(floor) = self.elevator.floor_sensor() {
                self.current_floor = floor;
                let _ = self.hw_floor_sensor_tx.send(floor);
            }

            for floor in 0..self.elevator.num_floors {
                let new_cabin_order = !self.requests[floor as usize][CAB as usize]
                    && self.elevator.call_button(floor, CAB);
                if new_cabin_order {
                    self.requests[floor as usize][CAB as usize] = true;
                    let _ = self.hw_request_tx.send((floor, CAB));
                }

                let new_call_upward = !self.requests[floor as usize][HALL_UP as usize]
                    && self.elevator.call_button(floor, HALL_UP);
                if new_call_upward {
                    self.requests[floor as usize][HALL_UP as usize] = true;
                    let _ = self.hw_request_tx.send((floor, HALL_UP));
                }

                let new_call_downward = !self.requests[floor as usize][HALL_DOWN as usize]
                    && self.elevator.call_button(floor, HALL_DOWN);
                if new_call_downward {
                    self.requests[floor as usize][HALL_DOWN as usize] = true;
                    let _ = self.hw_request_tx.send((floor, HALL_DOWN));
                }
            }
        }
    }
}