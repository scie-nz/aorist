use crate::layout::*;
use crate::location::*;
use crate::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
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
