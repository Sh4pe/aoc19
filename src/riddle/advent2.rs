use std::vec::{Vec};

#[derive(PartialEq, Eq, Debug)]
enum NextAction {
    Proceed,
    Stop
}

#[derive(Debug, Eq, PartialEq)]
pub enum IntcodeError {
    UnknownOpcode(i64, usize),
    ProgramPositionOutOfBounds(usize),
    ModifyPositionOutOfBounds(i64),
    ArgumentPositionOutOfBounds(usize, i64)
}

#[derive(Eq, PartialEq, Debug)]
pub struct Program {
    int_code: Vec<i64>
}

impl Program {
    pub fn new(int_code: Vec<i64>) -> Program {
        Program{ int_code }
    }

    pub fn execute(&mut self) -> Result<(), IntcodeError> {
        let opcodes = self.int_code.len();
        let highest_idx = opcodes - opcodes % 4;
        for i in (0..highest_idx).step_by(4) {
            if self.execute_step(i)? == NextAction::Stop {
                break
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
            },
            99 => Ok(NextAction::Stop),
            i => Err(IntcodeError::UnknownOpcode(i, program_position))
        }
    }

    fn get_args_and_pos(&self, position: usize) -> Result<(i64, i64, i64), IntcodeError> {
        let modify_position = self.get_opcode(position + 3)?;

        if modify_position < 0 || modify_position > self.int_code.len() as i64 {
            return Err(IntcodeError::ModifyPositionOutOfBounds(modify_position))
        } 
        let arg1 = self.get_arg(position + 1)?;
        let arg2 = self.get_arg(position + 2)?;

        Ok((arg1, arg2, modify_position))
    }

    fn get_arg(&self, position: usize) -> Result<i64, IntcodeError> {
        let argument_pos = self.get_opcode(position)?;
        if argument_pos < 0 || argument_pos as usize >= self.int_code.len() {
            return Err(IntcodeError::ArgumentPositionOutOfBounds(position, argument_pos));
        }
        Ok(self.int_code[argument_pos as usize])
    }

    fn get_opcode(&self, position: usize) -> Result<i64, IntcodeError> {
        if position > self.int_code.len() {
            Err(IntcodeError::ProgramPositionOutOfBounds(position))
        } else {
            Ok(self.int_code[position])
        }
    }
}

#[cfg(test)]
mod advent2_tests {
    mod program_tests {
        mod execute_tests {
            use super::super::super::{Program};

            #[test]
            fn it_works_as_expected() {
                let test_cases = vec![
                    (vec![1,9,10,3,2,3,11,0,99,30,40,50], vec![3500,9,10,70,2,3,11,0,99,30,40,50]),
                    (vec![1,0,0,0,99], vec![2,0,0,0,99]),
                    (vec![2,3,0,3,99], vec![2,3,0,6,99]),
                    (vec![2,4,4,5,99,0], vec![2,4,4,5,99,9801]),
                    (vec![1,1,1,4,99,5,6,0,99], vec![30,1,1,4,2,5,6,0,99]),
                ];
                for (input, output) in test_cases {
                    let mut input = Program::new(input);
                    let output = Program::new(output);

                    input.execute().unwrap();
                    assert_eq!(input, output);
                }
            }
        }

        mod execute_step_test {
            use super::super::super::{Program, IntcodeError, NextAction};

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
            use super::super::super::{Program, IntcodeError};

            #[test]
            fn it_works_as_expected() {
                let program = Program::new(vec![1,2,3,4,5,6,7,8]);
                assert_eq!(program.get_arg(1).unwrap(), 3);
            }

            #[test]
            fn it_complains_when_position_is_out_of_bounds() {
                let program = Program::new(vec![1,25,3,4,5,6,7,8]);
                assert_eq!(program.get_arg(1), Err(IntcodeError::ArgumentPositionOutOfBounds(1, 25)));
            }
        }
    }
}