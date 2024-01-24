use std::{io, result};
use std::fmt::{Display, Formatter};
use std::io::Error;
use thiserror::Error;

pub type Result<T> = result::Result<T, KanagawaError>;

#[derive(Error, Debug)]
pub enum KanagawaError {
    #[error("KanagawaError::IoError: {0}")]
    IoError(#[from] io::Error)
}