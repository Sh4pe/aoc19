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

fn execute_step(program: &mut Vec<i64>, program_position: usize) -> Result<NextAction, IntcodeError> {
    if program_position >= program.len() {
        return Err(IntcodeError::ProgramPositionOutOfBounds(program_position));
    }

    let opcode = program[program_position];
    match opcode {
        1 | 2 => {
            let modify_position_index = program_position + 3;
            if modify_position_index >= program.len() {
                return Err(IntcodeError::ProgramPositionOutOfBounds(modify_position_index));
            }
            let modify_position = program[modify_position_index];
            if modify_position < 0 || modify_position > program.len() as i64 {
                return Err(IntcodeError::ModifyPositionOutOfBounds(modify_position))
            }

            let new_value = if opcode == 1 {
                program[program_position + 1] + program[program_position + 2]
            } else {
                program[program_position + 1] * program[program_position + 2]
            };
            program[modify_position as usize] = new_value;

            Ok(NextAction::Proceed)
        },
        99 => Ok(NextAction::Stop),
        i => Err(IntcodeError::UnknownOpcode(i, program_position))
    }
}

#[cfg(test)]
mod advent2_tests {
    mod execute_step_test {
        use super::super::{execute_step, IntcodeError, NextAction};

        #[test]
        fn it_works_for_opcode_1() {
            let mut program = vec![1, 10, 10, 0];
            let next = execute_step(&mut program, 0).unwrap();
            assert_eq!(program, vec![20, 10, 10, 0]);
            assert_eq!(next, NextAction::Proceed);
        }

        #[test]
        fn it_works_for_opcode_2() {
            let mut program = vec![2, 5, 10, 2];
            let next = execute_step(&mut program, 0).unwrap();
            assert_eq!(program, vec![2, 5, 50, 2]);
            assert_eq!(next, NextAction::Proceed);
        }

        #[test]
        fn it_works_for_opcode_99() {
            let mut program = vec![99];
            let next = execute_step(&mut program, 0).unwrap();
            assert_eq!(next, NextAction::Stop);
        }

        #[test]
        fn it_complains_for_other_opcodes() {
            let mut program = vec![3, 5, 10, 2];
            let result = execute_step(&mut program, 0);
            assert_eq!(result, Err(IntcodeError::UnknownOpcode(3, 0)));
        }

        #[test]
        fn it_complains_when_the_store_position_is_out_of_bounds() {
            {
                let mut program = vec![1, 5, 10, -1];
                let result = execute_step(&mut program, 0);
                assert_eq!(result, Err(IntcodeError::ModifyPositionOutOfBounds(-1)));
            }
            {
                let mut program = vec![1, 5, 10, 10];
                let result = execute_step(&mut program, 0);
                assert_eq!(result, Err(IntcodeError::ModifyPositionOutOfBounds(10)));
            }
        }

        #[test]
        fn it_complains_when_the_program_position_is_out_of_bounds() {
            let mut program = vec![1, 5, 10, 10];
            let result = execute_step(&mut program, 10);
            assert_eq!(result, Err(IntcodeError::ProgramPositionOutOfBounds(10)));
        }
    }
}