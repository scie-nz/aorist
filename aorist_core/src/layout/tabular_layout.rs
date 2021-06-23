use crate::concept::{AoristRef, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct StaticTabularLayout {}

#[aorist]
pub struct DailyGranularity {}

#[aorist]
pub enum Granularity {
    #[constrainable]
    DailyGranularity(AoristRef<DailyGranularity>),
}

#[aorist]
pub struct DynamicTabularLayout {
    #[constrainable]
    granularity: AoristRef<Granularity>,
}

#[aorist]
pub enum TabularLayout {
    StaticTabularLayout(AoristRef<StaticTabularLayout>),
    DynamicTabularLayout(AoristRef<DynamicTabularLayout>),
}
