#![allow(non_snake_case)]
use crate::asset::*;
use crate::attributes::*;
use crate::compliance::*;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::dataset::*;
use crate::endpoints::*;
use crate::models::*;
use crate::role::*;
use crate::role_binding::*;
use crate::sql_parser::*;
use crate::storage::*;
use crate::storage_setup::*;
use crate::template::*;
use crate::user::*;
use crate::user_group::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use uuid::Uuid;
create_exception!(aorist, SQLParseError, PyException);

#[aorist_concept]
pub struct Universe {
    pub name: String,
    #[constrainable]
    pub users: Option<Vec<User>>,
    #[constrainable]
    pub groups: Option<Vec<UserGroup>>,
    #[constrainable]
    pub datasets: Option<Vec<DataSet>>,
    #[constrainable]
    pub role_bindings: Option<Vec<RoleBinding>>,
    #[constrainable]
    pub endpoints: EndpointConfig,
    #[constrainable]
    pub compliance: Option<ComplianceConfig>,
    #[constrainable]
    pub models: Option<Vec<Model>>,
}
pub trait TUniverse {
    fn get_user_unixname_map(&self) -> HashMap<String, User>;
    fn get_user_permissions(&self) -> Result<HashMap<String, HashSet<String>>, String>;
}
impl TUniverse for Universe {
    fn get_user_unixname_map(&self) -> HashMap<String, User> {
        self.users
            .as_ref()
            .unwrap()
            .iter()
            .map(|x| (x.get_unixname().clone(), x.clone()))
            .collect()
    }
    fn get_user_permissions(&self) -> Result<HashMap<String, HashSet<String>>, String> {
        let umap = self.get_user_unixname_map();
        let mut map: HashMap<_, HashSet<String>> = HashMap::new();
        for binding in self.role_bindings.as_ref().unwrap() {
            let name = binding.get_user_name();
            if !umap.contains_key(name) {
                return Err(format!("Cannot find user with name {}.", name));
            }
            let user = umap.get(name).unwrap().clone();
            for perm in binding.get_role().get_permissions() {
                map.entry(user.get_unixname().clone())
                    .or_insert_with(HashSet::new)
                    .insert(perm.clone());
            }
        }
        Ok(map)
    }
}
impl InnerUniverse {
    fn get_dataset(&mut self, dataset_name: String) -> Result<&mut InnerDataSet, String> {
        if let Some(ref mut datasets) = self.datasets {
            if let Some(dataset) = datasets
                .iter_mut()
                .filter(|x| *x.get_name() == dataset_name)
                .next()
            {
                return Ok(dataset);
            }
            return Err(format!("Cannot find dataset {}", dataset_name).to_string());
        }
        return Err(format!("No datasets present when looking for {}", dataset_name).to_string());
    }
}
#[pymethods]
impl InnerUniverse {
    pub fn add_template(&mut self, t: InnerDatumTemplate, dataset_name: String) -> PyResult<()> {
        let mut dataset = self.get_dataset(dataset_name).unwrap();
        dataset.add_template(t)
    }
    pub fn add_asset(&mut self, a: InnerAsset, dataset_name: String) -> PyResult<()> {
        let mut dataset = self.get_dataset(dataset_name).unwrap();
        dataset.add_asset(a)
    }
    pub fn derive_asset(
        &mut self,
        sql: String,
        name: String,
        storage: InnerStorage,
        tmp_dir: String,
    ) -> PyResult<()> {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, &sql).unwrap();
        if ast.len() != 1 {
            return Err(SQLParseError::new_err(
                "A single SELECT statement should be provided.",
            ));
        }
        println!("AST: {:?}", &ast);
        if let Statement::Query(query) = ast.into_iter().next().unwrap() {
            if let Some(ref datasets) = self.datasets {
                let mut dataset_map = HashMap::new();
                for dataset in datasets.iter() {
                    let templates = dataset.get_mapped_datum_templates();
                    let mut asset_map = HashMap::new();
                    for asset in dataset.assets.iter() {
                        let asset_name = asset.get_name();
                        let schema = asset.get_schema();
                        let attribute_names = schema.get_attribute_names();
                        let template_name = schema.get_datum_template_name();
                        let template =
                            DatumTemplate::from(templates.get(&template_name).unwrap().clone());
                        let mut attributes = template
                            .get_attributes()
                            .into_iter()
                            .map(|v| (v.get_name().clone(), v))
                            .collect::<HashMap<String, Attribute>>();
                        let mut map = HashMap::new();
                        for k in attribute_names {
                            let attr = attributes.remove(&k);
                            if let Some(a) = attr {
                                map.insert(k.clone(), a);
                            } else {
                                return Err(SQLParseError::new_err(
                                    format!(
                                        "Could not find attribute named {} for asset {}",
                                        k, asset_name
                                    )
                                    .to_string(),
                                ));
                            }
                        }
                        asset_map.insert(asset_name, map);
                    }
                    dataset_map.insert(dataset.get_name().clone(), asset_map);
                }
                let parser = SQLParser::new(dataset_map, name);
                return parser.parse_query(*query, self, storage, tmp_dir);
            } else {
                return Err(SQLParseError::new_err("No datasets found in universe."));
            }
        } else {
            return Err(SQLParseError::new_err("Only SELECT statements supported."));
        }
    }
}
