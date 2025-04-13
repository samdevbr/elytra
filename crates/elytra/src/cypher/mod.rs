pub mod label;

pub use label::*;

pub trait ToCypher {
    fn to_cypher(self) -> String;
}
