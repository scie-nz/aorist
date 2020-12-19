use crate::concept::AoristConcept;
use crate::constraint::{AoristConstraint, Constraint};
use crate::endpoints::EndpointConfig;
use crate::hive::THiveTableCreationTagMutator;
use crate::location::alluxio_location::AlluxioLocation;
use aorist_concept::Constrainable;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type", content = "spec")]
pub enum HiveLocation {
    #[constrainable]
    AlluxioLocation(AlluxioLocation),
}
