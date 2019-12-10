use std::collections::HashSet;
use std::convert::From;
use std::iter::{Extend, FromIterator};
use std::num::ParseIntError;
use std::str;

use super::super::io::lines_from_file;
use super::{Riddle, RiddleError, Solution};

pub struct Advent3Riddle1 {
    input_file: String,
}

impl Advent3Riddle1 {
    pub fn new(input_file: &str) -> Advent3Riddle1 {
        Advent3Riddle1 {
            input_file: input_file.to_string(),
        }
    }
}

impl Riddle for Advent3Riddle1 {
    fn solve(&self, _: &[String]) -> Result<Solution, RiddleError> {
        let lines_result: Result<Vec<_>, _> =
            lines_from_file(&self.input_file)?.map(|x| x).collect();
        let lines = lines_result?;
        assert_eq!(lines.len(), 2);

        let path1 = from_comma_separated_str(&lines[0]).unwrap();
        let path2 = from_comma_separated_str(&lines[1]).unwrap();
        let in_both_parts = points_in_both_paths(&path1, &path2);

        let min_distance = in_both_parts.iter().map(|p| p.manhattan_norm()).min();

        min_distance
            .map(|n| Solution::Number(n as i64))
            .ok_or_else(|| RiddleError::Generic("could not determine min".to_string()))
    }
}

fn points_in_both_paths(path1: &Vec<Segment>, path2: &Vec<Segment>) -> HashSet<Point> {
    let origin = Point { x: 0, y: 0 };
    let points1 = origin.points_in_path(path1);
    let points2 = origin.points_in_path(path2);
    let intersection: HashSet<_> = points1.intersection(&points2).cloned().collect();
    intersection
}

fn from_comma_separated_str(input: &str) -> Result<Vec<Segment>, SegmentParseError> {
    input.split(',').map(|s| s.parse::<Segment>()).collect()
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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
            return Err(SegmentParseError::LengthZero(s));
        }

        match direction {
            'U' => Ok(Segment::U(length)),
            'D' => Ok(Segment::D(length)),
            'L' => Ok(Segment::L(length)),
            'R' => Ok(Segment::R(length)),
            _ => Err(SegmentParseError::UnknownDirectoion(s)),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn manhattan_norm(&self) -> usize {
        (self.x.abs() as usize) + (self.y.abs() as usize)
    }

    pub fn points_in_segment(&self, segment: Segment) -> Vec<Point> {
        let (x_increment, y_increment): (i64, i64) = match segment {
            Segment::U(_) => (0, 1),
            Segment::D(_) => (0, -1),
            Segment::L(_) => (-1, 0),
            Segment::R(_) => (1, 0),
        };

        let num_steps = segment.len() + 1;
        (1..num_steps)
            .map(|i| {
                let i = i as i64;
                Point {
                    x: self.x + i * x_increment,
                    y: self.y + i * y_increment,
                }
            })
            .collect()
    }

    pub fn points_in_path(&self, path: &Vec<Segment>) -> HashSet<Point> {
        if path.is_empty() {
            return HashSet::new();
        }

        let first_segment = self.points_in_segment(path[0]);
        let mut start_point = (*first_segment.last().unwrap()).clone();
        let mut result = HashSet::<Point>::from_iter(first_segment);
        if path.len() > 1 {
            for segment in path.iter().skip(1) {
                let this_segment = start_point.points_in_segment(*segment);
                start_point = (*this_segment.last().unwrap()).clone();
                result.extend(this_segment);
            }
        }
        result
    }
}

#[cfg(test)]
mod advent3_tests {

    mod riddle1_test {
        use super::super::super::{Riddle, Solution};
        use super::super::Advent3Riddle1;

        #[test]
        fn it_works_as_expected() {
            let riddle = Advent3Riddle1::new("./data/input/3.txt");
            let solution = riddle.solve(&vec![]).unwrap();

            assert_eq!(solution, Solution::Number(4981));
        }
    }

    mod from_comma_separated_str_tests {
        use super::super::{from_comma_separated_str, Segment};

        #[test]
        fn it_works_as_expected() {
            let segments = from_comma_separated_str("R5,U3,D8").unwrap();
            let expected = vec![Segment::R(5), Segment::U(3), Segment::D(8)];
            assert_eq!(segments, expected);
        }
    }

    mod points_in_both_paths_tests {
        use super::super::{points_in_both_paths, Point, Segment};
        use std::collections::HashSet;

        #[test]
        fn it_works_as_expected() {
            let path1 = vec![Segment::R(8), Segment::U(5), Segment::L(5), Segment::D(3)];
            let path2 = vec![Segment::U(7), Segment::R(6), Segment::D(4), Segment::L(4)];
            let intersection = points_in_both_paths(&path1, &path2);
            let expected: HashSet<Point> = {
                let mut s = HashSet::new();
                s.insert(Point { x: 3, y: 3 });
                s.insert(Point { x: 6, y: 5 });
                s
            };

            assert_eq!(intersection, expected);
        }
    }

    mod segment_tests {

        mod parsing_tests {
            use super::super::super::{Segment, SegmentParseError};

            #[test]
            fn it_complains_when_segments_are_too_short() {
                for s in vec!["", "X"] {
                    let s = String::from(s);
                    assert_eq!(
                        s.parse::<Segment>().unwrap_err(),
                        SegmentParseError::TooShort(s)
                    );
                }
            }

            #[test]
            fn it_complains_when_direction_is_unknown() {
                for s in vec!["X10", "r10"] {
                    let s = String::from(s);
                    assert_eq!(
                        s.parse::<Segment>().unwrap_err(),
                        SegmentParseError::UnknownDirectoion(s)
                    );
                }
            }

            #[test]
            fn it_complains_when_length_is_not_a_number() -> Result<(), String> {
                for s in vec!["DD", "Rxxx"] {
                    let s = String::from(s);
                    match s.parse::<Segment>().unwrap_err() {
                        SegmentParseError::LengthParseError(_) => {}
                        _ => return Err("expected LengthParseError".to_string()),
                    }
                }
                Ok(())
            }

            #[test]
            fn it_complains_when_length_is_non_positive() {
                for s in vec!["R0", "D0"] {
                    let s = String::from(s);
                    assert_eq!(
                        s.parse::<Segment>().unwrap_err(),
                        SegmentParseError::LengthZero(s)
                    );
                }
            }

            #[test]
            fn it_works_as_expected() {
                for (s, expected) in vec![
                    ("R10", Segment::R(10)),
                    ("U9", Segment::U(9)),
                    ("L1", Segment::L(1)),
                ] {
                    let s = String::from(s);
                    assert_eq!(s.parse::<Segment>().unwrap(), expected);
                }
            }
        }
    }

    mod point_tests {
        use super::super::{Point, Segment};
        use std::collections::HashSet;
        use std::iter::FromIterator;

        #[test]
        fn the_norm_works_as_expected() {
            let p = Point { x: -3, y: 2 };
            assert_eq!(p.manhattan_norm(), 5);
        }

        #[test]
        fn points_in_seqment_works_as_expected() {
            let test_cases: Vec<((i64, i64), _, Vec<(i64, i64)>)> = vec![
                ((3, 2), Segment::R(4), vec![(4, 2), (5, 2), (6, 2), (7, 2)]),
                ((1, -1), Segment::U(2), vec![(1, 0), (1, 1)]),
                ((0, 0), Segment::L(3), vec![(-1, 0), (-2, 0), (-3, 0)]),
                ((4, 4), Segment::D(4), vec![(4, 3), (4, 2), (4, 1), (4, 0)]),
            ];

            for (origin, segment, expected) in test_cases {
                let origin = Point {
                    x: origin.0,
                    y: origin.1,
                };
                let expected: Vec<Point> =
                    expected.iter().map(|p| Point { x: p.0, y: p.1 }).collect();
                let points_in_segment = origin.points_in_segment(segment);

                assert_eq!(points_in_segment, expected);
            }
        }

        #[test]
        fn points_in_path_works_as_expected() {
            let calculated_path_points = Point { x: 0, y: 0 }.points_in_path(&vec![
                Segment::R(4),
                Segment::U(4),
                Segment::L(4),
                Segment::D(2),
                Segment::R(6),
            ]);
            let expected_points = {
                let points: Vec<_> = vec![
                    (1, 0),
                    (2, 0),
                    (3, 0),
                    (4, 0),
                    (4, 1),
                    (4, 2),
                    (4, 3),
                    (4, 4),
                    (3, 4),
                    (2, 4),
                    (1, 4),
                    (0, 4),
                    (0, 3),
                    (0, 2),
                    (1, 2),
                    (2, 2),
                    (2, 2),
                    (3, 2),
                    (5, 2),
                    (6, 2),
                ]
                .iter()
                .map(|(x, y)| Point { x: *x, y: *y })
                .collect();
                HashSet::from_iter(points)
            };
            assert_eq!(expected_points, calculated_path_points);
        }
    }
}
