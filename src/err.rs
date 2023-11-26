use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError<'a> {
    #[error("Invalid Format. Expected [{0}]")]
    InvalidFormat(&'a str),
}
