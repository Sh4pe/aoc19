use super::super::io::ints_from_file;
use super::{Riddle, RiddleError, Solution};

pub struct Advent1Riddle1 {
    input_file: String,
}

impl Advent1Riddle1 {
    pub fn new(input_file: &str) -> Advent1Riddle1 {
        let input_file = input_file.to_string();
        Advent1Riddle1 { input_file }
    }
}

impl Riddle for Advent1Riddle1 {
    fn solve(&self, _: &[String]) -> Result<Solution, RiddleError> {
        let ints = ints_from_file(&self.input_file).unwrap();

        let result = ints
            .iter()
            .map(|&x| calculate_fuel(x))
            .fold(0, |acc, x| acc + x);
        Ok(Solution::Number(result))
    }
}

pub struct Advent1Riddle2 {
    input_file: String,
}

impl Advent1Riddle2 {
    pub fn new(input_file: &str) -> Advent1Riddle2 {
        let input_file = input_file.to_string();
        Advent1Riddle2 { input_file }
    }
}

impl Riddle for Advent1Riddle2 {
    fn solve(&self, _: &[String]) -> Result<Solution, RiddleError> {
        let ints = ints_from_file(&self.input_file).unwrap();

        let result = ints
            .iter()
            .map(|&x| calculate_fuel_recursive(x))
            .fold(0, |acc, x| acc + x);
        Ok(Solution::Number(result))
    }
}

pub fn calculate_fuel(mass: i64) -> i64 {
    assert!(mass >= 0);
    if mass <= 6 {
        0
    } else {
        mass / 3 - 2
    }
}

pub fn calculate_fuel_recursive(mass: i64) -> i64 {
    let increment = calculate_fuel(mass);
    if increment == 0 {
        increment
    } else {
        increment + calculate_fuel_recursive(increment)
    }
}

#[cfg(test)]
mod advent1_tests {

    mod calculate_fuel_tests {
        use super::super::calculate_fuel;

        #[test]
        fn it_works_as_expected() {
            for i in 0..6 {
                assert_eq!(calculate_fuel(i), 0);
            }
            assert_eq!(calculate_fuel(12), 2);
            assert_eq!(calculate_fuel(14), 2);
            assert_eq!(calculate_fuel(1969), 654);
            assert_eq!(calculate_fuel(100756), 33583);
        }
    }

    mod calculate_fuel_recursive_tests {
        use super::super::calculate_fuel_recursive;

        #[test]
        fn it_works_as_expected() {
            let test_cases = vec![(14, 2), (1969, 966), (100756, 50346)];
            for (value, expected) in test_cases {
                assert_eq!(calculate_fuel_recursive(value), expected);
            }
        }
    }

    mod riddle1_test {
        use super::super::super::{Riddle, Solution};
        use super::super::Advent1Riddle1;

        #[test]
        fn it_works_as_expected() {
            let riddle = Advent1Riddle1::new("./data/input/1.txt");
            let solution = riddle.solve(&vec![]).unwrap();

            assert_eq!(solution, Solution::Number(3210097));
        }
    }

    mod riddle2_test {
        use super::super::super::{Riddle, Solution};
        use super::super::Advent1Riddle2;

        #[test]
        fn it_works_as_expected() {
            let riddle = Advent1Riddle2::new("./data/input/1.txt");
            let solution = riddle.solve(&vec![]).unwrap();

            assert_eq!(solution, Solution::Number(4812287));
        }
    }
}
