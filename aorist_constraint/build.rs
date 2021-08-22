use aorist_util::{process_constraints, process_constraints_py, read_file};
fn main() {
    let raw_objects = read_file("constraints.yaml");
    process_constraints(&raw_objects);
    process_constraints_py(&raw_objects);
}
