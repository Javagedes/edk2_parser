mod parser;
pub mod error;
mod section;
mod condition;
mod config;

use crate::parser::{ConfigParser, Edk2Section, ParseFn};

pub type InfParser = ConfigParser<config::Inf>;
pub type DscParser = ConfigParser<config::Dsc>;
