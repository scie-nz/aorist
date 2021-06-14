#![allow(non_snake_case)]
use crate::access_policy::*;
use crate::asset::*;
use crate::concept::{AoristConcept, ConceptEnum};
use crate::object::TAoristObject;
use crate::storage_setup::ComputedFromLocalData;
// TODO uncomment
//use crate::template::DatumTemplate;
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;

#[aorist]
pub struct DataSet {
    pub name: String,
    pub description: String,
    pub sourcePath: String,
    #[constrainable]
    pub accessPolicies: Vec<AccessPolicy>,
    // TODO uncomment
    /*#[constrainable]
    pub datumTemplates: Vec<DatumTemplate>,*/
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

    // TODO uncomment
    /*pub fn get_templates(&self) -> Vec<DatumTemplate> {
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
}*/

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

// TODO uncomment
/*
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
*/
