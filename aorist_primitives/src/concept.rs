use crate::endpoints::*;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use siphasher::sip128::{Hasher128, SipHasher};
use std::collections::{BTreeSet, HashMap};
use std::hash::Hasher;
use uuid::Uuid;
use abi_stable::StableAbi;
use serde::{Deserialize, Deserializer, Serialize};
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[repr(C)]
#[cfg(feature = "python")]
#[pyclass]
#[derive(StableAbi, Clone, PartialEq, Serialize, Debug, Hash, Eq, PartialOrd, Ord)]
pub struct AString(abi_stable::std_types::RString);
impl<'de> Deserialize<'de>
    for AString
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let d = abi_stable::std_types::RString::deserialize(deserializer)?;
        Ok(Self(d))
    }
}

impl std::fmt::Display for AString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl From<&str> for AString {
    fn from(s: &str) -> Self {
        AString(s.into())
    }
}
impl std::convert::AsRef<str> for AString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AString {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn contains(&self, c: char) -> bool {
        self.0.contains(c)
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    pub fn new(s: &str) -> Self {
        Self(s.into())
    }
}

pub trait ConceptEnum {}
pub trait AoristConcept {
    type TChildrenEnum: ConceptEnum;
    fn get_uuid(&self) -> Option<Uuid>;
    fn get_tag(&self) -> Option<AString>;
    fn compute_uuids(&self);
    fn get_children_uuid(&self) -> Vec<Uuid>;
    fn get_uuid_from_children_uuid(&self) -> Uuid {
        let child_uuids = self.get_children_uuid();
        if child_uuids.len() > 0 {
            let uuids = child_uuids.into_iter().collect::<BTreeSet<Uuid>>();
            let mut hasher = SipHasher::new();
            for uuid in uuids {
                hasher.write(uuid.as_bytes());
            }
            let bytes: [u8; 16] = hasher.finish128().as_bytes();
            Uuid::from_bytes(bytes)
        } else {
            // TODO: this should just be created from the hash
            Uuid::new_v4()
        }
    }
    fn get_children(
        &self,
    ) -> Vec<(
        // struct name
        &str,
        // field name
        Option<&str>,
        // ix
        Option<usize>,
        // uuid
        Option<Uuid>,
        // wrapped reference
        Self::TChildrenEnum,
    )>;
}

pub trait TConceptEnum: Sized + Clone {
    type TUniverse: AoristConcept + AoristUniverse;
    fn get_parent_id(&self) -> Option<(Uuid, AString)>;
    fn get_type(&self) -> AString;
    fn get_uuid(&self) -> Uuid;
    fn get_tag(&self) -> Option<AString>;
    fn get_index_as_child(&self) -> usize;
    fn get_child_concepts(&self) -> Vec<Self>;
    fn populate_child_concept_map(&self, concept_map: &mut HashMap<(Uuid, AString), Self>);
    fn from_universe(universe: Self::TUniverse) -> Self;
}

pub trait AoristUniverse {
    type TEndpoints: Clone;
    fn get_endpoints(&self) -> Self::TEndpoints;
}
pub trait TPrestoEndpoints {
    fn presto_config(&self) -> PrestoConfig;
}
pub trait Ancestry {
    type TConcept: ConceptEnum + Clone + TConceptEnum;
    fn new(parents: RArc<RRwLock<HashMap<(Uuid, AString), Self::TConcept>>>) -> Self;
    fn get_parents(&self) -> RArc<RRwLock<HashMap<(Uuid, AString), Self::TConcept>>>;
}
pub trait TAoristObject {
    fn get_name(&self) -> &AString;
}
