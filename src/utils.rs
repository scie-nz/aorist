#![allow(non_snake_case)]
use crate::object::AoristObject;
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
