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
use aorist_concept::{aorist_concept, Constrainable};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct EndpointConfig {
    presto: Option<PrestoConfig>,
    alluxio: Option<AlluxioConfig>,
    ranger: Option<RangerConfig>,
    gitea: Option<GiteaConfig>,
}

#[pymethods]
impl EndpointConfig {
    #[new]
    #[args(presto = "None", alluxio = "None", ranger = "None", gitea = "None")]
    fn new(
        presto: Option<PrestoConfig>,
        alluxio: Option<AlluxioConfig>,
        ranger: Option<RangerConfig>,
        gitea: Option<GiteaConfig>,
    ) -> Self {
        Self {
            presto,
            alluxio,
            ranger,
            gitea,
            tag: None,
            uuid: None,
            constraints: Vec::new(),
        }
    }
}
