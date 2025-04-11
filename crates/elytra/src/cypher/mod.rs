pub mod label;

pub trait ToCypher {
    fn to_cypher(&self) -> String;
}
