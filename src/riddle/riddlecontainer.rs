use std::collections::HashMap;

use super::advent1::{Advent1Riddle1, Advent1Riddle2};
use super::advent2::{Advent2Riddle1, Advent2Riddle2};
use super::advent3::Advent3Riddle1;
use super::{Riddle, RiddleError, Solution};

pub struct RiddleContainer {
    advent1_riddle1: Advent1Riddle1,
    advent1_riddle2: Advent1Riddle2,
    advent2_riddle1: Advent2Riddle1,
    advent2_riddle2: Advent2Riddle2,
    advent3_riddle1: Advent3Riddle1,
}

impl RiddleContainer {
    pub fn new() -> RiddleContainer {
        let advent1_riddle1 = Advent1Riddle1::new("./data/input/1.txt");
        let advent1_riddle2 = Advent1Riddle2::new("./data/input/1.txt");
        let advent2_riddle1 = Advent2Riddle1::new("./data/input/2.txt");
        let advent2_riddle2 = Advent2Riddle2::new("./data/input/2.txt");
        let advent3_riddle1 = Advent3Riddle1::new("./data/input/3.txt");

        RiddleContainer {
            advent1_riddle1,
            advent1_riddle2,
            advent2_riddle1,
            advent2_riddle2,
            advent3_riddle1,
        }
    }

    pub fn solve_riddle(&self, riddle: String, args: &[String]) -> Result<Solution, RiddleError> {
        let riddle_map = {
            let mut riddle_map: HashMap<
                String,
                Box<dyn Fn(&[String]) -> Result<Solution, RiddleError>>,
            > = HashMap::new();

            riddle_map.insert(
                "1.1".to_string(),
                Box::new(|args| self.advent1_riddle1.solve(args)),
            );
            riddle_map.insert(
                "1.2".to_string(),
                Box::new(|args| self.advent1_riddle2.solve(args)),
            );
            riddle_map.insert(
                "2.1".to_string(),
                Box::new(|args| self.advent2_riddle1.solve(args)),
            );
            riddle_map.insert(
                "2.2".to_string(),
                Box::new(|args| self.advent2_riddle2.solve(args)),
            );
            riddle_map.insert(
                "3.1".to_string(),
                Box::new(|args| self.advent3_riddle1.solve(args)),
            );

            riddle_map
        };

        if riddle_map.contains_key(&riddle) {
            riddle_map[&riddle](args)
        } else {
            Err(RiddleError::UnknownRiddle(riddle))
        }
    }
}

#[cfg(test)]
mod riddle_container_test {
    mod constructor_tests {
        use super::super::*;

        #[test]
        fn it_works() {
            let _ = RiddleContainer::new();
            assert_eq!(1, 1);
        }
    }
}
