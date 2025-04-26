use serde::{Deserialize, Serialize};
use snowflake::{snowflake, Snowflake};

use crate::{plan::LogicalPlan, types::Map};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Query {
    Upsert {
        id: Option<String>,
        collection: String,
        input: Map,
    },
    Select {
        id: Option<String>,
        collection: String,
        projection: Vec<String>,
    },
}

impl Query {
    pub fn build(self) -> crate::Result<LogicalPlan> {
        let plan = match self {
            Query::Upsert {
                id,
                collection,
                input,
            } => {
                let id = match id {
                    Some(id) => Snowflake::from_str(&id)?,
                    None => snowflake(),
                };

                LogicalPlan::UpsertDocument {
                    id,
                    collection,
                    input,
                }
            }
            Query::Select { .. } => todo!("Select queries are not implemented yet."),
        };

        Ok(plan)
    }
}
