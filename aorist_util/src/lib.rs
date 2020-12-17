use serde_yaml::{from_str, Value};
use std::collections::HashMap;
use std::fs;

pub fn read_file(filename: &str) -> Vec<HashMap<String, Value>> {
    let s = fs::read_to_string(filename).unwrap();
    s.split("\n---\n")
        .filter(|x| x.len() > 0)
        .map(|x| from_str(x).unwrap())
        .collect()
}
pub fn get_raw_objects_of_type(
    raw_objects: &Vec<HashMap<String, Value>>,
    object_type: String,
) -> Vec<HashMap<String, Value>> {
    raw_objects
        .into_iter()
        .filter(|x| x.get("type").unwrap().as_str().unwrap() == object_type)
        .map(|x| {
            x.get("spec")
                .unwrap()
                .as_mapping()
                .unwrap()
                .into_iter()
                .map(|(k, v)| (k.as_str().unwrap().into(), v.clone()))
                .collect()
        })
        .collect::<Vec<HashMap<String, Value>>>()
}

