use std::vec::{Vec};

#[derive(PartialEq, Eq, Debug)]
enum NextAction {
    Proceed,
    Stop
}

#[derive(Debug, Eq, PartialEq)]
enum IntcodeError {
    UnknownOpcode(i64, usize),
    ProgramPositionOutOfBounds(usize),
    ModifyPositionOutOfBounds(i64),
}

#[derive(Eq, PartialEq, Debug)]
pub struct Program {
    int_code: Vec<i64>
}

impl Program {
    pub fn new(int_code: Vec<i64>) -> Program {
        Program{ int_code }
    }

    fn execute_step(&mut self, program_position: usize) -> Result<NextAction, IntcodeError> {
        if program_position >= self.int_code.len() {
            return Err(IntcodeError::ProgramPositionOutOfBounds(program_position));
        }

        let opcode = self.int_code[program_position];
        match opcode {
            1 | 2 => {
                let modify_position_index = program_position + 3;
                if modify_position_index >= self.int_code.len() {
                    return Err(IntcodeError::ProgramPositionOutOfBounds(modify_position_index));
                }
                let modify_position = self.int_code[modify_position_index];
                if modify_position < 0 || modify_position > self.int_code.len() as i64 {
                    return Err(IntcodeError::ModifyPositionOutOfBounds(modify_position))
                }

                let new_value = if opcode == 1 {
                    self.int_code[program_position + 1] + self.int_code[program_position + 2]
                } else {
                    self.int_code[program_position + 1] * self.int_code[program_position + 2]
                };
                self.int_code[modify_position as usize] = new_value;

                Ok(NextAction::Proceed)
            },
            99 => Ok(NextAction::Stop),
            i => Err(IntcodeError::UnknownOpcode(i, program_position))
        }
    }
}

#[cfg(test)]
mod advent2_tests {
    mod program_tests {
        mod execute_step_test {
            use super::super::super::{Program, IntcodeError, NextAction};

            #[test]
            fn it_works_for_opcode_1() {
                let mut program = Program::new(vec![1, 10, 10, 0]);
                let next = program.execute_step(0).unwrap();
                assert_eq!(program, Program::new(vec![20, 10, 10, 0]));
                assert_eq!(next, NextAction::Proceed);
            }

            #[test]
            fn it_works_for_opcode_2() {
                let mut program = Program::new(vec![2, 5, 10, 2]);
                let next = program.execute_step(0).unwrap();
                assert_eq!(program, Program::new(vec![2, 5, 50, 2]));
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
    }
}