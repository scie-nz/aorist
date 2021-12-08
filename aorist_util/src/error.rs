use abi_stable::library::LibraryError;
use thiserror::Error;

pub type AResult<T> = Result<T, AoristError>;

#[repr(u8)]
#[derive(Error, Debug)]
pub enum AoristError {
    #[error("{0}")]
    OtherError(String),
    #[error("LibraryError")]
    LibraryLoadError(#[from] LibraryError),
    #[error("syn::Error")]
    SynError(#[from] syn::Error),
    #[error("std::io::Error")]
    IOError(#[from] std::io::Error),
}
impl AoristError {
    pub fn as_str(&self) -> String {
        let res = match self {
            Self::OtherError(e) => e.clone(),
            Self::LibraryLoadError(e) => format!("{:?}", e),
            Self::SynError(e) => format!("{:?}", e),
            Self::IOError(e) => format!("{:?}", e),
        };
        res
    }
}
