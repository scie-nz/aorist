use thiserror::Error;

#[derive(Error, Debug)]
pub enum AoristError {
    #[error("{0}")]
    OtherError(String),
}
