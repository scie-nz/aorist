pub mod alluxio;
pub mod gitea;
pub mod presto;
pub mod ranger;

use crate::constraint::Constraint;
pub use crate::endpoints::alluxio::AlluxioConfig;
pub use crate::endpoints::gitea::GiteaConfig;
pub use crate::endpoints::presto::PrestoConfig;
pub use crate::endpoints::ranger::RangerConfig;
use crate::utils::GetSetError;
use getset::{IncompleteGetters, IncompleteSetters};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[pyclass]
#[serde()]
#[derive(Serialize, Deserialize, Clone, IncompleteGetters, IncompleteSetters)]
pub struct EndpointConfig {
    #[getset(get_incomplete = "pub", set_incomplete = "pub")]
    presto: Option<PrestoConfig>,
    #[getset(get_incomplete = "pub", set_incomplete = "pub")]
    alluxio: Option<AlluxioConfig>,
    #[getset(get_incomplete = "pub", set_incomplete = "pub")]
    ranger: Option<RangerConfig>,
    #[getset(get_incomplete = "pub", set_incomplete = "pub")]
    gitea: Option<GiteaConfig>,
    uuid: Option<Uuid>,
    #[serde(skip)]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}

#[pymethods]
impl EndpointConfig {
    #[new]
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
            uuid: None,
            constraints: Vec::new(),
        }
    }
}
