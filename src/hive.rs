use crate::encoding::{CSVEncoding, Encoding, ORCEncoding};
use enum_dispatch::enum_dispatch;
use std::collections::HashMap;
use crate::data_setup::EndpointConfig;

#[enum_dispatch(HiveLocation, Encoding)]
pub trait THiveTableCreationTagMutator {
    fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
        endpoints: &EndpointConfig,
    ) -> Result<(), String>;
}
