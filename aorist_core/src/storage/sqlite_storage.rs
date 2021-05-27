use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::layout::*;
use crate::location::*;

#[aorist]
pub struct SQLiteStorage {
    #[constrainable]
    pub location: SQLiteLocation,
    #[constrainable]
    layout: TabularLayout,
}
