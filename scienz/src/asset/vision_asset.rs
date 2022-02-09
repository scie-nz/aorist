use crate::asset::*;

use crate::encoding::Encoding;
use crate::schema::*;
use crate::storage::Storage;
use crate::storage_setup::*;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{asset_enum, AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::{AString, AVec, AoristRef};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

asset_enum! {
    name: VisionAsset
    variants:
    - ImageCorpus
    - ImageCorpusKeyPoints
    - ImageCorporaKNNMatch
    - MLPAsset
    - ImageCorporaHomography
}
