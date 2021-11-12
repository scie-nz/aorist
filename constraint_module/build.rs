use aorist_util::{process_constraints_new, read_file};
fn main() {
    pyo3_build_config::add_extension_module_link_args();
    let raw_objects = read_file("../aorist_constraint/constraints.yaml");
    process_constraints_new(&raw_objects);
}
