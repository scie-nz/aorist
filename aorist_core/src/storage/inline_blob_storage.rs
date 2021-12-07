use aorist_primitives::AVec;
use crate::concept::{AoristRef, WrappedConcept};
use crate::encoding::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct InlineBlobStorage {
    #[constrainable]
    pub encoding: AoristRef<Encoding>,
    pub data: AVec<AString>,
}
