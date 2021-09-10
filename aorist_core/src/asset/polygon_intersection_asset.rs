#![allow(non_snake_case)]
use crate::asset::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::schema::*;
use crate::storage::*;
use crate::encoding::*;
use crate::storage_setup::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum, asset};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

asset!{ PolygonIntersectionAsset }
