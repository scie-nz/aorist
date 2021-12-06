use crate::concept::{AoristRef, WrappedConcept};
use crate::encoding::*;
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
pub struct HiveTableStorage {
    #[constrainable]
    pub location: AoristRef<HiveLocation>,
    #[constrainable]
    layout: AoristRef<TabularLayout>,
    #[constrainable]
    pub encoding: AoristRef<Encoding>,
}
