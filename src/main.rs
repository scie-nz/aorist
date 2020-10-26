use lib::datasets::get_data_setup;

fn main() -> Result<(), String> {
    let setup = get_data_setup();
    for dataset in setup.get_datasets() {
        println!("{}", dataset.to_yaml());
        println!("{}", dataset.get_presto_schemas().unwrap());
    }
    for user in setup.get_users() {
        println!("{}", user.to_yaml());
    }
    for group in setup.get_groups() {
        println!("{}", group.to_yaml());
    }
    Ok(())
}
