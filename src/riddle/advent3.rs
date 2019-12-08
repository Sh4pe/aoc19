use std::str;
use std::num::{ParseIntError};
use std::convert::{From};


#[derive(Debug, Eq, PartialEq)]
pub enum Segment {
    R(usize),
    L(usize),
    U(usize),
    D(usize),
}

impl Segment {
    fn len(&self) -> usize {
        match self {
            Segment::U(n) => *n,
            Segment::D(n) => *n,
            Segment::L(n) => *n,
            Segment::R(n) => *n,
        }
    }
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

#[derive(PartialEq, Eq, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64
}

impl Point {
    pub fn manhattan_norm(&self) -> usize {
        (self.x.abs() as usize) + (self.y.abs() as usize)
    }

    pub fn points_in_seqment(&self, segment: Segment) -> Vec<Point> {
        let (x_increment, y_increment) : (i64, i64) = match segment {
            Segment::U(_) => (0,  1),
            Segment::D(_) => (0, -1),
            Segment::L(_) => (-1, 0),
            Segment::R(_) => (1,  0),
        };

        let num_steps = segment.len() + 1;
        (1..num_steps)
            .map( |i| {
                let i = i as i64;
                Point{ x: self.x + i*x_increment, y: self.y + i*y_increment } 
            })
            .collect()
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

    mod point_tests {
        use super::super::{Point, Segment};

        #[test]
        fn the_norm_works_as_expected() {
            let p = Point{ x: -3, y: 2 };
            assert_eq!(p.manhattan_norm(), 5);
        }

        #[test]
        fn points_in_seqment_works_as_expected() {
            let test_cases : Vec<((i64, i64), _, Vec<(i64, i64)>)> = vec![
                ((3,2),  Segment::R(4), vec![(4,2), (5,2), (6,2), (7,2)]),
                ((1,-1), Segment::U(2), vec![(1,0), (1,1)]),
                ((0, 0), Segment::L(3), vec![(-1,0), (-2,0), (-3,0)]),
                ((4, 4), Segment::D(4), vec![(4,3), (4,2), (4,1), (4,0)]),
            ];

            for (origin, segment, expected) in test_cases {
                let origin = Point{ x: origin.0, y: origin.1 };
                let expected : Vec<Point> = expected
                    .iter()
                    .map( |p| Point{ x: p.0, y: p.1} )
                    .collect();
                let points_in_segment = origin.points_in_seqment(segment);

                assert_eq!(points_in_segment, expected);
            }
        }
    }

}