use std::{
    cell::RefCell,
    fmt::Display,
    sync::atomic::{AtomicU8, Ordering},
};

static THREAD_ID_SEQ: AtomicU8 = AtomicU8::new(0);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Snowflake(u64);

impl Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let h = base62::encode(self.0);

        f.write_str(&h)
    }
}

#[derive(Debug)]
pub struct Generator {
    thread_id: u8,
    sequence: u16,
    timestamp: u64,
}

impl Generator {
    pub fn next_id(&mut self, node_id: u8) -> Snowflake {
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
            | (self.thread_id as u64) << 12
            | self.sequence as u64;

        Snowflake(id)
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            thread_id: THREAD_ID_SEQ.fetch_add(1, Ordering::AcqRel),
            sequence: 0,
            timestamp: 0,
        }
    }
}

thread_local! {
    static GENERATOR: RefCell<Generator> = RefCell::new(Generator::default());
}

pub fn snowflake(node_id: u8) -> Snowflake {
    GENERATOR.with(|generator| generator.borrow_mut().next_id(node_id))
}
