use thiserror::Error;
use abi_stable::{std_types::RString, StableAbi};

#[repr(u8)]
#[derive(Error, Debug, StableAbi)]
pub enum AoristError {
    #[error("{0}")]
    OtherError(RString),
}
