use std::fs;
use std::vec::Vec;

use super::{Riddle, RiddleError, Solution};

pub struct Advent2Riddle1 {
    input_file: String,
}

impl Advent2Riddle1 {
    pub fn new(input_file: &str) -> Advent2Riddle1 {
        let input_file = input_file.to_string();
        Advent2Riddle1 { input_file }
    }
}

impl Riddle for Advent2Riddle1 {
    fn solve(&self, _: &[String]) -> Result<Solution, RiddleError> {
        let numbers = get_numbers(&self.input_file)?;
        let mut program = Program::new(numbers);

        let noun = 12;
        let verb = 2;
        let result = program.run_with_parameters(noun, verb)?;

        Ok(Solution::Number(result))
    }
}

fn get_numbers(input_file: &str) -> Result<Vec<i64>, RiddleError> {
    let file_content = fs::read_to_string(input_file)?;
    let numbers: Result<Vec<i64>, _> = file_content
        .split(',')
        .map(|s| s.trim().parse::<i64>())
        .collect();
    Ok(numbers.unwrap())
}

pub struct Advent2Riddle2 {
    input_file: String,
}

impl Advent2Riddle2 {
    pub fn new(input_file: &str) -> Advent2Riddle2 {
        Advent2Riddle2 {
            input_file: input_file.to_string(),
        }
    }
}

impl Riddle for Advent2Riddle2 {
    fn solve(&self, _: &[String]) -> Result<Solution, RiddleError> {
        let numbers = get_numbers(&self.input_file)?;
        let target_result = 19_690_720;

        for noun in 0..100 {
            for verb in 0..100 {
                let mut program = Program::new(numbers.clone());
                let result = program.run_with_parameters(noun, verb)?;
                if result == target_result {
                    return Ok(Solution::Number(100 * noun + verb));
                }
            }
        }
        Err(RiddleError::NoSolutionFound)
    }
}

impl std::convert::From<IntcodeError> for RiddleError {
    fn from(err: IntcodeError) -> Self {
        RiddleError::Generic(format!("{:?}", err))
    }
}

#[derive(PartialEq, Eq, Debug)]
enum NextAction {
    Proceed,
    Stop,
}

#[derive(Debug, Eq, PartialEq)]
pub enum IntcodeError {
    UnknownOpcode(i64, usize),
    ProgramPositionOutOfBounds(usize),
    ModifyPositionOutOfBounds(i64),
    ArgumentPositionOutOfBounds(usize, i64),
    ProgramTooShort(usize),
}

#[derive(Eq, PartialEq, Debug)]
pub struct Program {
    int_code: Vec<i64>,
}

impl Program {
    pub fn new(int_code: Vec<i64>) -> Program {
        assert!(!int_code.is_empty());
        Program { int_code }
    }

    pub fn run_with_parameters(&mut self, noun: i64, verb: i64) -> Result<i64, IntcodeError> {
        let int_code_len = self.int_code.len();
        if int_code_len <= 4 {
            return Err(IntcodeError::ProgramPositionOutOfBounds(int_code_len));
        }
        self.int_code[1] = noun;
        self.int_code[2] = verb;
        self.run()?;
        Ok(self.int_code[0])
    }

    fn run(&mut self) -> Result<(), IntcodeError> {
        let opcodes = self.int_code.len();
        let highest_idx = opcodes - opcodes % 4;
        for i in (0..highest_idx).step_by(4) {
            if self.execute_step(i)? == NextAction::Stop {
                break;
            }
        }
        Ok(())
    }

    fn execute_step(&mut self, program_position: usize) -> Result<NextAction, IntcodeError> {
        let opcode = self.get_opcode(program_position)?;
        match opcode {
            1 | 2 => {
                let (arg1, arg2, modify_position) = self.get_args_and_pos(program_position)?;

                let new_value = if opcode == 1 {
                    arg1 + arg2
                } else {
                    arg1 * arg2
                };
                self.int_code[modify_position as usize] = new_value;

                Ok(NextAction::Proceed)
            }
            99 => Ok(NextAction::Stop),
            i => Err(IntcodeError::UnknownOpcode(i, program_position)),
        }
    }

    fn get_args_and_pos(&self, position: usize) -> Result<(i64, i64, i64), IntcodeError> {
        let modify_position = self.get_opcode(position + 3)?;

        if modify_position < 0 || modify_position > self.int_code.len() as i64 {
            return Err(IntcodeError::ModifyPositionOutOfBounds(modify_position));
        }
        let arg1 = self.get_arg(position + 1)?;
        let arg2 = self.get_arg(position + 2)?;

        Ok((arg1, arg2, modify_position))
    }

    fn get_arg(&self, position: usize) -> Result<i64, IntcodeError> {
        let argument_pos = self.get_opcode(position)?;
        if argument_pos < 0 || argument_pos as usize >= self.int_code.len() {
            return Err(IntcodeError::ArgumentPositionOutOfBounds(
                position,
                argument_pos,
            ));
        }
        Ok(self.int_code[argument_pos as usize])
    }

    pub fn get_opcode(&self, position: usize) -> Result<i64, IntcodeError> {
        if position > self.int_code.len() {
            Err(IntcodeError::ProgramPositionOutOfBounds(position))
        } else {
            Ok(self.int_code[position])
        }
    }
}

#[cfg(test)]
mod advent2_tests {
    mod riddle1_test {
        use super::super::super::{Riddle, Solution};
        use super::super::Advent2Riddle1;

        #[test]
        fn it_works_as_expected() {
            let riddle = Advent2Riddle1::new("./data/input/2.txt");
            let solution = riddle.solve(&vec![]).unwrap();

            assert_eq!(solution, Solution::Number(4023471));
        }
    }

    mod riddle2_test {
        use super::super::super::{Riddle, Solution};
        use super::super::Advent2Riddle2;

        #[test]
        fn it_works_as_expected() {
            let riddle = Advent2Riddle2::new("./data/input/2.txt");
            let solution = riddle.solve(&vec![]).unwrap();

            assert_eq!(solution, Solution::Number(8051));
        }
    }

    mod program_tests {
        mod run_with_parameters_tests {
            use super::super::super::Program;

            #[test]
            fn it_works_as_expected() {
                let mut program = Program::new(vec![1, 0, 0, 0, 99]);
                let result = program.run_with_parameters(4, 1).unwrap();
                assert_eq!(result, 103);
            }
        }

        mod run_tests {
            use super::super::super::Program;

            #[test]
            fn it_works_as_expected() {
                let test_cases = vec![
                    (
                        vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
                        vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
                    ),
                    (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
                    (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
                    (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
                    (
                        vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
                        vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
                    ),
                ];
                for (input, output) in test_cases {
                    let mut input = Program::new(input);
                    let output = Program::new(output);

                    input.run().unwrap();
                    assert_eq!(input, output);
                }
            }
        }

        mod execute_step_test {
            use super::super::super::{IntcodeError, NextAction, Program};

            #[test]
            fn it_works_for_opcode_1() {
                let mut program = Program::new(vec![1, 2, 2, 0]);
                let next = program.execute_step(0).unwrap();
                assert_eq!(program, Program::new(vec![4, 2, 2, 0]));
                assert_eq!(next, NextAction::Proceed);
            }

            #[test]
            fn it_works_for_opcode_2() {
                let mut program = Program::new(vec![2, 1, 0, 3]);
                let next = program.execute_step(0).unwrap();
                assert_eq!(program, Program::new(vec![2, 1, 0, 2]));
                assert_eq!(next, NextAction::Proceed);
            }

            #[test]
            fn it_works_for_opcode_99() {
                let mut program = Program::new(vec![99]);
                let next = program.execute_step(0).unwrap();
                assert_eq!(next, NextAction::Stop);
            }

            #[test]
            fn it_complains_for_other_opcodes() {
                let mut program = Program::new(vec![3, 5, 10, 2]);
                let result = program.execute_step(0);
                assert_eq!(result, Err(IntcodeError::UnknownOpcode(3, 0)));
            }

            #[test]
            fn it_complains_when_the_store_position_is_out_of_bounds() {
                {
                    let mut program = Program::new(vec![1, 5, 10, -1]);
                    let result = program.execute_step(0);
                    assert_eq!(result, Err(IntcodeError::ModifyPositionOutOfBounds(-1)));
                }
                {
                    let mut program = Program::new(vec![1, 5, 10, 10]);
                    let result = program.execute_step(0);
                    assert_eq!(result, Err(IntcodeError::ModifyPositionOutOfBounds(10)));
                }
            }

            #[test]
            fn it_complains_when_the_program_position_is_out_of_bounds() {
                let mut program = Program::new(vec![1, 5, 10, 10]);
                let result = program.execute_step(10);
                assert_eq!(result, Err(IntcodeError::ProgramPositionOutOfBounds(10)));
            }
        }

        mod get_arg_tests {
            use super::super::super::{IntcodeError, Program};

            #[test]
            fn it_works_as_expected() {
                let program = Program::new(vec![1, 2, 3, 4, 5, 6, 7, 8]);
                assert_eq!(program.get_arg(1).unwrap(), 3);
            }

            #[test]
            fn it_complains_when_position_is_out_of_bounds() {
                let program = Program::new(vec![1, 25, 3, 4, 5, 6, 7, 8]);
                assert_eq!(
                    program.get_arg(1),
                    Err(IntcodeError::ArgumentPositionOutOfBounds(1, 25))
                );
            }
        }
    }
}
