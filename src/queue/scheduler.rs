use super::order::{Direction, Order};
use uuid::Uuid;

// Context information for scheduling decisions
#[derive(Debug, Clone)]
pub struct SchedulerContext {
    pub current_floor: u8,
    pub current_direction: Option<Direction>,
    pub elevator_id: Uuid,
}

impl SchedulerContext {
    pub fn new(current_floor: u8, current_direction: Option<Direction>, elevator_id: Uuid) -> Self {
        Self {
            current_floor,
            current_direction,
            elevator_id,
        }
    }
}

// Trait for implementing different scheduling algorithms
pub trait Scheduler {
    // Optimize the order of service for the given orders
    // Returns orders in the optimal sequence for service
    fn schedule(&self, orders: &[Order], context: &SchedulerContext) -> Vec<Order>;

    // Get the name of this scheduler (for debugging/logging)
    fn name(&self) -> &'static str;
}

// Simple FIFO (First In, First Out) scheduler
pub struct FifoScheduler;

impl Scheduler for FifoScheduler {
    fn schedule(&self, orders: &[Order], _context: &SchedulerContext) -> Vec<Order> {
        // Return orders in chronological order (oldest first)
        let mut sorted_orders = orders.to_vec();
        sorted_orders.sort_by_key(|order| order.created_at());
        sorted_orders
    }

    fn name(&self) -> &'static str {
        "FIFO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::init_clock_with_random_id;
    use crate::queue::order::{Call, Direction};

    fn setup_test_clock() {
        let _ = init_clock_with_random_id();
    }

    #[test]
    fn test_fifo_scheduler() {
        setup_test_clock();

        let context = SchedulerContext::new(1, Some(Direction::Up), Uuid::new_v4());
        let scheduler = FifoScheduler;

        // Add orders with some delay to ensure different timestamps
        let call1 = Call::new(5, Direction::Up);
        std::thread::sleep(std::time::Duration::from_millis(1));
        let call2 = Call::new(3, Direction::Down);

        let orders = vec![call1.clone().into(), call2.clone().into()];
        let scheduled = scheduler.schedule(&orders, &context);

        // Should be in chronological order (call1 first)
        assert_eq!(scheduled[0].id(), call1.id);
        assert_eq!(scheduled[1].id(), call2.id);
    }
}
