use thiserror::Error;

#[derive(Error, Debug)]
pub enum AoristError {
    #[error("error from gitea: {0:#?}")]
    GiteaError(#[from] gitea::Error),
    #[error("error from ranger: {0:#?}")]
    RangerError(#[from] ranger::RangerError),
    #[error("{0}")]
    OtherError(String),
}
