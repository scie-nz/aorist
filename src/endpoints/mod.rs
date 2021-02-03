pub mod alluxio;
pub mod gitea;
pub mod presto;
pub mod ranger;

use crate::concept::Concept;
use crate::constraint::Constraint;
pub use crate::endpoints::alluxio::AlluxioConfig;
pub use crate::endpoints::gitea::GiteaConfig;
pub use crate::endpoints::presto::PrestoConfig;
pub use crate::endpoints::ranger::RangerConfig;
use crate::AoristConcept;
use aorist_concept::{aorist_concept2, Constrainable, PythonObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
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
