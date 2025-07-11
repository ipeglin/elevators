use std::collections::VecDeque;
use uuid::Uuid;

use super::order::{Call, Command, Expiration, Order};
use super::scheduler::{Scheduler, SchedulerContext};

// Queue for managing elevator orders
#[derive(Debug)]
pub struct OrderQueue {
    orders: VecDeque<Order>,
    max_size: Option<usize>,
}

impl OrderQueue {
    // Create a new order queue
    pub fn new() -> Self {
        Self {
            orders: VecDeque::new(),
            max_size: None,
        }
    }

    // Create a new order queue with maximum size
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            orders: VecDeque::with_capacity(max_size),
            max_size: Some(max_size),
        }
    }

    // Add an order to the queue
    pub fn add_order(&mut self, order: Order) -> Result<(), QueueError> {
        if let Some(max_size) = self.max_size {
            if self.orders.len() >= max_size {
                return Err(QueueError::QueueFull);
            }
        }

        self.orders.push_back(order);
        Ok(())
    }

    // Add a call to the queue
    pub fn add_call(&mut self, call: Call) -> Result<(), QueueError> {
        self.add_order(Order::Call(call))
    }

    // Add a command to the queue
    pub fn add_command(&mut self, command: Command) -> Result<(), QueueError> {
        self.add_order(Order::Command(command))
    }

    // Remove an order by ID
    pub fn remove_order(&mut self, order_id: Uuid) -> Option<Order> {
        if let Some(pos) = self.orders.iter().position(|order| order.id() == order_id) {
            self.orders.remove(pos)
        } else {
            None
        }
    }

    // Get all orders (without removing them)
    pub fn get_orders(&self) -> Vec<Order> {
        self.orders.iter().cloned().collect()
    }

    // Get orders scheduled by the provided scheduler
    pub fn get_scheduled_orders<S: Scheduler>(
        &self,
        scheduler: &S,
        context: &SchedulerContext,
    ) -> Vec<Order> {
        let orders = self.get_orders();
        scheduler.schedule(&orders, context)
    }

    // Remove and return the next order using the provided scheduler
    pub fn take_next_order<S: Scheduler>(
        &mut self,
        scheduler: &S,
        context: &SchedulerContext,
    ) -> Option<Order> {
        if self.orders.is_empty() {
            return None;
        }

        let scheduled = self.get_scheduled_orders(scheduler, context);
        if let Some(next_order) = scheduled.first() {
            self.remove_order(next_order.id())
        } else {
            None
        }
    }

    // Remove expired orders
    pub fn remove_expired_orders(&mut self) -> Vec<Order> {
        let (expired, valid): (VecDeque<_>, VecDeque<_>) =
            self.orders.drain(..).partition(|order| order.is_expired());

        self.orders = valid;
        expired.into_iter().collect()
    }

    // Get the number of orders in the queue
    pub fn len(&self) -> usize {
        self.orders.len()
    }

    // Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    // Get orders filtered by type
    pub fn get_calls(&self) -> Vec<Call> {
        self.orders
            .iter()
            .filter_map(|order| match order {
                Order::Call(call) => Some(call.clone()),
                _ => None,
            })
            .collect()
    }

    pub fn get_commands(&self) -> Vec<Command> {
        self.orders
            .iter()
            .filter_map(|order| match order {
                Order::Command(command) => Some(command.clone()),
                _ => None,
            })
            .collect()
    }

    // Clear all orders
    pub fn clear(&mut self) {
        self.orders.clear();
    }

    // Get orders by floor
    pub fn get_orders_for_floor(&self, floor: u8) -> Vec<Order> {
        self.orders
            .iter()
            .filter(|order| order.target_floor() == floor)
            .cloned()
            .collect()
    }

    // Count orders by type
    pub fn count_calls(&self) -> usize {
        self.orders.iter().filter(|order| order.is_call()).count()
    }

    pub fn count_commands(&self) -> usize {
        self.orders
            .iter()
            .filter(|order| order.is_command())
            .count()
    }

    // Get the oldest order (by creation time)
    pub fn get_oldest_order(&self) -> Option<&Order> {
        self.orders.iter().min_by_key(|order| order.created_at())
    }

    // Get the newest order (by creation time)
    pub fn get_newest_order(&self) -> Option<&Order> {
        self.orders.iter().max_by_key(|order| order.created_at())
    }
}

impl Default for OrderQueue {
    fn default() -> Self {
        Self::new()
    }
}

// Errors that can occur with queue operations
#[derive(Debug, PartialEq, Eq)]
pub enum QueueError {
    QueueFull,
    OrderNotFound,
}

impl std::fmt::Display for QueueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueError::QueueFull => write!(f, "Queue is full"),
            QueueError::OrderNotFound => write!(f, "Order not found"),
        }
    }
}

impl std::error::Error for QueueError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::init_clock_with_random_id;
    use crate::queue::order::{Call, Command, Direction};
    use crate::queue::scheduler::{FifoScheduler, SchedulerContext};

    fn setup_test_clock() {
        let _ = init_clock_with_random_id();
    }

    #[test]
    fn test_queue_basic_operations() {
        setup_test_clock();

        let mut queue = OrderQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);

        let call = Call::new(5, Direction::Up);
        let command = Command::new(3);

        queue.add_call(call.clone()).unwrap();
        queue.add_command(command.clone()).unwrap();

        assert_eq!(queue.len(), 2);
        assert!(!queue.is_empty());

        let orders = queue.get_orders();
        assert_eq!(orders.len(), 2);

        // Remove by ID
        let removed = queue.remove_order(call.id);
        assert!(removed.is_some());
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_queue_filtering() {
        setup_test_clock();

        let mut queue = OrderQueue::new();

        let call1 = Call::new(5, Direction::Up);
        let call2 = Call::new(3, Direction::Down);
        let command1 = Command::new(7);
        let command2 = Command::new(2);

        queue.add_call(call1.clone()).unwrap();
        queue.add_call(call2.clone()).unwrap();
        queue.add_command(command1.clone()).unwrap();
        queue.add_command(command2.clone()).unwrap();

        assert_eq!(queue.count_calls(), 2);
        assert_eq!(queue.count_commands(), 2);

        let calls = queue.get_calls();
        let commands = queue.get_commands();

        assert_eq!(calls.len(), 2);
        assert_eq!(commands.len(), 2);

        // Test floor filtering
        let floor_5_orders = queue.get_orders_for_floor(5);
        assert_eq!(floor_5_orders.len(), 1);
        assert_eq!(floor_5_orders[0].target_floor(), 5);
    }

    #[test]
    fn test_take_next_order() {
        setup_test_clock();

        let mut queue = OrderQueue::new();
        let context = SchedulerContext::new(1, Some(Direction::Up), Uuid::new_v4());
        let scheduler = FifoScheduler;

        let call1 = Call::new(5, Direction::Up);
        let call2 = Call::new(2, Direction::Up);

        queue.add_call(call1.clone()).unwrap();
        queue.add_call(call2.clone()).unwrap();

        // Should take the closest one (floor 2)
        let next = queue.take_next_order(&scheduler, &context);
        assert!(next.is_some());
        assert_eq!(next.unwrap().target_floor(), 2);
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_queue_capacity() {
        setup_test_clock();

        let mut queue = OrderQueue::with_capacity(2);

        let call1 = Call::new(1, Direction::Up);
        let call2 = Call::new(2, Direction::Up);
        let call3 = Call::new(3, Direction::Up);

        assert!(queue.add_call(call1).is_ok());
        assert!(queue.add_call(call2).is_ok());
        assert_eq!(queue.add_call(call3), Err(QueueError::QueueFull));
    }

    #[test]
    fn test_expired_orders_removal() {
        setup_test_clock();

        let mut queue = OrderQueue::new();

        // Add some orders (they should not be expired immediately)
        let call1 = Call::new(5, Direction::Up);
        let call2 = Call::new(3, Direction::Down);

        queue.add_call(call1).unwrap();
        queue.add_call(call2).unwrap();

        assert_eq!(queue.len(), 2);

        // Remove expired orders (should be none immediately after creation)
        let expired = queue.remove_expired_orders();
        assert_eq!(expired.len(), 0);
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn test_oldest_newest_orders() {
        setup_test_clock();

        let mut queue = OrderQueue::new();

        let call1 = Call::new(1, Direction::Up);
        std::thread::sleep(std::time::Duration::from_millis(1));
        let call2 = Call::new(2, Direction::Up);

        queue.add_call(call1.clone()).unwrap();
        queue.add_call(call2.clone()).unwrap();

        let oldest = queue.get_oldest_order().unwrap();
        let newest = queue.get_newest_order().unwrap();

        assert_eq!(oldest.id(), call1.id);
        assert_eq!(newest.id(), call2.id);
        assert!(newest.created_at() > oldest.created_at());
    }
}
