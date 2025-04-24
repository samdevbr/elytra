use bytes::{Buf, BufMut, Bytes, BytesMut};
use sled::{Batch, Db};

pub struct Shard {
    db: Db,
}

impl Shard {}
