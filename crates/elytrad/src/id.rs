use std::{
    cell::RefCell,
    fmt::Display,
    sync::atomic::{AtomicU8, Ordering},
};

static WORKER_SEQ_ID: AtomicU8 = AtomicU8::new(0);

/// 64-bit Snowflake with the following layout:
///
/// ```
/// +------------+------------+-------------+------------+
/// | Timestamp  | Node ID    | Worker ID   | Increment  |
/// | 42 bits    | 5 bits     | 5 bits      | 12 bits    |
/// +------------+------------+-------------+------------+
/// 63         22 21        17 16         12 11          0
/// ```
///
/// Epoch that starts at 25-01-01, allowing us to generate ids until August 2164
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Snowflake(pub(crate) u64);

impl Snowflake {
    /// Returns time snowflake timestamp (ms since custom epoch)
    pub fn timestamp(&self) -> u64 {
        return self.0 >> 22;
    }

    /// Returns time snowflake unix timestamp (secs since unix epoch)
    pub fn unix_timestamp(&self) -> u64 {
        (self.timestamp() / 1000) + crate::time::EPOCH_OFFSET
    }

    pub fn worker_id(&self) -> u8 {
        return ((self.0 >> 12) & 0x1F) as u8;
    }

    pub fn node_id(&self) -> u8 {
        return ((self.0 >> 17) & 0x1F) as u8;
    }

    pub fn from_str<T>(v: T) -> Result<Self, base62::DecodeError>
    where
        T: AsRef<str>,
    {
        let id = base62::decode(v.as_ref())? as u64;

        Ok(Self(id))
    }
}

impl Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let h = base62::encode(self.0);

        f.write_str(&h)
    }
}

#[derive(Debug)]
pub struct Generator {
    worker_id: u8,
    sequence: u16,
    timestamp: u64,
}

impl Generator {
    pub fn next_id(&mut self, node_id: u8) -> Snowflake {
        assert!(node_id < 0x1F, "node id out of range: {node_id} ({})", 0x1F);

        let mut current_ts = crate::time::now();

        if current_ts == self.timestamp {
            self.sequence = (self.sequence + 1) & 0xFFF;

            if self.sequence == 0 {
                while current_ts <= self.timestamp {
                    current_ts = crate::time::now();
                }
            }
        } else {
            self.sequence = 0;
        }

        self.timestamp = current_ts;

        let id = (current_ts << 22)
            | (node_id as u64) << 17
            | (self.worker_id as u64) << 12
            | self.sequence as u64;

        Snowflake(id)
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            worker_id: WORKER_SEQ_ID.fetch_add(1, Ordering::AcqRel),
            sequence: 0,
            timestamp: 0,
        }
    }
}

thread_local! {
    static GENERATOR: RefCell<Generator> = RefCell::new(Generator::default());
}

/// Returns a new monotonic snowflake
pub fn snowflake(node_id: u8) -> Snowflake {
    GENERATOR.with(|generator| generator.borrow_mut().next_id(node_id))
}
