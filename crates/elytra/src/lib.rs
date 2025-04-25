use xxhash_rust::xxh64::xxh64;

pub mod document;
pub mod key;
pub mod shard;

trait Hashable {
    fn hash64(&self) -> u64;
}

impl<T> Hashable for T
where
    T: AsRef<[u8]>,
{
    fn hash64(&self) -> u64 {
        xxh64(self.as_ref(), 0)
    }
}
