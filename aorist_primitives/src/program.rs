use crate::endpoints::*;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::{RArc, ROption, RVec};
use abi_stable::StableAbi;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use siphasher::sip128::{Hasher128, SipHasher};
use std::collections::{BTreeSet, HashMap};
use std::convert::TryInto;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use crate::concept::{AoristRef, AOption, AUuid, AString, AVec};
use crate::dialect::Dialect;

#[repr(C)]
#[derive(Clone, Serialize, Debug, StableAbi, PartialEq, Eq)]
pub struct Function {
    arguments: AVec<AString>,
    body: AVec<AString>,
}

#[repr(C)]
#[derive(Clone, Serialize, Debug, StableAbi, PartialEq, Eq)]
pub struct NamedFunction {
    name: AString,
    function: Function,
}

#[repr(C)]
#[derive(Clone, Serialize, Debug, StableAbi, PartialEq, Eq)]
pub struct AProgram {
    dialect: Dialect,
    code: AString,
    entrypoint: AString,
    arg_functions: AVec<Function>,
    kwarg_functions: AVec<NamedFunction>,
}
