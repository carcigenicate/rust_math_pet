use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_as_milli() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub fn calc_days_since(timestamp: u128) -> f64 {
    let current = now_as_milli();
    let elapsed = (current - timestamp) as f64;

    elapsed / 1000.0 / 60.0 / 60.0 / 24.0
}