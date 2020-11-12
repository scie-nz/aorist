#![allow(non_snake_case)]
use crate::object::AoristObject;
use crate::data_setup::{DataSetup, ParsedDataSetup};
use std::fs;


pub fn read_file(filename: &str) -> Vec<AoristObject> {
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
