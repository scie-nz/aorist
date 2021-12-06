use abi_stable::{library::LibraryError, StableAbi};
use thiserror::Error;
use aorist_primitives::AString;
#[repr(u8)]
#[derive(Error, Debug, StableAbi)]
pub enum AoristError {
    #[error("{0}")]
    OtherError(AString),
    #[error("DataSchemaError")]
    LibraryLoadError(AString),
}
impl AoristError {
    pub fn as_str(&self) -> &str {
        match self {
            Self::OtherError(e) => e.as_str(),
            Self::LibraryLoadError(e) => e.as_str(),
        }
    }
}
#[derive(Error, Debug)]
pub enum AoristApplicationError {
    #[error("Library load error")]
    LibraryLoadError(#[from] LibraryError),
}
