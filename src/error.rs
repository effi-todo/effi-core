use thiserror::Error;

#[derive(Error, Debug)]
pub enum EffiCoreError {
    #[error("id '{0}' is already taken")]
    IdAlreadyTaken(String),

    #[error("could not convert '{0}' to an id: {1}")]
    InvalidStringForId(String, String),
}
