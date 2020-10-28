use std::collections::HashMap;

pub trait THiveTableCreationTagMutator {
    fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
    ) -> Result<(), String>;
}

