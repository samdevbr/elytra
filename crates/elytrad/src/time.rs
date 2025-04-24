use std::{
    sync::LazyLock,
    time::{Duration, SystemTime},
};

pub static EPOCH: LazyLock<SystemTime> =
    LazyLock::new(|| SystemTime::UNIX_EPOCH + Duration::from_secs(1_735_689_600));

/// Milliseconds elapsed since 2025-01-01T00:00:00 (UTC)
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(*EPOCH)
        .expect("clock error: time went backwards")
        .as_millis() as u64
}
