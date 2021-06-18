use crate::layout::*;
use crate::location::*;
use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct SQLiteStorage {
    #[constrainable]
    pub location: SQLiteLocation,
    #[constrainable]
    layout: TabularLayout,
}
