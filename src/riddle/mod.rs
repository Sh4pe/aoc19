
pub enum Solution {
    Number(i64)
}

pub enum Error {
    IO(std::io::Error),
    ArgumentsCount(u8, u8),
}

pub trait Riddle {
    fn solve(self, args: &[String]) -> Result<Solution, Error>;
}

pub struct Riddles {

}

impl Riddles {
    pub fn new() {}
}

#[cfg(test)]
mod riddle_tests {
    mod riddles_test {
        mod constructor_tests {
            use super::super::super::*;

            #[test]
            fn it_works() {
                let _ = Riddles::new();
                assert_eq!(1, 1);
            }
        }
    }
}