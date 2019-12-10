use std::error::Error;
use std::fmt;
use std::io::ErrorKind;

pub mod advent1;
pub mod advent2;
pub mod advent3;
pub mod riddlecontainer;

#[derive(Debug, PartialEq, Eq)]
pub enum Solution {
    Number(i64),
}

#[derive(Debug)]
pub enum RiddleError {
    UnknownRiddle(String),
    IO(std::io::Error),
    ArgumentsCount(u8, u8),
    Generic(String),
    NoSolutionFound,
}

impl fmt::Display for RiddleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RiddleError {}

impl std::convert::From<RiddleError> for std::io::Error {
    fn from(err: RiddleError) -> Self {
        std::io::Error::new(ErrorKind::Other, err)
    }
}

impl std::convert::From<std::io::Error> for RiddleError {
    fn from(err: std::io::Error) -> Self {
        RiddleError::IO(err)
    }
}

pub trait Riddle {
    fn solve(&self, args: &[String]) -> Result<Solution, RiddleError>;
}
