use crate::endpoints::*;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::{RArc, ROption, RVec};
use abi_stable::StableAbi;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use siphasher::sip128::{Hasher128, SipHasher};
use std::collections::{BTreeSet, HashMap};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::convert::TryInto;

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
        std::fmt::Display::fmt(&self.0, f)
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
    pub fn into_bytes(self) -> RVec<u8> {
        self.0.into_bytes()
    }
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
#[derive(Clone, PartialEq, Serialize, Debug, Hash, Eq, PartialOrd, Ord, StableAbi)]
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
#[derive(Clone, PartialEq, Serialize, Debug, Hash, Eq, PartialOrd, Ord, StableAbi)]
pub struct AOption<T>(pub abi_stable::std_types::ROption<T>);
impl<T> AOption<T> {
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }
    pub fn and_then<F, U>(self, f: F) -> AOption<U>
    where
        F: FnOnce(T) -> ROption<U>,
    {
        let out: ROption<U> = self.0.and_then(f);
        AOption(out)
    }
    pub fn as_ref(&self) -> AOption<&T> {
        AOption(self.0.as_ref())
    }
    pub fn unwrap(self) -> T {
        self.0.unwrap()
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for AOption<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let option = ROption::deserialize(deserializer)?;
        Ok(Self(option))
    }
}

#[repr(C)]
pub struct AMapNode<T: Clone + PartialEq + Eq + PartialOrd + Ord> {
    next: *mut AMapNode<T>,
    prev: *mut AMapNode<T>,
    key: AString,
    value: T,
}

#[repr(C)]
pub struct AMap<
    T: Clone + PartialEq + Eq + PartialOrd + Ord,
    S = std::collections::hash_map::RandomState,
> {
    map: abi_stable::std_types::RHashMap<AString, T, S>,
    head: *mut AMapNode<T>,
    free: *mut AMapNode<T>,
}

pub trait ConceptEnum {
    fn uuid(&self) -> AOption<AUuid>;
}

pub trait AoristConceptBase: Clone + Debug + Serialize + PartialEq + StableAbi {
    type TChildrenEnum: ConceptEnum;
    fn get_uuid(&self) -> AOption<AUuid>;
    fn set_uuid(&mut self, uuid: AUuid);
    fn get_tag(&self) -> AOption<AString>;
    fn compute_uuids(&mut self);
    fn deep_clone(&self) -> Self;
    fn get_children(
        &self,
    ) -> AVec<(
        // struct name
        AString,
        // field name
        AOption<AString>,
        // ix
        AOption<usize>,
        // uuid
        AOption<AUuid>,
        // wrapped reference
        Self::TChildrenEnum,
    )>;
    #[cfg(feature = "python")]
    fn py_object(inner: AoristRef<Self>, py: pyo3::Python) -> pyo3::PyResult<pyo3::PyObject>;
}

pub trait AoristConcept {
    type TChildrenEnum: ConceptEnum;
    fn get_uuid(&self) -> AOption<AUuid>;
    fn set_uuid(&mut self, uuid: AUuid);
    fn get_tag(&self) -> AOption<AString>;
    fn compute_uuids(&mut self);
    fn get_children_uuid(&self) -> AVec<AUuid>;
    fn get_uuid_from_children_uuid(&self) -> AUuid {
        let child_uuids = self.get_children_uuid();
        if child_uuids.len() > 0 {
            let uuids = child_uuids.into_iter().collect::<BTreeSet<AUuid>>();
            let mut hasher = SipHasher::new();
            for uuid in uuids {
                hasher.write(&uuid.as_bytes());
            }
            let bytes: [u8; 16] = hasher.finish128().as_bytes();
            AUuid::from_bytes(bytes)
        } else {
            // TODO: this should just be created from the hash
            AUuid::new_v4()
        }
    }
    fn get_children(
        &self,
    ) -> AVec<(
        // struct name
        AString,
        // field name
        AOption<AString>,
        // ix
        AOption<usize>,
        // uuid
        AOption<AUuid>,
        // wrapped reference
        Self::TChildrenEnum,
    )>;
    fn deep_clone(&self) -> Self;
    #[cfg(feature = "python")]
    fn py_object(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::PyObject>;
}

pub trait ToplineConcept: Sized + Clone + StableAbi {
    type TUniverse: AoristConcept + AoristUniverse;
    fn get_parent_id(&self) -> AOption<(AUuid, AString)>;
    fn get_type(&self) -> AString;
    fn get_uuid(&self) -> AUuid;
    fn get_tag(&self) -> AOption<AString>;
    fn get_index_as_child(&self) -> usize;
    fn get_child_concepts(&self) -> AVec<Self>;
    fn populate_child_concept_map(&self, concept_map: &mut HashMap<(AUuid, AString), Self>);
    fn from_universe(universe: Self::TUniverse) -> Self;
}
pub trait ToplineConceptBase: Sized + Clone + Debug + Serialize + PartialEq + StableAbi {
    type TUniverse: AoristConcept + AoristUniverse;
    fn get_parent_id(&self) -> AOption<(AUuid, AString)>;
    fn get_type(&self) -> AString;
    fn get_index_as_child(&self) -> usize;
    fn get_child_concepts(&self) -> AVec<AoristRef<Self>>;
    fn populate_child_concept_map(
        &self,
        concept_map: &mut HashMap<(AUuid, AString), AoristRef<Self>>,
    );
    fn build_universe(universe: Self::TUniverse) -> Self;
}

pub trait AoristUniverse {
    type TEndpoints: Clone;
    fn get_endpoints(&self) -> Self::TEndpoints;
}
pub trait TPrestoEndpoints {
    fn presto_config(&self) -> PrestoConfig;
}
pub trait Ancestry {
    type TConcept: ConceptEnum + Clone + ToplineConcept;
    fn new(parents: RArc<RRwLock<HashMap<(AUuid, AString), Self::TConcept>>>) -> Self;
    fn get_parents(&self) -> RArc<RRwLock<HashMap<(AUuid, AString), Self::TConcept>>>;
}
pub trait TAoristObject {
    fn get_name(&self) -> &AString;
}

#[repr(C)]
#[derive(StableAbi)]
pub struct AoristRef<T: PartialEq + Serialize + Debug + Clone + StableAbi>(pub RArc<RRwLock<T>>);

#[repr(C)]
#[derive(StableAbi, PartialEq, Ord, Eq, PartialOrd, Hash, Debug, Clone, Serialize)]
pub struct AUuid(AString);
impl AUuid {
    pub fn new_v4() -> Self {
        let uuid = uuid::Uuid::new_v4();
        Self(AString::new(&uuid.to_string()))
    }
    pub fn from_bytes(bytes: uuid::Bytes) -> AUuid {
        let uuid = uuid::Uuid::from_bytes(bytes);
        Self(AString::new(&uuid.to_string()))
    }
    pub fn as_bytes(&self) -> uuid::Bytes {
        self.0.clone().into_bytes()
			.into_vec()
            .try_into()
        	.unwrap_or_else(|v: Vec<u8>| panic!("Expected a Vec of length 16 but it was {}", v.len()))
    }

}
impl<'de> Deserialize<'de> for AUuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let d = AString::deserialize(deserializer)?;
        Ok(Self(d))
    }
}

impl std::fmt::Display for AUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl<T: PartialEq + Serialize + Debug + Clone + AoristConceptBase + StableAbi> AoristConcept for AoristRef<T> {
    type TChildrenEnum = <T as AoristConceptBase>::TChildrenEnum;
    fn get_uuid(&self) -> AOption<AUuid> {
        self.0.read().get_uuid()
    }
    fn deep_clone(&self) -> Self {
        AoristRef(RArc::new(RRwLock::new(self.0.read().deep_clone())))
    }
    fn set_uuid(&mut self, uuid: AUuid) {
        self.0.write().set_uuid(uuid);
    }
    fn compute_uuids(&mut self) {
        self.0.write().compute_uuids();
        let uuid;
        uuid = self.get_uuid_from_children_uuid();
        self.0.write().set_uuid(uuid);
    }
    fn get_children_uuid(&self) -> AVec<AUuid> {
        self.get_children()
            .iter()
            .map(|x| x.4.uuid().unwrap())
            .collect()
    }
    fn get_tag(&self) -> AOption<AString> {
        self.0.read().get_tag()
    }
    fn get_children(
        &self,
    ) -> AVec<(
        // struct name
        AString,
        // field name
        AOption<AString>,
        // ix
        AOption<usize>,
        // uuid
        AOption<AUuid>,
        Self::TChildrenEnum,
    )> {
        self.0.read().get_children()
    }
    #[cfg(feature = "python")]
    fn py_object(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::PyObject> {
        T::py_object(self.clone(), py)
    }
}

#[cfg(feature = "python")]
impl<'a, T: PartialEq + Serialize + Debug + Clone + FromPyObject<'a> + StableAbi> FromPyObject<'a>
    for AoristRef<T>
{
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        Ok(AoristRef(RArc::new(RRwLock::new(T::extract(ob)?))))
    }
}

impl<T: PartialEq + Eq + Serialize + Debug + Clone + StableAbi> PartialEq for AoristRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.read().eq(&other.0.read())
    }
}
impl<T: PartialEq + Eq + Serialize + Debug + Clone + StableAbi> Eq for AoristRef<T> {}
impl<T: PartialEq + Serialize + Debug + Clone + StableAbi> Serialize for AoristRef<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.read().serialize(serializer)
    }
}
impl<'de, T: Deserialize<'de> + PartialEq + Serialize + Debug + Clone + StableAbi> Deserialize<'de>
    for AoristRef<T>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let d = T::deserialize(deserializer)?;
        Ok(Self(RArc::new(RRwLock::new(d))))
    }
}
impl<T: Clone + Debug + Serialize + PartialEq + StableAbi> Clone for AoristRef<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T: Debug + Clone + Serialize + PartialEq + StableAbi> Debug for AoristRef<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.0.read().fmt(f)
    }
}
impl<T: Clone + Debug + Serialize + PartialEq + AoristConceptBase + StableAbi> ConceptEnum for AoristRef<T> {
    fn uuid(&self) -> AOption<AUuid> {
        self.0.read().get_uuid()
    }
}

impl<T: Debug + Clone + Serialize + PartialEq + StableAbi> Hash for AoristRef<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.read().hash(state);
    }
}
impl<T: Debug + Clone + Serialize + PartialEq + ToplineConceptBase + AoristConceptBase + StableAbi>
    ToplineConcept for AoristRef<T>
{
    type TUniverse = <T as ToplineConceptBase>::TUniverse;
    fn get_parent_id(&self) -> AOption<(AUuid, AString)> {
        self.0.read().get_parent_id()
    }
    fn from_universe(universe: Self::TUniverse) -> Self {
        AoristRef(RArc::new(RRwLock::new(T::build_universe(universe))))
    }
    fn get_type(&self) -> AString {
        self.0.read().get_type()
    }
    fn get_uuid(&self) -> AUuid {
        self.0.read().get_uuid().unwrap()
    }
    fn get_tag(&self) -> AOption<AString> {
        self.0.read().get_tag()
    }
    fn get_index_as_child(&self) -> usize {
        self.0.read().get_index_as_child()
    }
    fn get_child_concepts(&self) -> AVec<Self> {
        self.0.read().get_child_concepts()
    }
    fn populate_child_concept_map(&self, concept_map: &mut HashMap<(AUuid, AString), Self>) {
        self.0.read().populate_child_concept_map(concept_map)
    }
}
// note: both Universe and EndpointConfig must exist
impl<T: Debug + Clone + Serialize + PartialEq + AoristUniverseBase + StableAbi> AoristUniverse
    for AoristRef<T>
{
    type TEndpoints = <T as AoristUniverseBase>::TEndpoints;
    fn get_endpoints(&self) -> Self::TEndpoints {
        (*self.0.read()).get_endpoints()
    }
}
pub trait AoristUniverseBase {
    type TEndpoints: Clone;
    fn get_endpoints(&self) -> Self::TEndpoints;
}
