use std::fs::File;
use std::io::{BufReader, BufRead, Lines, Error};
use std::vec::{Vec};

type FileLines = Lines<BufReader<File>>;

pub fn lines_from_file(filename: &str) -> Result<FileLines, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

#[derive(Debug)]
pub enum IntsError {
    Convert(String),
    ReadLine,
    IO(Error)
}

impl std::convert::From<std::io::Error> for IntsError {
    fn from(err: std::io::Error) -> Self { IntsError::IO(err) }
}

pub fn ints_from_file(filename: &str) -> Result<Vec<i64>, IntsError> {
    let lines = lines_from_file(filename)?;
    let result : Result<Vec<i64>, _> = lines.map( |line| {
        match line {
            Err(_) => Err(IntsError::ReadLine),
            Ok(l) => match l.parse::<i64>() {
                Err(_) => Err(IntsError::Convert(format!("could not convert {} to int", l))),
                Ok(i) => Ok(i)
            }
        }
    }).collect();
    result
}

#[cfg(test)]
mod io_tests {
    mod lines_from_file_test {
        use super::super::{lines_from_file};

        #[test]
        fn it_works_with_str() {
            let lines : Vec<_> = lines_from_file(
                "./data/test/io/lines_from_file/three_lines.txt"
            ).unwrap().collect();
            assert_eq!(3, lines.len());
        }

        #[test]
        fn it_works_with_string() {
            let path = String::from("./data/test/io/lines_from_file/three_lines.txt");
            let lines : Vec<_> = lines_from_file(&path).unwrap().collect();
            assert_eq!(3, lines.len());
        }
    }

    mod ints_from_file_test {
        use super::super::{ints_from_file, IntsError};

        #[test]
        fn it_works_in_normal_cases() {
            let ints = ints_from_file(
                "./data/test/io/ints_from_file/ints.txt"
            ).unwrap();

            assert_eq!(ints, vec![1, -1, 100, 9999]);
        }

        #[test]
        fn it_behaves_as_expected_when_it_cannot_parse() -> Result<(), String> {
            let path = "./data/test/io/ints_from_file/no_ints.txt";
            match ints_from_file(path) {
                Err(IntsError::Convert(_)) => Ok(()),
                Ok(_) => Err(String::from("expected error")),
                Err(_) =>  Err(String::from("expected convert error"))
            }
        }
    }
}