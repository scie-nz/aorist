#![allow(non_snake_case)]

use crate::concept::{AoristConcept, AoristConceptChildren, Concept};
use crate::constraint::*;
use crate::encoding::*;
use crate::layout::*;
use crate::location::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct HiveTableStorage {
    #[constrainable]
    pub location: HiveLocation,
    #[constrainable]
    layout: TabularLayout,
    #[constrainable]
    pub encoding: Encoding,
}
