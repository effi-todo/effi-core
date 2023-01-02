use thiserror::Error;

#[derive(Error, Debug)]
pub enum EffiCoreError {
    #[error("id '{0}' is already taken")]
    IdAlreadyTaken(String),
}
