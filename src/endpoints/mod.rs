pub mod alluxio;
pub mod gitea;
pub mod presto;
pub mod ranger;

use crate::concept::Concept;
use crate::constraint::Constraint;
use crate::AoristConcept;
pub use alluxio::{AlluxioConfig, InnerAlluxioConfig};
use aorist_concept::{aorist_concept2, ConstrainObject, Constrainable};
use derivative::Derivative;
pub use gitea::{GiteaConfig, InnerGiteaConfig};
use paste::paste;
pub use presto::{InnerPrestoConfig, PrestoConfig};
use pyo3::prelude::*;
pub use ranger::{InnerRangerConfig, RangerConfig};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub struct EndpointConfig {
    #[constrainable]
    #[py_default = "None"]
    presto: Option<PrestoConfig>,
    #[constrainable]
    #[py_default = "None"]
    alluxio: Option<AlluxioConfig>,
    #[constrainable]
    #[py_default = "None"]
    ranger: Option<RangerConfig>,
    #[constrainable]
    #[py_default = "None"]
    gitea: Option<GiteaConfig>,
}
