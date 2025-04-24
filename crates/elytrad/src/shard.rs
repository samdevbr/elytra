use bytes::{Buf, BufMut, Bytes, BytesMut};
use sled::{Batch, Db};

use crate::id::Snowflake;

pub struct Key {
    header: u64,
    label: u64,
    field: u64,
    pk: u64,
}

impl Key {
    pub fn as_bytes(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(32);

        buf.put_u64(self.header);
        buf.put_u64(self.label);
        buf.put_u64(self.field);
        buf.put_u64(self.pk);

        buf.freeze()
    }

    pub fn from_buf<B: Buf>(buf: &mut B) -> Self {
        assert_eq!(
            buf.remaining(),
            32,
            "not enough bytes for a key: {} needed 32",
            buf.remaining()
        );

        let header = buf.get_u64();
        let label = buf.get_u64();
        let field = buf.get_u64();
        let pk = buf.get_u64();

        Self {
            header,
            label,
            field,
            pk,
        }
    }

    pub fn id(&self) -> Snowflake {
        Snowflake(self.pk)
    }
}

pub trait Record {
    type Value: AsRef<[u8]>;

    fn key(&self) -> Key;

    fn value(&self) -> Self::Value;
}

pub struct Shard {
    db: Db,
}

impl Shard {
    pub fn put<R: Record>(&self, record: R) -> Result<Snowflake, sled::Error> {
        let k = record.key();
        let v = record.value();

        self.db.insert(k.as_bytes(), v.as_ref())?;

        Ok(k.id())
    }

    pub fn batch_put<R: Record>(
        &self,
        items: impl Iterator<Item = R>,
    ) -> Result<Vec<Snowflake>, sled::Error> {
        let mut batch = Batch::default();

        let ids = items
            .map(|i| {
                let k = i.key();
                let v = i.value();

                batch.insert(k.as_bytes().as_ref(), v.as_ref());

                k.id()
            })
            .collect();

        self.db.apply_batch(batch)?;

        Ok(ids)
    }
}
