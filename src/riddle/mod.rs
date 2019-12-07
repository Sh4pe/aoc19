use std::io::{ErrorKind};
use std::error::{Error};
use std::fmt;

pub mod advent1;
pub mod riddlecontainer;

#[derive(Debug)]
pub enum Solution {
    Number(i64)
}

#[derive(Debug)]
pub enum RiddleError {
    UnknownRiddle(String),
    IO(std::io::Error),
    ArgumentsCount(u8, u8),
}

impl fmt::Display for RiddleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        
    }
}

impl Error for RiddleError {}

impl std::convert::From<RiddleError> for std::io::Error {
    fn from(err: RiddleError) -> Self { std::io::Error::new(ErrorKind::Other, err) }
}

pub trait Riddle {
    fn solve(&self, args: &[String]) -> Result<Solution, RiddleError>;
}
