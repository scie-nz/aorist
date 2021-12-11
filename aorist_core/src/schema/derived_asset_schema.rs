use crate::asset::*;
use crate::concept::AoristRef;
use aorist_primitives::AVec;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait DerivedAssetSchema<'a> {
    type SourceAssetType: TAsset + Debug + PartialEq + Clone + Eq + Serialize + Deserialize<'a>;
}
pub trait SingleSourceDerivedAssetSchema<'a>: DerivedAssetSchema<'a> {
    fn get_source(&self) -> AoristRef<<Self as DerivedAssetSchema<'a>>::SourceAssetType>;
}
pub trait MultipleSourceDerivedAssetSchema<'a>: DerivedAssetSchema<'a> {
    fn get_sources(&self) -> AVec<Asset>;
}
