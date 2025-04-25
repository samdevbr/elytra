pub mod error;
pub mod key;
pub mod shard;
mod util;

pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
