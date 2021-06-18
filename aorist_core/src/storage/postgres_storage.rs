use crate::layout::*;
use crate::location::*;
use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct PostgresStorage {
    #[constrainable]
    pub location: PostgresLocation,
    #[constrainable]
    layout: TabularLayout,
}
