use std::collections::BTreeMap;

use bytes::{BufMut, BytesMut};
use sled::Batch;
use snowflake::{snowflake, Snowflake};

use crate::{key::Key, shard::RecordEncoder, Hashable};

pub struct Document {
    id: Snowflake,
    label: String,
    fields: BTreeMap<String, serde_cbor::Value>,
}

impl RecordEncoder for Document {
    fn encode_into(&self, db: &sled::Db) -> Result<Snowflake, sled::Error> {
        let mut buf = BytesMut::with_capacity(32);
        let mut batch = Batch::default();

        let document_hash = self.label.as_bytes().hash64();

        for (k, v) in &self.fields {
            buf.clear();

            let label = k.as_bytes().hash64();

            buf.put_u64(0x01); // header (only tag type as of now)
            buf.put_u64(document_hash); // record label
            buf.put_u64(label); // field label
            buf.put_u64(self.id.as_u64()); // pk

            batch.insert(
                Key::from(buf.to_vec()),
                serde_cbor::to_vec(&v).expect("failed to serialize field"),
            );
        }

        db.apply_batch(batch).map(|_| self.id)
    }
}
