pub mod alluxio;
pub mod gitea;
pub mod presto;
pub mod ranger;

use crate::endpoints::alluxio::AlluxioConfig;
use crate::endpoints::gitea::GiteaConfig;
use crate::endpoints::presto::PrestoConfig;
use crate::endpoints::ranger::RangerConfig;
use crate::data_setup::GetSetError;
use getset::{IncompleteGetters, IncompleteSetters};
use serde::{Deserialize, Serialize};

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
}

