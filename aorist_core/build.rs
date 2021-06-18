use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    attributes: Vec<String>,
}

fn main() {
   let toml_str = fs::read_to_string("aorist.toml").unwrap(); 
   let decoded: Config = toml::from_str(&toml_str).unwrap();
}
