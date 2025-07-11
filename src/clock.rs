use std::sync::{Arc, OnceLock};
use std::time::Duration;
use uhlc::{HLCBuilder, Timestamp, HLC, ID, NTP64};

pub trait TimestampExt {
    fn add_millis(&self, millis: u64) -> Self;
    fn add_duration(&self, duration: Duration) -> Self;
}

impl TimestampExt for Timestamp {
    fn add_millis(&self, millis: u64) -> Self {
        let duration = Duration::from_millis(millis);
        let ntp64_offset = NTP64::from(duration);
        let new_time = *self.get_time() + ntp64_offset;
        Timestamp::new(new_time, *self.get_id())
    }

    fn add_duration(&self, duration: Duration) -> Self {
        let ntp64_offset = NTP64::from(duration);
        let new_time = *self.get_time() + ntp64_offset;
        Timestamp::new(new_time, *self.get_id())
    }
}

// Global HLC instance
static GLOBAL_HLC: OnceLock<Arc<HLC>> = OnceLock::new();

/// Initialize the global clock with a specific ID
/// This should be called once at application startup
pub fn init_clock(node_id: ID) -> Result<(), &'static str> {
    let hlc = Arc::new(HLCBuilder::new().with_id(node_id).build());

    GLOBAL_HLC.set(hlc).map_err(|_| "Clock already initialized")
}

/// Initialize the global clock with a random ID
/// This should be called once at application startup
pub fn init_clock_with_random_id() -> Result<(), &'static str> {
    let hlc = Arc::new(HLC::default());

    GLOBAL_HLC.set(hlc).map_err(|_| "Clock already initialized")
}

/// Get a reference to the global HLC
/// Panics if clock hasn't been initialized
pub fn get_clock() -> &'static Arc<HLC> {
    GLOBAL_HLC
        .get()
        .expect("Clock not initialized. Call init_clock() first.")
}

/// Generate a new timestamp using the global clock
pub fn current_timestamp() -> Timestamp {
    get_clock().new_timestamp()
}

/// Update the global clock with an external timestamp
pub fn update_clock_with_timestamp(timestamp: &Timestamp) -> Result<(), String> {
    get_clock().update_with_timestamp(timestamp)
}

/// Get the ID of the global clock
pub fn get_clock_id() -> &'static ID {
    get_clock().get_id()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::sync::Arc as StdArc;
    use std::sync::Barrier;
    use std::thread;

    #[test]
    fn test_global_clock_thread_safety() {
        // Initialize clock for test
        let test_id = ID::try_from([0x42]).unwrap();
        init_clock(test_id).unwrap();

        let barrier = StdArc::new(Barrier::new(4));
        let mut handles = vec![];

        // Spawn multiple threads that generate timestamps
        for i in 0..4 {
            let barrier = barrier.clone();
            let handle = thread::spawn(move || {
                barrier.wait(); // Synchronize start

                let mut timestamps = Vec::new();
                for _ in 0..1000 {
                    timestamps.push(current_timestamp());
                }

                // Verify timestamps are monotonic within this thread
                for window in timestamps.windows(2) {
                    assert!(
                        window[1] > window[0],
                        "Timestamps not monotonic in thread {}",
                        i
                    );
                }

                timestamps
            });
            handles.push(handle);
        }

        // Collect all timestamps from all threads
        let mut all_timestamps = Vec::new();
        for handle in handles {
            let timestamps = handle.join().unwrap();
            all_timestamps.extend(timestamps);
        }

        // Verify no duplicate timestamps across all threads
        all_timestamps.sort();
        let original_len = all_timestamps.len();
        all_timestamps.dedup();
        assert_eq!(
            original_len,
            all_timestamps.len(),
            "Found duplicate timestamps across threads"
        );
    }
}
