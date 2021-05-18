#![allow(non_snake_case)]
use crate::access_policy::*;
use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristConceptChildren, Concept};
use crate::constraint::Constraint;
use crate::encoding::*;
use crate::object::TAoristObject;
use crate::storage::*;
use crate::storage_setup::ComputedFromLocalData;
use crate::template::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren,  InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct DataSet {
    pub name: String,
    pub description: String,
    pub sourcePath: String,
    #[constrainable]
    #[py_default = "Vec::new()"]
    pub accessPolicies: Vec<AccessPolicy>,
    #[constrainable]
    pub datumTemplates: Vec<DatumTemplate>,
    #[constrainable]
    pub assets: BTreeMap<String, Asset>,
}

impl DataSet {
    pub fn get_assets(&self) -> Vec<Asset> {
        self.assets
            .values()
            .collect::<Vec<_>>()
            .iter()
            .map(|x| (*x).clone())
            .collect()
    }
    pub fn get_templates(&self) -> Vec<DatumTemplate> {
        self.datumTemplates.clone()
    }
    pub fn get_template_for_asset<T: TAsset>(&self, asset: &T) -> Result<DatumTemplate, String> {
        let schema = asset.get_schema();
        let template_name = schema.get_datum_template_name().unwrap();
        let mapped_templates = self.get_mapped_datum_templates();
        let template = mapped_templates.get(&template_name);
        match template {
            Some(template) => Ok(template.clone()),
            None => Err(format!(
                "Could not find template for asset {} in dataset {}",
                asset.get_name(),
                self.name,
            )),
        }
    }
    pub fn get_asset(&self, name: String) -> Result<Asset, String> {
        if let Some(asset) = self.assets.get(&name) {
            return Ok(asset.clone());
        }
        Err(format!("Could not find asset {} in dataset {}.", name, self.name).to_string())
    }
    // TODO: should reference identifier tuple
    pub fn get_source_assets(
        &self,
        setup: &ComputedFromLocalData,
    ) -> Result<BTreeMap<String, Asset>, String> {
        let mut assets = BTreeMap::new();
        for name in setup.source_asset_names.values() {
            assets.insert(name.clone(), self.get_asset(name.clone())?);
        }
        Ok(assets)
    }
}
impl TAoristObject for DataSet {
    fn get_name(&self) -> &String {
        &self.name
    }
}
pub trait TDataSet {
    fn get_mapped_datum_templates(&self) -> HashMap<String, DatumTemplate>;
}
impl TDataSet for DataSet {
    fn get_mapped_datum_templates(&self) -> HashMap<String, DatumTemplate> {
        self.datumTemplates
            .iter()
            .map(|x| (x.get_name().clone(), x.clone()))
            .collect()
    }
}
impl InnerDataSet {
    pub fn get_mapped_datum_templates(&self) -> HashMap<String, InnerDatumTemplate> {
        self.datumTemplates
            .iter()
            .map(|x| (x.get_name().clone(), x.clone()))
            .collect()
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
#[pymethods]
impl InnerDataSet {
    pub fn add_template(&mut self, t: InnerDatumTemplate) -> PyResult<()> {
        self.datumTemplates.push(t);
        Ok(())
    }
    pub fn add_asset(&mut self, a: InnerAsset) -> PyResult<()> {
        let asset_name = a.get_name();
        assert!(!self.assets.contains_key(&asset_name));
        self.assets.insert(a.get_name(), a);
        Ok(())
    }
    pub fn get_static_data_table(&self, asset_name: String) -> PyResult<InnerStaticDataTable> {
        match self.assets.get(&asset_name) {
            Some(InnerAsset::StaticDataTable(x)) => Ok(x.clone()),
            Some(_) => panic!("Asset {} is not a StaticDataTable.", asset_name),
            _ => panic!("Dataset does not contain asset called {}.", asset_name),
        }
    }
    pub fn replicate_to_local(
        &self,
        storage: InnerStorage,
        tmp_dir: String,
        tmp_encoding: InnerEncoding,
    ) -> InnerDataSet {
        InnerDataSet {
            name: self.name.clone(),
            description: self.description.clone(),
            sourcePath: self.sourcePath.clone(),
            accessPolicies: self.accessPolicies.clone(),
            datumTemplates: self.datumTemplates.clone(),
            assets: self
                .assets
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.replicate_to_local(
                            storage.clone(),
                            tmp_dir.clone(),
                            tmp_encoding.clone(),
                        ),
                    )
                })
                .collect(),
            tag: self.tag.clone(),
        }
    }
    pub fn get_attributes_for_asset(&self, asset_name: String) -> PyResult<Vec<InnerAttribute>> {
        let asset = self.assets.get(&asset_name).unwrap();
        let schema = asset.get_schema();
        let template_name = schema.get_datum_template_name().unwrap();
        let mapped_templates = self.get_mapped_datum_templates();
        let template = mapped_templates.get(&template_name);
        match template {
            Some(template) => {
                let mut attributes = template
                    .get_attributes()
                    .into_iter()
                    .map(|x| (x.name().unwrap().clone(), x))
                    .collect::<HashMap<_, _>>();
                let mut asset_attributes = Vec::new();
                for attribute_name in schema.get_attribute_names() {
                    asset_attributes.push(attributes.remove(&attribute_name).unwrap());
                }
                Ok(asset_attributes)
            }
            None => panic!(
                "Could not find template for asset {} in dataset {}",
                asset.get_name(),
                self.name,
            ),
        }
    }
}
