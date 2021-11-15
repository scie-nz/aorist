use abi_stable::{library::LibraryError, std_types::RString, StableAbi};
use thiserror::Error;

#[repr(u8)]
#[derive(Error, Debug, StableAbi)]
pub enum AoristError {
    #[error("{0}")]
    OtherError(RString),
}

#[derive(Error, Debug)]
pub enum AoristApplicationError {
    #[error("Library load error")]
    LibraryLoadError(#[from] LibraryError),
}
