#![allow(non_snake_case)]
use crate::access_policy::AccessPolicy;
use crate::asset::Asset;
use crate::concept::AoristConcept;
use crate::constraint::{AoristConstraint, Constraint};
use crate::endpoints::EndpointConfig;
use crate::object::TAoristObject;
use crate::prefect::{TObjectWithPrefectCodeGen, TPrefectAsset, TPrefectDataSet};
use crate::python::TObjectWithPythonCodeGen;
use crate::template::DatumTemplate;
use aorist_concept::Constrainable;
use indoc::formatdoc;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;
use textwrap::indent;
use uuid::Uuid;
use derivative::Derivative;

#[derive(Derivative, Serialize, Deserialize, Default, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct DataSet {
    name: String,
    #[constrainable]
    accessPolicies: Vec<AccessPolicy>,
    #[constrainable]
    datumTemplates: Vec<DatumTemplate>,
    #[constrainable]
    assets: Vec<Asset>,
    uuid: Option<Uuid>,
    #[serde(skip)]
    #[derivative(PartialEq="ignore", Debug="ignore")]
    constraints: Vec<Rc<Constraint>>,
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
    fn get_prefect_preamble(
        &self,
        preamble: &mut HashMap<String, String>,
        endpoints: &EndpointConfig,
    ) {
        for asset in &self.assets {
            asset.get_prefect_preamble(preamble, endpoints);
        }
    }
}
impl TPrefectDataSet for DataSet {
    fn get_prefect_dag(&self, endpoints: &EndpointConfig) -> Result<String, String> {
        let mappedTemplates = self.get_mapped_datum_templates();
        let materialized_assets: Vec<String> = self
            .assets
            .iter()
            .map(|x| x.get_prefect_dag(&mappedTemplates, endpoints).unwrap())
            .collect();
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
    pub fn get_presto_schemas(&self, endpoints: &EndpointConfig) -> String {
        let mappedTemplates = self.get_mapped_datum_templates();
        let mut schemas: String = "".to_string();
        for asset in &self.assets {
            let schema = asset.get_presto_schemas(&mappedTemplates, endpoints);
            schemas += "\n\n";
            schemas += &schema;
        }
        schemas
    }
    pub fn get_materialize_pipeline_name(&self) -> String {
        format!("materialize_{}.py", self.get_name()).to_string()
    }
    pub fn get_materialize_pipeline(&self, endpoints: &EndpointConfig) -> Result<String, String> {
        let mut preamble: HashMap<String, String> = HashMap::new();
        self.get_python_imports(&mut preamble);
        let imports_deduped: BTreeSet<String> = preamble.values().map(|x| x.clone()).collect();

        let mut preamble: HashMap<String, String> = HashMap::new();
        self.get_prefect_preamble(&mut preamble, endpoints);
        let prefect_preamble_deduped: BTreeSet<String> =
            preamble.values().map(|x| x.clone()).collect();

        let code = formatdoc! {
            "{}
             {}
             with Flow('Test Flow') as flow:
             {}
             flow.register(project_name='Test')",
            imports_deduped
                .into_iter()
                .collect::<Vec<String>>()
                .join("\n"),
            prefect_preamble_deduped
                .into_iter()
                .collect::<Vec<String>>()
                .join("\n"),
            indent(&self.get_prefect_dag(endpoints)?, "    "),
        };
        Ok(code)
    }
}
