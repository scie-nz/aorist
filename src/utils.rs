#![allow(non_snake_case)]
use crate::data_setup::data_setup::DataSetup;
use crate::data_setup::parsed_data_setup::ParsedDataSetup;
use crate::object::AoristObject;
use std::fs;
use thiserror::Error;

pub fn read_file(filename: &str) -> Vec<AoristObject> {
    println!("Reading {}", filename);
    let s = fs::read_to_string(filename).unwrap();
    let objects: Vec<AoristObject> = s
        .split("\n---\n")
        .filter(|x| x.len() > 0)
        .map(|x| serde_yaml::from_str(x).unwrap())
        .collect();
    objects
}

pub fn get_data_setup() -> ParsedDataSetup {
    let objects = read_file("basic.yaml");
    let v: Vec<Option<&DataSetup>> = objects
        .iter()
        .map(|x| match x {
            AoristObject::DataSetup(x) => Some(x),
            _ => None,
        })
        .filter(|x| x.is_some())
        .collect();
    let dataSetup: DataSetup = v.first().unwrap().unwrap().to_owned();

    dataSetup.parse(objects)
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum GetSetError {
    #[error("Get was called, but attribute was not set: {0:#?}")]
    GetError(String),
    #[error("Set was called twice for the attribute: {0:#?}")]
    SetError(String),
}
