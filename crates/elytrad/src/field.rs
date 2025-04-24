use xxhash_rust::xxh64::xxh64;

#[derive(Debug)]
pub struct FieldKey(pub(crate) [u8; 8]);

pub struct Field {
    name: String,
    value: serde_cbor::Value,
}

impl Field {
    pub fn key(&self) -> FieldKey {
        let hash = xxh64(self.name.as_bytes(), 0);

        FieldKey(hash.to_be_bytes())
    }
}
