use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::{RArc, ROption, RVec};
use abi_stable::StableAbi;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryInto;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

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
        self.0
            .clone()
            .into_bytes()
            .into_vec()
            .try_into()
            .unwrap_or_else(|v: Vec<u8>| {
                panic!("Expected a Vec of length 16 but it was {}", v.len())
            })
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

impl<T: Debug + Clone + Serialize + PartialEq + StableAbi> Hash for AoristRef<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.read().hash(state);
    }
}
