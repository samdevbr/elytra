use crate::{
    id::{snowflake, Snowflake},
    shard::Record,
};

pub struct Document {
    id: Snowflake,
}

impl Document {
    pub fn new() -> Self {
        Self {
            id: snowflake(crate::node_id()),
        }
    }
}

impl Record for Document {
    type Value = Vec<u8>;

    fn key(&self) -> crate::shard::Key {
        todo!()
    }

    fn value(&self) -> Self::Value {
        todo!()
    }
}
