use serde::{Serialize, Deserialize};
use crate::storage::Storage;
use crate::encoding::Encoding;
use crate::schema::DataSchema;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticDataTable {
    storage: Storage,
    encoding: Encoding,
    schema: DataSchema,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content="spec")]
pub enum Asset {
    StaticDataTable(StaticDataTable),
}


