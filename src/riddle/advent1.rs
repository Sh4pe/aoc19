use super::{Riddle, Solution, RiddleError};

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
    fn solve(&self, args: &[String]) -> Result<Solution, RiddleError> {
        println!("asdfasdf");
        Ok(Solution::Number(2))
    }
}

pub fn calculate_fuel(mass: u64) -> u64 {
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
}