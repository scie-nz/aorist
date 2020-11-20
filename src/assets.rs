#![allow(non_snake_case)]
use crate::endpoints::EndpointConfig;
use crate::prefect::{TObjectWithPrefectCodeGen, TPrefectAsset, TPrefectStorageSetup};
use crate::python::TObjectWithPythonCodeGen;
use crate::schema::DataSchema;
use crate::storage_setup::StorageSetup;
use crate::templates::DatumTemplate;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sqlparser::ast::{Query, SetExpr, Select, SelectItem, TableWithJoins, ObjectName, Ident, TableFactor};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StaticDataTable {
    name: String,
    setup: StorageSetup,
    schema: DataSchema,
}
impl StaticDataTable {
    pub fn get_presto_schemas(
        &self,
        templates: &HashMap<String, DatumTemplate>,
        endpoints: &EndpointConfig,
    ) -> String {
        let columnSchema = self.schema.get_presto_schema(templates);
        self.setup
            .get_presto_schemas(self.get_name(), columnSchema, endpoints)
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
impl TObjectWithPythonCodeGen for StaticDataTable {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        self.setup.get_python_imports(preamble);
    }
}
impl TObjectWithPrefectCodeGen for StaticDataTable {
    fn get_prefect_preamble(
        &self,
        preamble: &mut HashMap<String, String>,
        endpoints: &EndpointConfig,
    ) {
        self.setup.get_prefect_preamble(preamble, endpoints);
    }
}
impl TPrefectAsset for StaticDataTable {
    fn get_prefect_dag(
        &self,
        templates: &HashMap<String, DatumTemplate>,
        endpoints: &EndpointConfig,
    ) -> Result<String, String> {
        self.setup
            .get_prefect_dag(&self.schema, templates, self.get_name(), endpoints)
    }
}

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "spec")]
pub enum Asset {
    StaticDataTable(StaticDataTable),
}
impl Asset {
    pub fn get_presto_schemas(
        &self,
        templates: &HashMap<String, DatumTemplate>,
        endpoints: &EndpointConfig,
    ) -> String {
        match self {
            Asset::StaticDataTable(x) => x.get_presto_schemas(templates, endpoints),
        }
    }
    pub fn get_presto_query() -> Result<Query, String> {
        let projection = vec![
            SelectItem::Wildcard,
        ];
        let table = vec![
            TableWithJoins{
                relation: TableFactor::Table{
                    // TODO: remove when this function is actually used
                    name: ObjectName(vec![Ident::new("some_table")]),
                    alias: None,
                    args: Vec::new(),
                    with_hints: Vec::new(),
                },
                joins: Vec::new(),
            },
        ];
        let select = Select {
            distinct: false,
            top: None,
            projection: projection,
            from: table,
            selection: None,
            group_by: Vec::new(),
            having: None,

        };
        let query = Query {
            ctes: Vec::new(),
            body: SetExpr::Select(Box::new(select)),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            fetch: None,
        };
        Ok(query)
    }
}
