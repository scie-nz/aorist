use serde::{Serialize, Deserialize};
use std::fs;
use lib::datasets::{Dataset, get_dataset};

fn main() -> Result<(), String> {
    let dataset = get_dataset();
    println!("{}", dataset.to_yaml());
    Ok(())
}
