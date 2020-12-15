use crate::constraint::Constraint;
use crate::endpoints::EndpointConfig;
use crate::hive::THiveTableCreationTagMutator;
use crate::location::alluxio_location::AlluxioLocation;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::concept::AoristConcept;
use aorist_concept::Constrainable;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type", content = "spec")]
pub enum HiveLocation {
    #[constrainable]
    AlluxioLocation(AlluxioLocation),
}
