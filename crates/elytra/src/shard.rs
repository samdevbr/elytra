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

    pub fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, k: K, v: V) -> crate::Result<()> {
        self.db.insert(k, v.as_ref())?;

        Ok(())
    }
}
