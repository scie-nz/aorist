use crate::concept::{AoristRef, WrappedConcept};
use crate::layout::*;
use crate::location::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum, AString};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct BigQueryStorage {
    #[constrainable]
    pub location: AoristRef<BigQueryLocation>,
    #[constrainable]
    layout: AoristRef<TabularLayout>,
}
