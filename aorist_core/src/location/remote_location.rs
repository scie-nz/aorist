use crate::location::bigquery_location::*;
use crate::location::gcs_location::*;
use crate::location::github_location::*;
use crate::location::pushshift_api_location::*;
use crate::location::web_location::*;
use crate::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum RemoteLocation {
    #[constrainable]
    GCSLocation(AoristRef<GCSLocation>),
    #[constrainable]
    WebLocation(AoristRef<WebLocation>),
    #[constrainable]
    PushshiftAPILocation(AoristRef<PushshiftAPILocation>),
    #[constrainable]
    BigQueryLocation(AoristRef<BigQueryLocation>),
    #[constrainable]
    GithubLocation(AoristRef<GithubLocation>),
}
