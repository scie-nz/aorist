pub mod alluxio;
pub mod gitea;
pub mod presto;
pub mod ranger;

use crate::concept::AoristConcept;
use crate::endpoints::alluxio::AlluxioConfig;
use crate::endpoints::gitea::GiteaConfig;
use crate::endpoints::presto::PrestoConfig;
use crate::endpoints::ranger::RangerConfig;
use crate::utils::GetSetError;
use getset::{IncompleteGetters, IncompleteSetters};
use serde::{Deserialize, Serialize};
use aorist_concept::Constrainable;

#[serde()]
#[derive(Constrainable, Serialize, Deserialize, Clone, IncompleteGetters, IncompleteSetters)]
pub struct EndpointConfig {
    #[getset(get_incomplete = "pub", set_incomplete = "pub")]
    presto: Option<PrestoConfig>,
    #[getset(get_incomplete = "pub", set_incomplete = "pub")]
    alluxio: Option<AlluxioConfig>,
    #[getset(get_incomplete = "pub", set_incomplete = "pub")]
    ranger: Option<RangerConfig>,
    #[getset(get_incomplete = "pub", set_incomplete = "pub")]
    gitea: Option<GiteaConfig>,
}
