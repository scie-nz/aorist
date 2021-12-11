use crate::endpoints::*;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::{RArc, RVec};
use abi_stable::StableAbi;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};
use siphasher::sip128::{Hasher128, SipHasher};
use std::collections::{BTreeSet, HashMap};
use std::hash::Hasher;
use uuid::Uuid;

#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(StableAbi, Clone, PartialEq, Serialize, Debug, Hash, Eq, PartialOrd, Ord)]
pub struct AString(abi_stable::std_types::RString);

impl<'de> Deserialize<'de> for AString {
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

#[repr(C)]
#[derive(Clone, PartialEq, Serialize, Debug, Hash, Eq, PartialOrd, Ord)]
pub struct AVec<T>(abi_stable::std_types::RVec<T>);

impl<'de, T> Deserialize<'de> for AVec<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let d = abi_stable::std_types::RVec::<T>::deserialize(deserializer)?;
        Ok(Self(d))
    }
}

impl<T> std::iter::IntoIterator for AVec<T> {
    type Item = T;
    type IntoIter = <abi_stable::std_types::RVec<T> as std::iter::IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<T> std::iter::FromIterator<T> for AVec<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(abi_stable::std_types::RVec::<T>::from_iter(iter))
    }
}
impl<T> std::ops::Deref for AVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.0.deref()
    }
}

impl<T> AVec<T> {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, T> {
        self.0.iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, T> {
        self.0.iter_mut()
    }
    pub fn new() -> AVec<T> {
        Self(RVec::<T>::new())
    }
    pub fn push(&mut self, elem: T) {
        self.0.push(elem)
    }
    pub fn insert(&mut self, index: usize, value: T) {
        self.0.insert(index, value)
    }
}
impl AVec<String> {
    pub fn join(&self, separator: &str) -> String {
        self.0.join(separator)
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, Serialize, Debug, Hash, Eq, PartialOrd, Ord)]
pub struct AOption<T>(pub abi_stable::std_types::ROption<T>);
impl <T> AOption<T> {
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
}
pub trait ConceptEnum {}
pub trait AoristConcept {
    type TChildrenEnum: ConceptEnum;
    fn get_uuid(&self) -> Option<Uuid>;
    fn get_tag(&self) -> Option<AString>;
    fn compute_uuids(&self);
    fn get_children_uuid(&self) -> AVec<Uuid>;
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
    ) -> AVec<(
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
    fn get_child_concepts(&self) -> AVec<Self>;
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
