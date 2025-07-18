// WIP
use std::time::SystemTime;

pub struct Clock;

impl Clock {
    pub fn new() -> Self {
        Clock
    }

    pub fn sec(self: &Self) -> f64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Could not get system time.")
            .as_secs() as f64
    }

    pub fn milli(self: &Self) -> f64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Could not get system time.")
            .as_millis() as f64 / 1000.0
    }

    pub fn micro(self: &Self) -> f64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Could not get system time.")
            .as_micros() as f64 / 1_000_000.0
    }

    pub fn nano(self: &Self) -> f64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Could not get system time.")
            .as_nanos() as f64 / 1_000_000_000.0
    }
}
