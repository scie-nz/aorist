use abi_stable::StableAbi;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug};
use crate::concept::{AString, AVec};
use crate::dialect::Dialect;

#[repr(C)]
#[derive(Clone, Serialize, Debug, StableAbi, PartialEq, Eq, Deserialize)]
pub struct Function {
    arguments: AVec<AString>,
    body: AVec<AString>,
}

#[repr(C)]
#[derive(Clone, Serialize, Debug, StableAbi, PartialEq, Eq, Deserialize)]
pub struct NamedFunction {
    name: AString,
    function: Function,
}

#[repr(C)]
#[derive(Clone, Serialize, Debug, StableAbi, PartialEq, Eq, Deserialize)]
pub struct AProgram {
    dialect: Dialect,
    code: AString,
    entrypoint: AString,
    arg_functions: AVec<Function>,
    kwarg_functions: AVec<NamedFunction>,
}
