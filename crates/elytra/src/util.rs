use xxhash_rust::xxh64::xxh64;

#[inline]
pub fn hash_str<T: AsRef<str>>(value: T) -> u64 {
    xxh64(value.as_ref().as_bytes(), 0)
}
