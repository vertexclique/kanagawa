use std::{io, result};
use std::fmt::{Display, Formatter};
use std::io::Error;
use thiserror::Error;

pub type Result<T> = result::Result<T, KanagawaError>;

#[derive(Error, Debug)]
pub enum KanagawaError {
    IoError(io::Error)
}

impl From<io::Error> for KanagawaError {
    fn from(value: Error) -> Self {
        KanagawaError::IoError(value)
    }
}

impl Display for KanagawaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KanagawaError")
            .field("error", &format!("{:?}", &self))
            .finish()
    }
}