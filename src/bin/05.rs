advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i64> {
    run_machine(input, "1")
}

pub fn part_two(_input: &str) -> Option<i64> {
    None
}

pub fn run_machine(memory: &str, input: &str) -> Option<i64> {
    let memory: Vec<i64> = memory
        .split(",")
        .map(|i| i.parse().expect("Expected i64 list"))
        .collect();

    let input: Vec<i64> = if input.len() != 0 {
        input
            .split(",")
            .map(|i| i.parse().expect("Expected i64 list"))
            .collect()
    } else {
        vec![]
    };

    let mut machine = Machine {
        memory,
        instruction_pointer: 0,
        input,
        input_pointer: 0,
    };
    match machine.execute() {
        Ok(result) => Some(result),
        Err(m) => panic!("{}", m),
    }
}

struct Machine {
    memory: Vec<i64>,
    instruction_pointer: usize,

    input: Vec<i64>,
    input_pointer: usize,
}

#[allow(clippy::let_and_return)]
#[allow(dead_code)]
impl Machine {
    // fn get_noun(&self) -> u64 {
    //     self.memory[1]
    // }

    // fn get_verb(&self) -> u64 {
    //     self.memory[2]
    // }

    fn clone_with_noun_verb(&self, noun: i64, verb: i64) -> Machine {
        let mut new_memory = self.memory.clone();
        new_memory[1] = noun;
        new_memory[2] = verb;

        Machine {
            memory: new_memory,
            instruction_pointer: 0,
            input: self.input.clone(),
            input_pointer: self.input_pointer,
        }
    }

    fn get_parameter_value(&self, parameter: &InputParameter) -> i64 {
        match parameter.mode {
            ParameterMode::Position => {
                let addr = parameter.value as usize;
                if addr >= self.memory.len() {
                    panic!("Unexpected memory address for operand address [{addr}]!");
                } else {
                    self.memory[addr]
                }
            }
            ParameterMode::Immediate => parameter.value,
        }
    }

    fn get_output_parameter_address(&self, parameter: &OutputParameter) -> i64 {
        let addr = parameter.value as usize;
        if addr >= self.memory.len() {
            panic!("Unexpected memory address for operand address [{addr}]!");
        } else {
            parameter.value
        }
    }

    fn write_to_memory(&mut self, address: i64, value: i64) {
        let address = address as usize;
        if address >= self.memory.len() {
            panic!("Unexpected memory address for operand address [{address}]!");
        } else {
            self.memory[address] = value;
        }
    }

    // execute until halt
    fn execute(&mut self) -> Result<i64, String> {
        //self.print();

        let mut exe_result;
        loop {
            exe_result = self.execute_step();
            match exe_result {
                ExecuteResult::Continue => (), // go on
                ExecuteResult::Panic(message) => return Result::Err(message),
                ExecuteResult::Halt(result) => {
                    //machine.print();
                    return Ok(result);
                }
            }
        }
    }

    fn execute_step(&mut self) -> ExecuteResult {
        if self.memory.len() < self.instruction_pointer + 1 {
            return ExecuteResult::Panic("Unexpected memory overflow!".to_string());
        }

        // Parse instruction
        let instruction = Instruction::from(&self.memory[self.instruction_pointer..]);

        let result = self.exe_instruction(&instruction);

        //self.print();

        result
    }

    fn exe_instruction(&mut self, instruction: &Instruction) -> ExecuteResult {
        match instruction {
            Instruction::Add {
                first_operand,
                second_operand,
                result,
            } => {
                let a = self.get_parameter_value(first_operand);
                let b = self.get_parameter_value(second_operand);
                let res_addr = self.get_output_parameter_address(result);

                let result = a + b;
                self.write_to_memory(res_addr, result);

                self.instruction_pointer += instruction.get_size() as usize;

                ExecuteResult::Continue
            }
            Instruction::Mul {
                first_operand,
                second_operand,
                result,
            } => {
                let a = self.get_parameter_value(first_operand);
                let b = self.get_parameter_value(second_operand);
                let res_addr = self.get_output_parameter_address(result);

                let result = a * b;
                self.write_to_memory(res_addr, result);

                self.instruction_pointer += instruction.get_size() as usize;

                ExecuteResult::Continue
            }
            Instruction::Inp { result } => {
                let res_addr = self.get_output_parameter_address(result);

                let value = self.input[self.input_pointer];
                self.input_pointer += 1;

                self.write_to_memory(res_addr, value);

                self.instruction_pointer += instruction.get_size() as usize;

                ExecuteResult::Continue
            }
            Instruction::Out { operand } => {
                let result = self.get_parameter_value(operand);
                println!("> {result}");

                self.instruction_pointer += instruction.get_size() as usize;

                ExecuteResult::Continue
            }
            Instruction::Halt => {
                let result = self.memory[0];
                self.instruction_pointer += instruction.get_size() as usize;
                ExecuteResult::Halt(result)
            }
            Instruction::Reserved => todo!(),
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("Instruction pointer C: {}", self.instruction_pointer);
        println!("Memory: {:?}", self.memory);
        println!();
    }
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

impl From<i64> for ParameterMode {
    fn from(value: i64) -> Self {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Unknown parameter mode [{value}]"),
        }
    }
}

#[derive(Debug)]
struct InputParameter {
    mode: ParameterMode,
    value: i64,
}

#[derive(Debug)]
struct OutputParameter {
    value: i64,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Instruction {
    Add {
        first_operand: InputParameter,
        second_operand: InputParameter,
        result: OutputParameter,
    },
    Mul {
        first_operand: InputParameter,
        second_operand: InputParameter,
        result: OutputParameter,
    },
    Inp {
        result: OutputParameter,
    },
    Out {
        operand: InputParameter,
    },
    Halt,
    Reserved,
}

impl Instruction {
    fn get_size(&self) -> u8 {
        match self {
            Instruction::Add { .. } => 4,
            Instruction::Mul { .. } => 4,
            Instruction::Inp { .. } => 2,
            Instruction::Out { .. } => 2,
            Instruction::Halt => 1,
            Instruction::Reserved => todo!(),
        }
    }
}

impl From<&[i64]> for Instruction {
    fn from(memory: &[i64]) -> Self {
        let opcode = memory[0];
        let mut opcode_len = (opcode.checked_ilog10().unwrap_or(0) + 1) as i64;

        let instruction_code = opcode % 100;
        let mut modes_opcode = opcode / 100;
        opcode_len -= 2;

        let mut parameter_modes = Vec::new();
        while opcode_len > 0 {
            parameter_modes.push(ParameterMode::from(modes_opcode % 10));
            modes_opcode /= 10;
            opcode_len -= 1;
        }

        //parameter_modes.reverse();
        let mut parameter_modes_iter = parameter_modes.into_iter();

        match instruction_code {
            1 => Self::Add {
                first_operand: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
                second_operand: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[2],
                },
                result: OutputParameter { value: memory[3] },
            },
            2 => Self::Mul {
                first_operand: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
                second_operand: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[2],
                },
                result: OutputParameter { value: memory[3] },
            },
            3 => Self::Inp {
                result: OutputParameter { value: memory[1] },
            },
            4 => Self::Out {
                operand: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
            },
            99 => Self::Halt {},
            _ => panic!("Unknown instruction opcode [{instruction_code}]!"),
        }
    }
}

#[derive(PartialEq)]
enum ExecuteResult {
    Continue,
    Halt(i64),
    Panic(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_1() {
        let result = run_machine(
            &advent_of_code::template::read_file_part("examples", DAY, 1),
            "",
        );
        assert_eq!(result, Some(3500));
    }

    #[test]
    fn test_run_2() {
        let result = run_machine(
            &advent_of_code::template::read_file_part("examples", DAY, 2),
            "",
        );
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_run_3() {
        let result = run_machine(
            &advent_of_code::template::read_file_part("examples", DAY, 3),
            "",
        );
        assert_eq!(result, Some(1101));
    }

    #[test]
    fn test_run_4() {
        let result = run_machine(
            &advent_of_code::template::read_file_part("examples", DAY, 4),
            "",
        );
        assert_eq!(result, Some(1002));
    }

    #[test]
    fn test_run_5() {
        // read value, print value, return value
        let result = run_machine(
            &advent_of_code::template::read_file_part("examples", DAY, 5),
            "1337",
        );
        assert_eq!(result, Some(1337));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5, // todo - change!
        ));
        assert_eq!(result, None);
    }
}
