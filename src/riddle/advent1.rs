use super::{Riddle, Solution, RiddleError};
use super::super::io::{ints_from_file};

pub struct Advent1Riddle1 {
    input_file: String
}

impl Advent1Riddle1 {
    pub fn new(input_file: &str) -> Advent1Riddle1 {
        let input_file = input_file.to_string();
        Advent1Riddle1{ input_file }
    }
}

impl Riddle for Advent1Riddle1 {
    fn solve(&self, _: &[String]) -> Result<Solution, RiddleError> {
        let ints = ints_from_file(&self.input_file).unwrap();

        let result = ints
            .iter()
            .map( |&x| calculate_fuel(x) )
            .fold(0, |acc, x| acc + x );
        Ok(Solution::Number(result))
    }
}

pub fn calculate_fuel(mass: i64) -> i64 {
    assert!(mass >= 6);
    mass/3 - 2
}

#[cfg(test)]
mod advent1_tests {

    mod calculate_fuel_tests {
        use super::super::calculate_fuel;

        #[test]
        fn it_works_as_expected() {
            assert_eq!(calculate_fuel(12), 2);
            assert_eq!(calculate_fuel(14), 2);
            assert_eq!(calculate_fuel(1969), 654);
            assert_eq!(calculate_fuel(100756), 33583);
        }
    }

    mod riddle1_test {
        use super::super::Advent1Riddle1;
        use super::super::super::{Riddle, Solution};

        #[test]
        fn it_works_as_expected() {
            let riddle = Advent1Riddle1::new("./data/input/1.txt");
            let solution = riddle.solve(&vec![]).unwrap();

            assert_eq!(solution, Solution::Number(3210097));
        }
    } 
}