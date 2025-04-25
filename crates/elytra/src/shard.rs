use std::path::Path;

use sled::Db;

#[derive(Clone)]
pub struct Shard {
    db: Db,
}

impl Shard {
    pub fn open<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let db = sled::Config::default()
            .create_new(false)
            .mode(sled::Mode::HighThroughput)
            .path(path.as_ref())
            .open()?;

        Ok(Self { db })
    }
}
