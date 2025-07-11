advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let result = run_machine_with_extended_memory(input, "1");
    //dbg!(&result);
    Some(result.map(|(_, output)| output).unwrap()[0])
}

pub fn part_two(input: &str) -> Option<i64> {
    let result = run_machine_with_extended_memory(input, "2");
    //dbg!(&result);
    Some(result.map(|(_, output)| output).unwrap()[0])
}

pub fn run_machine_with_extended_memory(firmware: &str, input: &str) -> Option<(i64, Vec<i64>)> {
    let firmware: Vec<i64> = firmware
        .split(",")
        .map(|i| i.parse().expect("Expected i64 list"))
        .collect();

    let input: Vec<i64> = if !input.is_empty() {
        input
            .split(",")
            .map(|i| i.parse().expect("Expected i64 list"))
            .collect()
    } else {
        vec![]
    };

    // extend memory to at least 4096
    let ext_memory_size = 4096;
    let mut ext_memory: Vec<i64> = Vec::with_capacity(ext_memory_size);
    for i in 0..ext_memory_size {
        if i < firmware.len() {
            ext_memory.push(firmware[i]);
        } else {
            ext_memory.push(0);
        }
    }

    let mut machine = Machine::new_with_input(ext_memory, input);
    match machine.execute() {
        ExecuteResult::Halt(result) => Some((result, machine.output)),
        ExecuteResult::Panic(m) => panic!("{}", m),
        _ => panic!("Unexpected result!"),
    }
}

pub fn run_machine(memory: &str, input: &str) -> Option<(i64, Vec<i64>)> {
    let memory: Vec<i64> = memory
        .split(",")
        .map(|i| i.parse().expect("Expected i64 list"))
        .collect();

    let input: Vec<i64> = if !input.is_empty() {
        input
            .split(",")
            .map(|i| i.parse().expect("Expected i64 list"))
            .collect()
    } else {
        vec![]
    };

    let mut machine = Machine::new_with_input(memory, input);
    match machine.execute() {
        ExecuteResult::Halt(result) => Some((result, machine.output)),
        ExecuteResult::Panic(m) => panic!("{}", m),
        _ => panic!("Unexpected result!"),
    }
}

struct Machine {
    memory: Vec<i64>,
    instruction_pointer: usize,

    input: Vec<i64>,
    input_pointer: usize,

    output: Vec<i64>,
    output_pointer: usize,

    relative_base: i64,
}

#[allow(clippy::let_and_return)]
#[allow(dead_code)]
impl Machine {
    fn new(memory: Vec<i64>) -> Self {
        Machine {
            memory,
            instruction_pointer: 0,
            input: Vec::new(),
            input_pointer: 0,
            output: Vec::new(),
            output_pointer: 0,
            relative_base: 0,
        }
    }

    fn new_with_input(memory: Vec<i64>, input: Vec<i64>) -> Self {
        Machine {
            memory,
            instruction_pointer: 0,
            input,
            input_pointer: 0,
            output: Vec::new(),
            output_pointer: 0,
            relative_base: 0,
        }
    }

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
            output: self.output.clone(),
            output_pointer: self.output_pointer,
            relative_base: self.relative_base,
        }
    }

    fn dump(&self) -> String {
        let memory_dump = format!("MEM: {:?}", self.memory);
        let ins_p_dump = format!("IP: {}", self.instruction_pointer);
        let input_dump = format!("INP: {:?}", self.input);
        let inp_p_dump = format!("INP_P: {}", self.input_pointer);
        let output_dump = format!("OUT: {:?}", self.output);
        let otp_p_dump = format!("OUT_P: {}", self.output_pointer);
        let rel_base = format!("REL_B: {}", self.relative_base);

        format!(
            " ======\n {memory_dump}\n {ins_p_dump}\n {input_dump} \n {inp_p_dump}\n {output_dump} \n {otp_p_dump} \n {rel_base}"
        )
        .to_string()
    }

    fn get_parameter_value(&self, parameter: &Parameter) -> i64 {
        match parameter.mode {
            ParameterMode::Position => {
                let addr = parameter.value as usize;
                if addr >= self.memory.len() {
                    panic!("Unexpected memory address for parameter address [{addr}]!");
                } else {
                    self.memory[addr]
                }
            }
            ParameterMode::Immediate => parameter.value,
            ParameterMode::Relative => {
                let expected_addr = parameter.value + self.relative_base;
                if expected_addr < 0 {
                    panic!("Unexpected memory address for relative parameter address [{expected_addr}]!");
                }
                let addr = expected_addr as usize;
                if addr >= self.memory.len() {
                    panic!("Unexpected memory address for parameter address [{addr}]!");
                } else {
                    self.memory[addr]
                }
            }
        }
    }

    fn get_output_parameter_address(&self, parameter: &Parameter) -> i64 {
        match parameter.mode {
            ParameterMode::Position => {
                let addr = parameter.value as usize;
                if addr >= self.memory.len() {
                    panic!("Unexpected memory address for parameter address [{addr}]!");
                } else {
                    addr as i64
                }
            }
            ParameterMode::Relative => {
                let expected_addr = parameter.value + self.relative_base;
                if expected_addr < 0 {
                    panic!("Unexpected memory address for relative parameter address [{expected_addr}]!");
                }
                let addr = expected_addr as usize;
                if addr >= self.memory.len() {
                    panic!("Unexpected memory address for parameter address [{addr}]!");
                } else {
                    addr as i64
                }
            }
            ParameterMode::Immediate => {
                panic!("Unexpected Immediate parameter for output instruction!")
            }
        }
    }

    fn write_to_memory(&mut self, address: i64, value: i64) {
        let address = address as usize;
        if address >= self.memory.len() {
            panic!("Unexpected memory address for parameter address [{address}]!");
        } else {
            self.memory[address] = value;
        }
    }

    // execute until halt
    fn execute(&mut self) -> ExecuteResult {
        //self.print();

        let mut exe_result;
        //let mut max_loop = 1000;

        loop {
            ///////////// dbg
            //print!("{}", self.dump());
            //if max_loop == 0 {
            //    return Result::Err("INFL".to_string());
            //}
            //max_loop -= 1;
            ///////////// end dbg
            exe_result = self.execute_step();
            match exe_result {
                ExecuteResult::Continue => (),
                ExecuteResult::Panic(message) => return ExecuteResult::Panic(message),
                ExecuteResult::Halt(result) => {
                    //machine.print();
                    return ExecuteResult::Halt(result);
                }
                ExecuteResult::WaitingInput => return ExecuteResult::WaitingInput,
            }
        }
    }

    fn execute_step(&mut self) -> ExecuteResult {
        if self.memory.len() < self.instruction_pointer + 1 {
            return ExecuteResult::Panic("Unexpected memory overflow!".to_string());
        }

        // Parse instruction
        match Instruction::try_from(&self.memory[self.instruction_pointer..]) {
            Ok(instruction) => {
                let result = self.exe_instruction(&instruction);

                //self.print();

                result
            }
            Err(message) => {
                let dump = self.dump();
                panic!("Error while parsing instructions: {message} \n{dump}")
            }
        }
    }

    fn exe_instruction(&mut self, instruction: &Instruction) -> ExecuteResult {
        match instruction {
            Instruction::Add {
                first_parameter,
                second_parameter,
                result,
            } => {
                let a = self.get_parameter_value(first_parameter);
                let b = self.get_parameter_value(second_parameter);
                let res_addr = self.get_output_parameter_address(result);

                let result = a + b;
                self.write_to_memory(res_addr, result);

                self.instruction_pointer += instruction.get_size() as usize;

                ExecuteResult::Continue
            }
            Instruction::Mul {
                first_parameter,
                second_parameter,
                result,
            } => {
                let a = self.get_parameter_value(first_parameter);
                let b = self.get_parameter_value(second_parameter);
                let res_addr = self.get_output_parameter_address(result);

                let result = a * b;
                self.write_to_memory(res_addr, result);

                self.instruction_pointer += instruction.get_size() as usize;

                ExecuteResult::Continue
            }
            Instruction::Inp { result } => {
                let res_addr = self.get_output_parameter_address(result);

                if self.input_pointer >= self.input.len() {
                    return ExecuteResult::WaitingInput;
                }

                let value = self.input[self.input_pointer];
                self.input_pointer += 1;

                self.write_to_memory(res_addr, value);

                self.instruction_pointer += instruction.get_size() as usize;

                ExecuteResult::Continue
            }
            Instruction::Out { parameter } => {
                let result = self.get_parameter_value(parameter);
                //println!("> {result}");

                self.output.push(result);
                self.output_pointer += 1;

                self.instruction_pointer += instruction.get_size() as usize;

                ExecuteResult::Continue
            }
            Instruction::Halt => {
                let result = self.memory[0];
                self.instruction_pointer += instruction.get_size() as usize;
                ExecuteResult::Halt(result)
            }

            Instruction::Reserved => todo!(),
            Instruction::Jit {
                first_parameter,
                second_parameter,
            } => {
                if 0 != self.get_parameter_value(first_parameter) {
                    self.instruction_pointer = self.get_parameter_value(second_parameter) as usize;
                    return ExecuteResult::Continue;
                }

                self.instruction_pointer += instruction.get_size() as usize;
                ExecuteResult::Continue
            }
            Instruction::Jif {
                first_parameter,
                second_parameter,
            } => {
                if 0 == self.get_parameter_value(first_parameter) {
                    self.instruction_pointer = self.get_parameter_value(second_parameter) as usize;
                    return ExecuteResult::Continue;
                }

                self.instruction_pointer += instruction.get_size() as usize;
                ExecuteResult::Continue
            }
            Instruction::Lst {
                first_parameter,
                second_parameter,
                result,
            } => {
                let a = self.get_parameter_value(first_parameter);
                let b = self.get_parameter_value(second_parameter);

                let value = if a < b { 1 } else { 0 };

                let res_addr = self.get_output_parameter_address(result);
                self.write_to_memory(res_addr, value);

                self.instruction_pointer += instruction.get_size() as usize;
                ExecuteResult::Continue
            }
            Instruction::Equ {
                first_parameter,
                second_parameter,
                result,
            } => {
                let a = self.get_parameter_value(first_parameter);
                let b = self.get_parameter_value(second_parameter);

                let value = if a == b { 1 } else { 0 };

                let res_addr = self.get_output_parameter_address(result);
                self.write_to_memory(res_addr, value);

                self.instruction_pointer += instruction.get_size() as usize;
                ExecuteResult::Continue
            }
            Instruction::Arb { first_parameter } => {
                let relative_base = self.get_parameter_value(first_parameter);
                self.relative_base += relative_base;

                self.instruction_pointer += instruction.get_size() as usize;
                ExecuteResult::Continue
            }
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
    Relative,
}

impl From<i64> for ParameterMode {
    fn from(value: i64) -> Self {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Unknown parameter mode [{value}]"),
        }
    }
}

#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    value: i64,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Instruction {
    Add {
        first_parameter: Parameter,
        second_parameter: Parameter,
        result: Parameter,
    },
    Mul {
        first_parameter: Parameter,
        second_parameter: Parameter,
        result: Parameter,
    },
    Inp {
        result: Parameter,
    },
    Out {
        parameter: Parameter,
    },
    Jit {
        first_parameter: Parameter,
        second_parameter: Parameter,
    },
    Jif {
        first_parameter: Parameter,
        second_parameter: Parameter,
    },
    Lst {
        first_parameter: Parameter,
        second_parameter: Parameter,
        result: Parameter,
    },
    Equ {
        first_parameter: Parameter,
        second_parameter: Parameter,
        result: Parameter,
    },
    Arb {
        first_parameter: Parameter,
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
            Instruction::Jit { .. } => 3,
            Instruction::Jif { .. } => 3,
            Instruction::Lst { .. } => 4,
            Instruction::Equ { .. } => 4,
            Instruction::Arb { .. } => 2, // ??? check to make sure
            Instruction::Halt => 1,
            Instruction::Reserved => todo!(),
        }
    }
}

impl TryFrom<&[i64]> for Instruction {
    type Error = String;

    fn try_from(memory: &[i64]) -> Result<Self, Self::Error> {
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
            1 => Ok(Self::Add {
                first_parameter: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
                second_parameter: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[2],
                },
                result: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[3],
                },
            }),
            2 => Ok(Self::Mul {
                first_parameter: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
                second_parameter: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[2],
                },
                result: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[3],
                },
            }),
            3 => Ok(Self::Inp {
                result: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
            }),
            4 => Ok(Self::Out {
                parameter: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
            }),
            5 => Ok(Self::Jit {
                first_parameter: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
                second_parameter: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[2],
                },
            }),
            6 => Ok(Self::Jif {
                first_parameter: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
                second_parameter: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[2],
                },
            }),
            7 => Ok(Self::Lst {
                first_parameter: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
                second_parameter: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[2],
                },
                result: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[3],
                },
            }),
            8 => Ok(Self::Equ {
                first_parameter: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
                second_parameter: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[2],
                },
                result: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[3],
                },
            }),
            9 => Ok(Self::Arb {
                first_parameter: Parameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
            }),
            99 => Ok(Self::Halt {}),
            _ => Err(format!("Unknown instruction opcode [{instruction_code}]!")),
        }
    }
}

#[derive(PartialEq)]
enum ExecuteResult {
    Continue,
    Halt(i64),
    Panic(String),
    WaitingInput,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = run_machine_with_extended_memory(
            &advent_of_code::template::read_file_part("examples", DAY, 1),
            "",
        )
        .map(|(_, output)| output);

        let expected = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]
        .to_vec();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_one_2() {
        let result = run_machine_with_extended_memory(
            &advent_of_code::template::read_file_part("examples", DAY, 2),
            "",
        )
        .map(|(_, output)| output);

        let expected = [1219070632396864].to_vec();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_one_3() {
        let result = run_machine_with_extended_memory(
            &advent_of_code::template::read_file_part("examples", DAY, 3),
            "",
        )
        .map(|(_, output)| output);
        let expected = [1125899906842624].to_vec();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_one_4() {
        let result = run_machine_with_extended_memory(
            &advent_of_code::template::read_file_part("examples", DAY, 4),
            "1337",
        )
        .map(|(_, output)| output);
        let expected = [1337].to_vec();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_one_5() {
        let result = run_machine_with_extended_memory(
            &advent_of_code::template::read_file_part("examples", DAY, 5),
            "",
        )
        .map(|(_, output)| output);
        let expected = [-1].to_vec();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_one_6() {
        let result = run_machine_with_extended_memory(
            &advent_of_code::template::read_file_part("examples", DAY, 6),
            "",
        )
        .map(|(_, output)| output);
        let expected = [1].to_vec();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_one_7() {
        let result = run_machine_with_extended_memory(
            &advent_of_code::template::read_file_part("examples", DAY, 7),
            "",
        )
        .map(|(_, output)| output);
        let expected = [109].to_vec();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_one_8() {
        let result = run_machine_with_extended_memory(
            &advent_of_code::template::read_file_part("examples", DAY, 8),
            "",
        )
        .map(|(_, output)| output);
        let expected = [204].to_vec();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_one_9() {
        let result = run_machine_with_extended_memory(
            &advent_of_code::template::read_file_part("examples", DAY, 9),
            "",
        )
        .map(|(_, output)| output);
        let expected = [204].to_vec();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_one_10() {
        let result = run_machine_with_extended_memory(
            &advent_of_code::template::read_file_part("examples", DAY, 10),
            "",
        )
        .map(|(_, output)| output);
        let expected = [204].to_vec();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_one_11() {
        let result = run_machine_with_extended_memory(
            &advent_of_code::template::read_file_part("examples", DAY, 11),
            "1337",
        )
        .map(|(_, output)| output);
        let expected = [1337].to_vec();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_one_12() {
        let result = run_machine_with_extended_memory(
            &advent_of_code::template::read_file_part("examples", DAY, 12),
            "1337",
        )
        .map(|(_, output)| output);
        let expected = [1337].to_vec();
        assert_eq!(result, Some(expected));
    }
}
