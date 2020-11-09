#![allow(non_snake_case)]
use crate::access_policies::AccessPolicy;
use crate::assets::Asset;
use crate::object::TAoristObject;
use crate::python::TObjectWithPythonCodeGen;
use crate::templates::DatumTemplate;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use crate::prefect::{TObjectWithPrefectCodeGen, TObjectWithPrefectDAGCodeGen};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSet {
    name: String,
    accessPolicies: Vec<AccessPolicy>,
    datumTemplates: Vec<DatumTemplate>,
    assets: Vec<Asset>,
}
impl TAoristObject for DataSet {
    fn get_name(&self) -> &String {
        &self.name
    }
}
impl TObjectWithPythonCodeGen for DataSet {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        for asset in &self.assets {
            asset.get_python_imports(preamble);
        }
    }
}
impl TObjectWithPrefectCodeGen for DataSet {
    fn get_prefect_preamble(&self, preamble: &mut HashMap<String, String>) {
        for asset in &self.assets {
            asset.get_prefect_preamble(preamble);
        }
    }
}
impl TObjectWithPrefectDAGCodeGen for DataSet {
    fn get_prefect_dag(&self) -> Result<String, String> {
        let materialized_assets: Vec<String> = self.assets.iter().map(|x| x.get_prefect_dag().unwrap()).collect();
        Ok(materialized_assets.join("\n"))
    }
}
impl DataSet {
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
    pub fn get_mapped_datum_templates(&self) -> HashMap<String, DatumTemplate> {
        self.datumTemplates
            .iter()
            .map(|x| (x.get_name().clone(), x.clone()))
            .collect()
    }
    pub fn get_presto_schemas(&self) -> String {
        let mappedTemplates = self.get_mapped_datum_templates();
        let mut schemas: String = "".to_string();
        for asset in &self.assets {
            let schema = asset.get_presto_schemas(&mappedTemplates);
            schemas += "\n\n";
            schemas += &schema;
        }
        schemas
    }
    pub fn get_materialize_pipeline_name(&self) -> String {
        format!("materialize_{}.py", self.get_name()).to_string()
    }
    pub fn get_materialize_pipeline(&self) -> Result<String, String> {
        let mut preamble: HashMap<String, String> = HashMap::new();
        self.get_python_imports(&mut preamble);
        let imports_deduped: BTreeSet<String> = preamble.values().map(|x| x.clone()).collect();

        let mut preamble: HashMap<String, String> = HashMap::new();
        self.get_prefect_preamble(&mut preamble);
        let prefect_preamble_deduped: BTreeSet<String> = preamble.values().map(|x| x.clone()).collect();


        let code = format!(
            "{}\n{}\nwith Flow() as flow:\n{}",
            imports_deduped
                .into_iter()
                .collect::<Vec<String>>()
                .join("\n"),
            prefect_preamble_deduped
                .into_iter()
                .collect::<Vec<String>>()
                .join("\n"),
            self.get_prefect_dag()?,
        );
        Ok(code)
    }
}
