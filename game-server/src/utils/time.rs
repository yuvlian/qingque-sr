use std::time::{SystemTime, UNIX_EPOCH};

pub fn cur_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time error")
        .as_millis() as u64
}

pub fn cur_timestamp_for_seed() -> u32 {
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time error")
        .as_secs()
        / 5)
    .try_into()
    .expect("Timestamp integer overflow")
}
