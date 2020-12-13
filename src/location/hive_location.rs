use crate::endpoints::EndpointConfig;
use crate::hive::THiveTableCreationTagMutator;
use crate::location::alluxio_location::AlluxioLocation;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "spec")]
pub enum HiveLocation {
    AlluxioLocation(AlluxioLocation),
}
