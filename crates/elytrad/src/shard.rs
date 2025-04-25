use sled::Db;

use crate::id::Snowflake;

pub trait Encodable {
    fn encode_into(&self, db: &Db) -> Result<Snowflake, sled::Error>;
}

pub struct Shard {
    db: Db,
}

impl Shard {
    pub fn put<R: Encodable>(&self, record: R) -> Result<Snowflake, sled::Error> {
        record.encode_into(&self.db)
    }
}
