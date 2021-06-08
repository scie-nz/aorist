use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[aorist]
pub struct StaticTabularLayout {}

#[aorist]
pub struct DailyGranularity {}

#[aorist]
pub enum Granularity {
    #[constrainable]
    DailyGranularity(DailyGranularity),
}

#[aorist]
pub struct DynamicTabularLayout {
    #[constrainable]
    granularity: Granularity,
}

#[aorist]
pub enum TabularLayout {
    StaticTabularLayout(StaticTabularLayout),
    DynamicTabularLayout(DynamicTabularLayout),
}
