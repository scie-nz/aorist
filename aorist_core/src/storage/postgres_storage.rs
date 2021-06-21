use crate::layout::*;
use crate::location::*;
use crate::concept::{AoristRef, WrappedConcept};
use aorist_primitives::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct PostgresStorage {
    #[constrainable]
    pub location: AoristRef<PostgresLocation>,
    #[constrainable]
    layout: AoristRef<TabularLayout>,
}
