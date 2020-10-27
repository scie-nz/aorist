#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use crate::storage::Storage;
use crate::schema::DataSchema;
use crate::templates::DatumTemplate;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StaticDataTable {
    name: String,
    storage: Vec<Storage>,
    schema: DataSchema,
}
impl StaticDataTable {
    pub fn get_presto_schemas(&self, templates: &HashMap<String, DatumTemplate>, indent: usize) -> String {
        let columnSchema = self.schema.get_presto_schema(templates, indent);
        let mut schemas: Vec<String> = Vec::new();
        for storage in &self.storage {
            if storage.is_hive_storage() {
                let mut tags: HashMap<String, String> = HashMap::new();
                storage.populate_table_creation_tags(&mut tags).unwrap();
                let tags_str = match tags.len() {
                    0 => "".to_string(),
                    _ => format!(
                        " WITH (\n{}\n)",
                        tags.iter()
                            .map(|(k, v)| format!("{:indent$}{}='{}'", "", k, v, indent=indent))
                            .collect::<Vec<String>>()
                            .join("\n,")
                    ).to_string()
                };
                schemas.push(
                    format!("\
                        CREATE TABLE IF NOT EXISTS {table} (\n\
                            {column_schema}\n\
                        ){tags_str};",
                        table=self.get_name(),
                        column_schema=columnSchema,
                        tags_str=tags_str,
                    )
                );
            }
        }
        schemas.join("\n")
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content="spec")]
pub enum Asset {
    StaticDataTable(StaticDataTable),
}
impl Asset {
    pub fn get_presto_schemas(&self, templates: &HashMap<String, DatumTemplate>, indent: usize) -> String {
        match self {
            Asset::StaticDataTable(x) => x.get_presto_schemas(templates, indent),
        }
    }
}
