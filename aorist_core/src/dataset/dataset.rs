#![allow(dead_code)]
use crate::access_policy::*;
use crate::asset::*;
use crate::concept::{AoristConcept, ConceptEnum};
use aorist_primitives::TAoristObject;
use crate::storage_setup::*;
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use linked_hash_map::LinkedHashMap;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[aorist]
pub struct DataSet {
    pub name: String,
    pub description: String,
    pub source_path: String,
    #[constrainable]
    pub access_policies: Vec<AccessPolicy>,
    #[constrainable]
    pub datum_templates: Vec<DatumTemplate>,
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
        self.datum_templates.clone()
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
    fn get_mapped_datum_templates(&self) -> LinkedHashMap<String, DatumTemplate>;
}

impl TDataSet for DataSet {
    fn get_mapped_datum_templates(&self) -> LinkedHashMap<String, DatumTemplate> {
        self.datum_templates
            .iter()
            .map(|x| (x.get_name().clone(), x.clone()))
            .collect()
    }
}
