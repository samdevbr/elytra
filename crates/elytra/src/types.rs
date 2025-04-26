use bincode::Encode;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    Array(Vec<Value>),
    Map(Map),
}

impl Encode for Value {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        match self {
            Value::Null => ().encode(encoder),
            Value::Bool(b) => b.encode(encoder),
            Value::Int(i) => i.encode(encoder),
            Value::UInt(i) => i.encode(encoder),
            Value::Float(f) => f.encode(encoder),
            Value::String(s) => s.encode(encoder),
            Value::Bytes(items) => items.encode(encoder),
            Value::Array(values) => {
                values.len().encode(encoder)?;

                for v in values {
                    v.encode(encoder)?;
                }

                Ok(())
            }
            Value::Map(map) => map.encode(encoder),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    #[serde(flatten)]
    inner: IndexMap<String, Value>,
}

impl<K> From<Vec<(K, Value)>> for Map
where
    K: ToString,
{
    fn from(value: Vec<(K, Value)>) -> Self {
        Self {
            inner: value.into_iter().map(|(k, v)| (k.to_string(), v)).collect(),
        }
    }
}

impl Encode for Map {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        self.inner.len().encode(encoder)?;

        for kv in self.inner.iter() {
            kv.encode(encoder)?;
        }

        Ok(())
    }
}
