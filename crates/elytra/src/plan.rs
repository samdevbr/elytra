use std::fmt::Debug;

use bytes::{BufMut, BytesMut};
use snowflake::Snowflake;

use crate::{key::PartitionKey, shard::Shard, types::Map, util::hash_str};

#[derive(Debug)]
pub enum LogicalPlan {
    UpsertDocument {
        id: Snowflake,
        collection: String,
        input: Map,
    },
}

impl LogicalPlan {
    pub fn execute(self) -> crate::Result<PhysicalPlan> {
        match self {
            LogicalPlan::UpsertDocument {
                id,
                collection,
                input,
            } => {
                let blob = bincode::encode_to_vec(input, bincode::config::standard())?;
                let pk = PartitionKey::new(hash_str(&collection), id);

                let mut buf = BytesMut::with_capacity(17);

                buf.put_u8(0x1);
                buf.put_slice(&pk.as_slice());

                Ok(PhysicalPlan::PutKey(buf.to_vec(), blob))
            }
        }
    }
}

#[derive(Debug)]
pub enum PhysicalPlan {
    PutKey(Vec<u8>, Vec<u8>),
}

impl PhysicalPlan {
    pub fn execute(self, shard: &Shard) -> crate::Result<Option<()>> {
        match self {
            PhysicalPlan::PutKey(k, v) => shard.put(k, v).map(|_| None),
        }
    }
}
