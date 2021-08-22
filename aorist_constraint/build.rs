use aorist_util::{read_file, process_constraints, process_constraints_py};
fn main() {
    let raw_objects = read_file("constraints.yaml");
    process_constraints(&raw_objects);
    process_constraints_py(&raw_objects);
}
