use crate::location::alluxio_location::AlluxioLocation;
use serde::{Deserialize, Serialize};
use enum_dispatch::enum_dispatch;
use crate::hive::THiveTableCreationTagMutator;
use crate::endpoints::EndpointConfig;
use std::collections::HashMap;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "spec")]
pub enum HiveLocation {
    AlluxioLocation(AlluxioLocation),
}
