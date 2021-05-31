use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::location::bigquery_location::*;
use crate::location::gcs_location::*;
use crate::location::github_location::*;
use crate::location::pushshift_api_location::*;
use crate::location::web_location::*;

#[aorist]
pub enum RemoteLocation {
    #[constrainable]
    GCSLocation(GCSLocation),
    #[constrainable]
    WebLocation(WebLocation),
    #[constrainable]
    PushshiftAPILocation(PushshiftAPILocation),
    #[constrainable]
    BigQueryLocation(BigQueryLocation),
    #[constrainable]
    GithubLocation(GithubLocation),
}
