use std::{
    sync::LazyLock,
    time::{Duration, SystemTime},
};

/// Epoch offset in seconds
pub const EPOCH_OFFSET: u64 = 1_735_689_600;

/// Local epoch for snowflakes
pub static EPOCH: LazyLock<SystemTime> =
    LazyLock::new(|| SystemTime::UNIX_EPOCH + Duration::from_secs(EPOCH_OFFSET));

/// Milliseconds elapsed since 2025-01-01T00:00:00 (UTC)
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(*EPOCH)
        .expect("clock error: time went backwards")
        .as_millis() as u64
}

/// Seconds elapsed since unix epoch
pub fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("clock error: time went backwards")
        .as_secs()
}
