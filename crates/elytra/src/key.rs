use std::ops::{Deref, DerefMut};

use sled::IVec;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key([u8; 32]);

impl AsMut<[u8]> for Key {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl AsRef<[u8]> for Key {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl Deref for Key {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl DerefMut for Key {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl From<Vec<u8>> for Key {
    fn from(value: Vec<u8>) -> Self {
        assert!(value.len() == 32, "invalid key length");

        let mut buf = [0u8; 32];

        buf.copy_from_slice(&value);

        Self(buf)
    }
}

impl Into<IVec> for Key {
    fn into(self) -> IVec {
        IVec::from(self.as_ref())
    }
}

impl From<IVec> for Key {
    fn from(value: IVec) -> Self {
        Self::from(value.to_vec())
    }
}
