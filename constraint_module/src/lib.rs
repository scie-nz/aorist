use aorist_core::{ConstraintMod, ConstraintMod_Ref};
use aorist_core::AoristError;
use abi_stable::{
    export_root_module, sabi_extern_fn, std_types::{RString, ROk, RResult, RVec},
    prefix_type::PrefixTypeTrait,
};

#[export_root_module]
fn instantiate_root_module() -> ConstraintMod_Ref {
    ConstraintMod { new, builders }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn new() -> RResult<RString, AoristError> {
    ROk("hello".into())
}

/*#[sabi_extern_fn]
pub fn builders() -> RResult<RVec<RString>, AoristError> {
    ROk(vec!["hello".into()].into())
}*/
include!(concat!(env!("OUT_DIR"), "/constraints.rs"));
