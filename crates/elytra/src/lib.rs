pub mod error;
pub mod key;
mod plan;
pub mod query;
pub mod shard;
pub mod types;
mod util;

pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
