use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
