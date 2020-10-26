use lib::datasets::get_dataset;

fn main() -> Result<(), String> {
    let dataset = get_dataset();
    println!("{}", dataset.to_yaml());
    Ok(())
}
