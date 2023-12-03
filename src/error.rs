use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid Format.\n  Expected [{0}]\n  Detected [{1}]")]
    InvalidFormat(String, String),
    #[error("Unknown Section: [{0}]")]
    UnknownSection(String),
    #[error("Could not find file: [{0}]")]
    MissingFile(String),
    #[error("Missing permissions to open file: [{0}]")]
    FileLocked(String),
    #[error("Failed in an unexpeceted location. Please report the stack.")]
    Unexpected,
    #[error("{0}")]
    IniParseError(String),
}


