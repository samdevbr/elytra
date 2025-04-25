use std::path::Path;

use sled::Db;

use snowflake::Snowflake;

pub trait RecordEncoder {
    fn encode_into(&self, db: &Db) -> Result<Snowflake, sled::Error>;
}

pub trait RecordDecoder {
    type Record;

    fn decode_from<F>(
        self,
        db: &sled::Db,
        projection: Option<Vec<F>>,
    ) -> Result<Self::Record, sled::Error>
    where
        F: AsRef<str>;
}

#[derive(Clone)]
pub struct Shard {
    db: Db,
}

impl Shard {
    pub fn open<P: AsRef<Path>>(data_dir: P) -> Result<Self, sled::Error> {
        let db = sled::Config::default()
            .create_new(false)
            .mode(sled::Mode::HighThroughput)
            .path(data_dir.as_ref().join("shard0"))
            .open()?;

        Ok(Self { db })
    }

    pub fn put<E: RecordEncoder>(&self, encoder: E) -> Result<Snowflake, sled::Error> {
        encoder.encode_into(&self.db)
    }

    pub fn find<D: RecordDecoder>(
        &self,
        decoder: D,
        projection: Option<Vec<String>>,
    ) -> Result<D::Record, sled::Error> {
        decoder.decode_from(&self.db, projection)
    }
}
