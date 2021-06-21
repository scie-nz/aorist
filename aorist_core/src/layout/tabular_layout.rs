use crate::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
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
    granularity: Granularity,
}

#[aorist]
pub enum TabularLayout {
    StaticTabularLayout(AoristRef<StaticTabularLayout>),
    DynamicTabularLayout(AoristRef<DynamicTabularLayout>),
}
