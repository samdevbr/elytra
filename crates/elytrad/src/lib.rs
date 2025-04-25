use std::sync::OnceLock;

use xxhash_rust::xxh64::xxh64;

pub mod document;
pub mod field;
pub mod id;
pub mod key;
pub mod shard;
pub mod time;

static NODE_ID: OnceLock<u8> = OnceLock::new();

pub fn node_id() -> u8 {
    *NODE_ID.get().expect("node id not set")
}

pub fn set_node_id(id: u8) {
    assert!(id < 0x1F, "cannot set node id: value out of bounds");

    NODE_ID.set(id).expect("node id is already set");
}

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
