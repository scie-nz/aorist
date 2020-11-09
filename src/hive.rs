use std::collections::HashMap;
use crate::encoding::{Encoding, CSVEncoding, ORCEncoding};
use enum_dispatch::enum_dispatch;

#[enum_dispatch(HiveLocation, Encoding)]
pub trait THiveTableCreationTagMutator {
    fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
    ) -> Result<(), String>;
}
