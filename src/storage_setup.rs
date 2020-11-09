#![allow(non_snake_case)]
use crate::storage::Storage;
use indoc::indoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RemoteImportStorageSetup {
    remote: Storage,
    local: Vec<Storage>,
}
impl RemoteImportStorageSetup {
    pub fn get_local_storage(&self) -> &Vec<Storage> {
        &self.local
    }
    pub fn get_presto_schemas(&self, name: &String, columnSchema: String) -> String {
        let mut schemas: Vec<String> = Vec::new();
        for storage in self.get_local_storage() {
            if storage.is_hive_storage() {
                let mut tags: HashMap<String, String> = HashMap::new();
                storage.populate_table_creation_tags(&mut tags).unwrap();
                let tags_str = match tags.len() {
                    0 => "".to_string(),
                    _ => format!(
                        " WITH (\n    {}\n)",
                        tags.iter()
                            .map(|(k, v)| format!("{}='{}'", k, v))
                            .collect::<Vec<String>>()
                            .join(",\n    ")
                    )
                    .to_string(),
                };
                schemas.push(format!(
                    indoc! {
                        "CREATE TABLE IF NOT EXISTS {table} (
                                {column_schema}
                            ){tags_str};"
                    },
                    table = name,
                    column_schema = columnSchema.replace("\n", "\n    "),
                    tags_str = tags_str,
                ));
            }
        }
        schemas.join("\n")
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "spec")]
pub enum StorageSetup {
    RemoteImportStorageSetup(RemoteImportStorageSetup),
}

impl StorageSetup {
    pub fn get_local_storage(&self) -> &Vec<Storage> {
        match self {
            StorageSetup::RemoteImportStorageSetup(x) => x.get_local_storage(),
        }
    }
    pub fn get_presto_schemas(&self, name: &String, columnSchema: String) -> String {
        match self {
            StorageSetup::RemoteImportStorageSetup(x) => x.get_presto_schemas(name, columnSchema),
        }
    }
}
