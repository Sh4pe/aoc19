use std::str;
use std::num::{ParseIntError};
use std::convert::{From};

#[derive(Debug, Eq, PartialEq)]
enum Segment {
    R(usize),
    L(usize),
    U(usize),
    D(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SegmentParseError {
    TooShort(String),
    UnknownDirectoion(String),
    LengthParseError(ParseIntError),
    LengthZero(String),
}

impl From<ParseIntError> for SegmentParseError {
    fn from(err: ParseIntError) -> SegmentParseError { 
        SegmentParseError::LengthParseError(err) 
    }
}

impl str::FromStr for Segment {
    type Err = SegmentParseError;

    fn from_str(s: &str) -> std::result::Result<Self, SegmentParseError> { 
        let s = s.to_string();
        if s.len() < 2 {
            return Err(SegmentParseError::TooShort(s));
        }

        let direction = s.chars().next().unwrap();
        let length = s[1..].parse::<usize>()?;
        if length == 0 {
            return Err(SegmentParseError::LengthZero(s))
        }

        match direction {
            'U' => Ok(Segment::U(length)),
            'D' => Ok(Segment::D(length)),
            'L' => Ok(Segment::L(length)),
            'R' => Ok(Segment::R(length)),
            _ => Err(SegmentParseError::UnknownDirectoion(s))
        }
    }
}

#[cfg(test)]
mod advent3_tests {

    mod segment_tests {

        mod parsing_tests {
            use super::super::super::{Segment, SegmentParseError};

            #[test]
            fn it_complains_when_segments_are_too_short() {
                for s in vec!["", "X"] {
                    let s = String::from(s);
                    assert_eq!(s.parse::<Segment>().unwrap_err(), SegmentParseError::TooShort(s));
                }
            }

            #[test]
            fn it_complains_when_direction_is_unknown() {
                for s in vec!["X10", "r10"] {
                    let s = String::from(s);
                    assert_eq!(s.parse::<Segment>().unwrap_err(), SegmentParseError::UnknownDirectoion(s));
                }
            }

            #[test]
            fn it_complains_when_length_is_not_a_number() -> Result<(), String> {
                for s in vec!["DD", "Rxxx"] {
                    let s = String::from(s);
                    match s.parse::<Segment>().unwrap_err() {
                        SegmentParseError::LengthParseError(_) => {},
                        _ => return Err("expected LengthParseError".to_string())
                    }
                }
                Ok(())
            }

            #[test]
            fn it_complains_when_length_is_non_positive() {
                for s in vec!["R0", "D0"] {
                    let s = String::from(s);
                    assert_eq!(s.parse::<Segment>().unwrap_err(), SegmentParseError::LengthZero(s));
                }
            }

            #[test]
            fn it_works_as_expected() {
                for (s, expected) in vec![("R10", Segment::R(10)), ("U9", Segment::U(9)), ("L1", Segment::L(1)), ] {
                    let s = String::from(s);
                    assert_eq!(s.parse::<Segment>().unwrap(), expected);
                }
            }
        }

    }

}