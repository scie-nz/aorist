use crate::asset::*;
use crate::concept::AoristRef;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait DerivedAssetSchema<'a> {
    type SourceAssetType: TAsset + Debug + PartialEq + Clone + Eq + Serialize + Deserialize<'a>;
    fn get_source(&self) -> AoristRef<Self::SourceAssetType>;
}
