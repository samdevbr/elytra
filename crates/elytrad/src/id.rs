use std::{fmt::Display, sync::atomic::Ordering, time::SystemTime};

use portable_atomic::AtomicU128;

static STATE: AtomicU128 = AtomicU128::new(0);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id {
    bytes: [u8; 16],
}

impl AsRef<[u8]> for Id {
    fn as_ref(&self) -> &[u8] {
        &self.bytes[..]
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = u128::from_be_bytes(self.bytes);
        let s = base62::encode(id);

        f.write_str(&s)
    }
}

pub fn generate(shard_id: u16) -> Id {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system clock error")
        .as_millis();

    loop {
        let state = STATE.load(Ordering::Acquire);
        let last_ts = state >> 64;
        let last_seq = state as u32;

        let (ts, seq) = if now > last_ts {
            (now, 0)
        } else {
            (last_ts, last_seq.wrapping_add(1))
        };

        let new_state = (ts << 64) | seq as u128;

        if STATE
            .compare_exchange(state, new_state, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
        {
            let id = ((shard_id as u128) << 112) | (ts << 48) | seq as u128;

            return Id {
                bytes: id.to_be_bytes(),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

    use crate::id::generate;

    #[test]
    fn test_id_generation() {
        let mut ids = Vec::with_capacity(1_000_000);

        (0..1_000_000)
            .into_par_iter()
            .map(|_| generate(0))
            .collect_into_vec(&mut ids);

        let map = ids.iter().fold(BTreeMap::new(), |mut map, id| {
            map.entry(id).and_modify(|c| *c += 1).or_insert(1);

            map
        });

        let duplicates: BTreeMap<_, _> = map.into_iter().filter(|(_, v)| *v > 1).collect();

        assert_eq!(duplicates.len(), 0);
    }
}
