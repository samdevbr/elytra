use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Sled(#[from] sled::Error),

    #[error(transparent)]
    BinodeEncode(#[from] bincode::error::EncodeError),

    #[error(transparent)]
    Base62Decode(#[from] base62::DecodeError),
}
