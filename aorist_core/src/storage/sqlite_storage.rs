use crate::concept::{AoristRef, WrappedConcept};
use crate::layout::*;
use crate::location::*;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct SQLiteStorage {
    #[constrainable]
    pub location: AoristRef<SQLiteLocation>,
    #[constrainable]
    layout: AoristRef<TabularLayout>,
}
