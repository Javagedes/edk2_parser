use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid Format. Expected [{0}]")]
    InvalidFormat(String),
    #[error("Unknown Section: [{0}]")]
    UnknownSection(String),
}
