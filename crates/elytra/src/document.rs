use std::collections::{BTreeMap, BTreeSet};

use bytes::{BufMut, BytesMut};
use sled::Batch;
use snowflake::{snowflake, Snowflake};

use crate::{
    key::Key,
    shard::{RecordDecoder, RecordEncoder},
    Hashable,
};

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

    pub fn new_with_pk<L>(label: L, pk: Snowflake) -> Self
    where
        L: AsRef<str>,
    {
        Self {
            id: pk,
            label: label.as_ref().to_string(),
            fields: BTreeMap::new(),
        }
    }

    pub fn set_field<K, V>(&mut self, k: K, v: V) -> Option<serde_cbor::Value>
    where
        K: AsRef<str>,
        V: Into<serde_cbor::Value>,
    {
        self.fields.insert(k.as_ref().to_string(), v.into())
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
                serde_cbor::to_vec(&(k, v)).expect("failed to serialize field"),
            );
        }

        db.apply_batch(batch).map(|_| self.id)
    }
}

impl RecordDecoder for Document {
    type Record = Self;

    fn decode_from<F>(self, db: &sled::Db, projection: Option<Vec<F>>) -> Result<Self, sled::Error>
    where
        F: AsRef<str>,
    {
        let projection: Option<BTreeSet<u64>> =
            projection.map(|p| p.into_iter().map(|f| f.as_ref().hash64()).collect());

        let (field_lower_bound, field_upper_bound) = match &projection {
            Some(fields) => {
                let lower_field = fields.first().expect("missing lower bound field");
                let upper_field = fields.last().expect("missing upper bound field");

                (*lower_field, *upper_field)
            }
            None => (u64::MIN, u64::MAX),
        };

        let mut buf = BytesMut::with_capacity(32);

        buf.put_u64(0x01); // tag
        buf.put_u64(self.label.hash64()); // document label
        buf.put_u64(field_lower_bound); // field lower bound
        buf.put_u64(self.id.as_u64()); // pk

        let lower_key = Key::from(buf.to_vec());

        buf.clear();

        buf.put_u64(0x01); // tag
        buf.put_u64(self.label.hash64()); // document label
        buf.put_u64(field_upper_bound); // field lower bound
        buf.put_u64(self.id.as_u64()); // pk

        let upper_key = Key::from(buf.to_vec());

        let mut fields = BTreeMap::new();

        for op in db.range(lower_key..=upper_key) {
            let (k, v) = op?;
            let (k, v) = (Key::from(k), v.as_ref());

            if let Some(projection) = &projection {
                let mut hash = [0u8; 8];
                hash.copy_from_slice(&k[16..24]);

                let hash = u64::from_be_bytes(hash);

                if !projection.contains(&hash) {
                    continue;
                }
            }

            let (label, value): (String, serde_cbor::Value) =
                serde_cbor::from_slice(v).expect("failed to deserialize column");

            fields.insert(label, value);
        }

        Ok(Document {
            id: self.id,
            label: self.label.to_owned(),
            fields,
        })
    }
}
