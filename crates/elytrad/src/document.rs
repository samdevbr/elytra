use std::collections::BTreeMap;

use bytes::{BufMut, BytesMut};
use sled::Batch;
use snowflake::{snowflake, Snowflake};

use crate::{key::Key, shard::RecordEncoder, Hashable};

#[derive(Debug)]
pub struct Document {
    id: Snowflake,
    label: String,
    fields: BTreeMap<String, serde_cbor::Value>,
}

impl Document {
    pub fn new<L>(label: L) -> Self
    where
        L: AsRef<str>,
    {
        Self {
            id: snowflake(),
            fields: BTreeMap::new(),
            label: label.as_ref().to_string(),
        }
    }

    pub fn set_field<V>(&mut self, k: String, v: V) -> Option<serde_cbor::Value>
    where
        V: Into<serde_cbor::Value>,
    {
        self.fields.insert(k, v.into())
    }
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
