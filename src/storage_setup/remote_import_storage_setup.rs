#![allow(non_snake_case)]
use crate::endpoints::EndpointConfig;
use crate::prefect::{TObjectWithPrefectCodeGen, TPrefectStorage, TPrefectStorageSetup};
use crate::python::TObjectWithPythonCodeGen;
use crate::schema::DataSchema;
use crate::storage::Storage;
use crate::template::DatumTemplate;
use indoc::indoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RemoteImportStorageSetup {
    remote: Storage,
    local: Vec<Storage>,
}
impl TObjectWithPrefectCodeGen for RemoteImportStorageSetup {
    fn get_prefect_preamble(
        &self,
        preamble: &mut HashMap<String, String>,
        endpoints: &EndpointConfig,
    ) {
        self.remote.get_prefect_preamble(preamble, endpoints);
        for storage in &self.local {
            storage.get_prefect_preamble(preamble, endpoints);
        }
    }
}
impl TPrefectStorageSetup for RemoteImportStorageSetup {
    fn get_prefect_dag(
        &self,
        schema: &DataSchema,
        templates: &HashMap<String, DatumTemplate>,
        table_name: &String,
        endpoints: &EndpointConfig,
    ) -> Result<String, String> {
        let remote_dag = self.remote.get_prefect_dag(schema)?;
        let mut dag = format!("{}", remote_dag);
        let columnSchema = schema.get_presto_schema(templates);

        for (i, local_storage) in self.local.iter().enumerate() {
            // TODO: 1st, 2nd and 6th argument should be provided by remote_dag
            let local = local_storage.get_prefect_ingest_dag(
                "/tmp".to_string(),
                "decode_file.no_header".to_string(),
                schema,
                templates,
                format!("upload_{}", i),
                "decode_file_remove_header".to_string(),
                endpoints,
            )?;
            // TODO: last argument should come from upstream pipeline
            let schema_creation = self.get_presto_schema_creation_task(
                table_name,
                &columnSchema,
                local_storage,
                format!("create_table_{}", i),
                format!("upload_{}_encode", i),
                endpoints,
            );

            dag = format!("{}\n{}\n{}", &dag, local, schema_creation);
        }
        Ok(dag.to_string())
    }
}

impl RemoteImportStorageSetup {
    pub fn get_local_storage(&self) -> &Vec<Storage> {
        &self.local
    }
    // TODO: move to Storage. Also, need presto-cli to be configurable
    pub fn get_presto_schema_creation_task(
        &self,
        name: &String,
        columnSchema: &String,
        storage: &Storage,
        task_name: String,
        upstream_task_name: String,
        endpoints: &EndpointConfig,
    ) -> String {
        let schema = self.get_presto_schema(name, columnSchema, storage, endpoints);
        // TODO: get presto port from endpoints
        format!(
            indoc! {
                "
                    {task_name} = ShellTask(
                        command=\"\"\"
                        presto -e \"{schema}\"
                        \"\"\"
                    )(upstream_tasks=[{upstream_task_name}])
                "
            },
            task_name = task_name,
            upstream_task_name = upstream_task_name,
            schema = schema,
        )
    }
    // TODO: move to Storage
    pub fn get_presto_schema(
        &self,
        name: &String,
        columnSchema: &String,
        storage: &Storage,
        endpoints: &EndpointConfig,
    ) -> String {
        if storage.is_hive_storage() {
            let mut tags: HashMap<String, String> = HashMap::new();
            storage
                .populate_table_creation_tags(&mut tags, endpoints)
                .unwrap();
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
            return format!(
                indoc! {
                    "CREATE TABLE IF NOT EXISTS {table} (
                            {column_schema}
                        ){tags_str};"
                },
                table = name,
                column_schema = columnSchema.replace("\n", "\n    "),
                tags_str = tags_str,
            )
            .to_string();
        }
        "".to_string()
    }
    pub fn get_presto_schemas(
        &self,
        name: &String,
        columnSchema: String,
        endpoints: &EndpointConfig,
    ) -> String {
        let mut schemas: Vec<String> = Vec::new();
        for storage in self.get_local_storage() {
            schemas.push(self.get_presto_schema(name, &columnSchema, storage, endpoints));
        }
        schemas.join("\n")
    }
}
impl TObjectWithPythonCodeGen for RemoteImportStorageSetup {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        self.remote.get_python_imports(preamble);
        for storage in &self.local {
            storage.get_python_imports(preamble);
        }
    }
}
