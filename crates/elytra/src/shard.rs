use sled::Db;

use snowflake::Snowflake;

pub trait RecordEncoder {
    fn encode_into(&self, db: &Db) -> Result<Snowflake, sled::Error>;
}

pub struct Shard {
    db: Db,
}

impl Shard {
    pub fn put<E: RecordEncoder>(&self, encoder: E) -> Result<Snowflake, sled::Error> {
        encoder.encode_into(&self.db)
    }
}
