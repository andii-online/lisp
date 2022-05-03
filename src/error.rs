use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use super::Lval;

pub type Result<T> = std::result::Result<T, RisprError>;
pub type LvalResult = Result<Lval>;

#[derive(Debug, Clone)]
pub enum RisprError {
    ReadLineError(String),
    ParseError(String),
}

impl fmt::Display for RisprError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RisprError::ParseError(err) => write!(f, "{}", err),
            RisprError::ReadLineError(err) => write!(f, "{}", err),
        }
    }
}

impl<T> From<pest::error::Error<T>> for RisprError
where
    T: Debug + Ord + Copy + Hash,
{
    fn from(error: pest::error::Error<T>) -> Self {
        RisprError::ParseError(format!("{}", error))
    }
}
