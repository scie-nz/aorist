#![allow(non_snake_case)]
use crate::object::AoristObject;
use getset::Getters;
use serde::{Deserialize, Serialize};
use crate::imports::TAoristImport;
use crate::utils::read_file;

#[serde(tag = "type")]
#[derive(Serialize, Deserialize, Clone, Getters, Debug, PartialEq)]
pub struct LocalFileImport {
    #[getset(get = "pub")]
    filename: String,
}

impl TAoristImport for LocalFileImport {
    fn get_objects(&self) -> Vec<AoristObject> {
        let filename = self.filename();
        let imported_objects = read_file(&filename);
        imported_objects
    }
}
