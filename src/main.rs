use serde::{Serialize, Deserialize};
use std::fs;
use lib::datasets::{Dataset, get_dataset};

fn main() -> Result<(), String> {
    let dataset: Box<dyn Dataset> = get_dataset().unwrap();
    println!("{}", dataset.to_yaml());
    Ok(())
}
