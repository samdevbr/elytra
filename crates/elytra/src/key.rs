use snowflake::Snowflake;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PartitionKey(u128);

impl PartitionKey {
    pub fn new(tag: u64, id: Snowflake) -> Self {
        let id = id.as_u64() as u128;
        let tag = tag as u128;

        Self(tag << 64 | id)
    }

    pub fn tag(&self) -> u64 {
        (self.0 >> 64) as u64
    }

    pub fn id(&self) -> Snowflake {
        Snowflake::new(self.0 as u64)
    }
}
