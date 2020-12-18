use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::location::gcs_location::GCSLocation;
use aorist_concept::Constrainable;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use uuid::Uuid;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type", content = "spec")]
pub enum RemoteLocation {
    #[constrainable]
    GCSLocation(GCSLocation),
}
