use std::{
    sync::OnceLock,
    time::{Duration, SystemTime, SystemTimeError},
};

static EPOCH_OFFSET: OnceLock<Duration> = OnceLock::new();
static CUSTOM_EPOCH: OnceLock<SystemTime> = OnceLock::new();

#[inline]
pub fn now() -> Result<u128, SystemTimeError> {
    let elapsed = SystemTime::now().duration_since(*global_epoch())?;

    Ok(elapsed.as_millis())
}

#[inline]
pub fn unix_now() -> Result<u128, SystemTimeError> {
    let elapsed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

    Ok(elapsed.as_millis())
}

#[inline]
pub fn global_epoch() -> &'static SystemTime {
    CUSTOM_EPOCH.get_or_init(|| SystemTime::UNIX_EPOCH + *global_offset())
}

#[inline]
pub fn global_offset() -> &'static Duration {
    EPOCH_OFFSET.get().expect("global offset not set")
}

pub fn set_global_epoch_offset(offset: Duration) {
    if is_global_epoch_offset_set() {
        return;
    }

    EPOCH_OFFSET.set(offset).ok();
}

#[inline]
pub fn is_global_epoch_offset_set() -> bool {
    EPOCH_OFFSET.get().is_some()
}
