use crate::clock::{current_timestamp, TimestampExt};
use std::time::Duration;
use uhlc::Timestamp;
use uuid::Uuid;

const ORDER_EXPIRY_SECONDS: u64 = 60;

pub trait Expiration {
    fn is_expired(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub id: Uuid,
    pub target_floor: u8,
    pub direction: Direction,
    pub created_at: Timestamp,
    pub expires_at: Timestamp,
}

impl Call {
    /// Create a new Call with default expiration
    pub fn new(target_floor: u8, direction: Direction) -> Self {
        let created_at = current_timestamp();
        let expires_at = created_at.add_duration(Duration::from_secs(ORDER_EXPIRY_SECONDS));

        Self {
            id: Uuid::new_v4(),
            target_floor,
            direction,
            created_at,
            expires_at,
        }
    }

    /// Create a new Call with custom expiration duration
    pub fn new_with_expiration(
        target_floor: u8,
        direction: Direction,
        expiration_seconds: u64,
    ) -> Self {
        let created_at = current_timestamp();
        let expires_at = created_at.add_duration(Duration::from_secs(expiration_seconds));

        Self {
            id: Uuid::new_v4(),
            target_floor,
            direction,
            created_at,
            expires_at,
        }
    }
}

impl Expiration for Call {
    fn is_expired(&self) -> bool {
        current_timestamp() > self.expires_at
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub id: Uuid,
    pub target_floor: u8,
    pub created_at: Timestamp,
    pub expires_at: Timestamp,
    pub claimed_by: Option<Uuid>, // Elevator node ID
    pub claimed_at: Option<Timestamp>,
}

impl Command {
    /// Create a new Command with default expiration
    pub fn new(target_floor: u8) -> Self {
        let created_at = current_timestamp();
        let expires_at = created_at.add_duration(Duration::from_secs(ORDER_EXPIRY_SECONDS));

        Self {
            id: Uuid::new_v4(),
            target_floor,
            created_at,
            expires_at,
            claimed_by: None,
            claimed_at: None,
        }
    }

    /// Create a new Command with custom expiration duration
    pub fn new_with_expiration(target_floor: u8, expiration_seconds: u64) -> Self {
        let created_at = current_timestamp();
        let expires_at = created_at.add_duration(Duration::from_secs(expiration_seconds));

        Self {
            id: Uuid::new_v4(),
            target_floor,
            created_at,
            expires_at,
            claimed_by: None,
            claimed_at: None,
        }
    }

    /// Claim this command for a specific elevator
    pub fn claim(&mut self, elevator_id: Uuid) {
        self.claimed_by = Some(elevator_id);
        self.claimed_at = Some(current_timestamp());
    }

    /// Release the claim on this command
    pub fn release_claim(&mut self) {
        self.claimed_by = None;
        self.claimed_at = None;
    }

    /// Check if the command is claimed
    pub fn is_claimed(&self) -> bool {
        self.claimed_by.is_some()
    }
}

impl Expiration for Command {
    fn is_expired(&self) -> bool {
        current_timestamp() > self.expires_at
    }
}

// Order enum that contains both Call and Command
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Order {
    Call(Call),
    Command(Command),
}

impl Order {
    /// Get the ID of the order
    pub fn id(&self) -> Uuid {
        match self {
            Order::Call(call) => call.id,
            Order::Command(command) => command.id,
        }
    }

    /// Get the target floor of the order
    pub fn target_floor(&self) -> u8 {
        match self {
            Order::Call(call) => call.target_floor,
            Order::Command(command) => command.target_floor,
        }
    }

    /// Get the creation timestamp of the order
    pub fn created_at(&self) -> Timestamp {
        match self {
            Order::Call(call) => call.created_at,
            Order::Command(command) => command.created_at,
        }
    }

    /// Get the expiration timestamp of the order
    pub fn expires_at(&self) -> Timestamp {
        match self {
            Order::Call(call) => call.expires_at,
            Order::Command(command) => command.expires_at,
        }
    }

    /// Get the direction for calls, None for commands
    pub fn direction(&self) -> Option<Direction> {
        match self {
            Order::Call(call) => Some(call.direction),
            Order::Command(_) => None,
        }
    }
}

impl Expiration for Order {
    fn is_expired(&self) -> bool {
        match self {
            Order::Call(call) => call.is_expired(),
            Order::Command(command) => command.is_expired(),
        }
    }
}

// Convenient From implementations
impl From<Call> for Order {
    fn from(call: Call) -> Self {
        Order::Call(call)
    }
}

impl From<Command> for Order {
    fn from(command: Command) -> Self {
        Order::Command(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::init_clock_with_random_id;

    fn setup_test_clock() {
        // Try to initialize, ignore error if already initialized
        let _ = init_clock_with_random_id();
    }

    #[test]
    fn test_call_creation() {
        setup_test_clock();

        let call = Call::new(5, Direction::Up);
        assert_eq!(call.target_floor, 5);
        assert_eq!(call.direction, Direction::Up);
        assert!(!call.is_expired());
    }

    #[test]
    fn test_command_creation() {
        setup_test_clock();

        let mut command = Command::new(3);
        assert_eq!(command.target_floor, 3);
        assert!(!command.is_claimed());
        assert!(!command.is_expired());

        let elevator_id = Uuid::new_v4();
        command.claim(elevator_id);
        assert!(command.is_claimed());
        assert_eq!(command.claimed_by, Some(elevator_id));
    }

    #[test]
    fn test_order_matching() {
        setup_test_clock();

        let call = Call::new(2, Direction::Down);
        let command = Command::new(4);

        let orders = vec![Order::from(call), Order::from(command)];

        for order in orders {
            match order {
                Order::Call(call) => {
                    println!(
                        "Processing hall call to floor {} going {:?}",
                        call.target_floor, call.direction
                    );
                }
                Order::Command(command) => {
                    println!("Processing cabin call to floor {}", command.target_floor);
                }
            }
        }
    }
}
