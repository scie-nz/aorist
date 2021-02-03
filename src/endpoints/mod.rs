pub mod alluxio;
pub mod gitea;
pub mod presto;
pub mod ranger;

use crate::concept::Concept;
use crate::constraint::Constraint;
use crate::AoristConcept;
pub use alluxio::{AlluxioConfig, ConstrainedAlluxioConfig};
use aorist_concept::{aorist_concept2, Constrainable, PythonObject};
use derivative::Derivative;
pub use gitea::{ConstrainedGiteaConfig, GiteaConfig};
use paste::paste;
pub use presto::{ConstrainedPrestoConfig, PrestoConfig};
use pyo3::prelude::*;
pub use ranger::{ConstrainedRangerConfig, RangerConfig};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub struct EndpointConfig {
    #[py_default = "None"]
    presto: Option<PrestoConfig>,
    #[py_default = "None"]
    alluxio: Option<AlluxioConfig>,
    #[py_default = "None"]
    ranger: Option<RangerConfig>,
    #[py_default = "None"]
    gitea: Option<GiteaConfig>,
}
