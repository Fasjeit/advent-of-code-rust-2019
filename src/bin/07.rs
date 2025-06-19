advent_of_code::solution!(7);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let (firmware, phase_inputs) = (lines.next().unwrap(), lines.next().unwrap_or("0,1,2,3,4"));
    Some(create_amplifiers(firmware, phase_inputs) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let (firmware, phase_inputs) = (lines.next().unwrap(), lines.next().unwrap_or("9,8,7,6,5"));
    Some(create_amplifiers_part_2(firmware, phase_inputs) as u64)
}

pub fn create_amplifiers(memory: &str, phase_inputs: &str) -> i64 {
    let memory: Vec<i64> = memory
        .split(",")
        .map(|i| i.parse().expect("Expected i64 list"))
        .collect();

    let phase_inputs: Vec<i64> = phase_inputs
        .split(",")
        .map(|i| i.parse().expect("Expected i64 list"))
        .collect();

    let mut max = 0;

    for perm in phase_inputs
        .iter()
        .permutations(phase_inputs.len())
        .unique()
    {
        //println!("{:?}", perm);

        let mut amps: [Machine; 5] = [
            Machine::new(memory.clone()),
            Machine::new(memory.clone()),
            Machine::new(memory.clone()),
            Machine::new(memory.clone()),
            Machine::new(memory.clone()),
        ];
        amps[0].input = [*perm[0], 0].to_vec();
        let _ = amps[0].execute();
        let mut res = amps[0].output[0];

        amps[1].input = [*perm[1], res].to_vec();
        let _ = amps[1].execute();
        res = amps[1].output[0];

        amps[2].input = [*perm[2], res].to_vec();
        let _ = amps[2].execute();
        res = amps[2].output[0];

        amps[3].input = [*perm[3], res].to_vec();
        let _ = amps[3].execute();
        res = amps[3].output[0];

        amps[4].input = [*perm[4], res].to_vec();
        let _ = amps[4].execute();
        res = amps[4].output[0];

        max = if res > max { res } else { max }
    }

    max
}

pub fn create_amplifiers_part_2(memory: &str, phase_inputs: &str) -> i64 {
    let memory: Vec<i64> = memory
        .split(",")
        .map(|i| i.parse().expect("Expected i64 list"))
        .collect();

    let phase_inputs: Vec<i64> = phase_inputs
        .split(",")
        .map(|i| i.parse().expect("Expected i64 list"))
        .collect();

    let mut max = 0;

    for perm in phase_inputs
        .iter()
        .permutations(phase_inputs.len())
        .unique()
    {
        println!("{:?}", perm);

        let mut amps: [Machine; 5] = [
            Machine::new(memory.clone()),
            Machine::new(memory.clone()),
            Machine::new(memory.clone()),
            Machine::new(memory.clone()),
            Machine::new(memory.clone()),
        ];

        let mut exited = false;
        let mut input_value = 0;

        while !exited {
            println!("New amp cycle!");
            for amp in &mut amps {
                amp.input = [*perm[0], input_value].to_vec();
                let result = amp.execute();
                input_value = *amp.output.last().unwrap();

                if let Ok(result) = result {
                    if result != -1 {
                        // -1 is waiting for input
                        exited = true;
                        break;
                    } else {
                        println!("Machine is waiting input!");
                    }
                }
            }
        }

        let res = input_value;

        max = if res > max { res } else { max };
    }

    max
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
        Ok(result) => Some((result, machine.output)),
        Err(m) => panic!("{}", m),
    }
}

struct Machine {
    memory: Vec<i64>,
    instruction_pointer: usize,

    input: Vec<i64>,
    input_pointer: usize,

    output: Vec<i64>,
    output_pointer: usize,
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
        }
    }

    fn dump(&self) -> String {
        let memory_dump = format!("MEM: {:?}", self.memory);
        let ins_p_dump = format!("IP: {}", self.instruction_pointer);
        let input_dump = format!("INP: {:?}", self.input);
        let inp_p_dump = format!("INP_P: {}", self.input_pointer);
        let output_dump = format!("OUT: {:?}", self.output);
        let otp_p_dump = format!("OUT_P: {}", self.output_pointer);

        format!(
            " ======\n {memory_dump}\n {ins_p_dump}\n {input_dump} \n {inp_p_dump}\n {output_dump} \n {otp_p_dump}"
        )
        .to_string()
    }

    fn get_parameter_value(&self, parameter: &InputParameter) -> i64 {
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
        }
    }

    fn get_output_parameter_address(&self, parameter: &OutputParameter) -> i64 {
        let addr = parameter.value as usize;
        if addr >= self.memory.len() {
            panic!("Unexpected memory address for parameter address [{addr}]!");
        } else {
            parameter.value
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
    fn execute(&mut self) -> Result<i64, String> {
        //self.print();

        let mut exe_result;
        loop {
            exe_result = self.execute_step();
            match exe_result {
                ExecuteResult::Continue => (),
                ExecuteResult::Panic(message) => return Result::Err(message),
                ExecuteResult::Halt(result) => {
                    //machine.print();
                    return Ok(result);
                }
                ExecuteResult::WaitingInput => return Ok(-1),
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
        first_parameter: InputParameter,
        second_parameter: InputParameter,
        result: OutputParameter,
    },
    Mul {
        first_parameter: InputParameter,
        second_parameter: InputParameter,
        result: OutputParameter,
    },
    Inp {
        result: OutputParameter,
    },
    Out {
        parameter: InputParameter,
    },
    Jit {
        first_parameter: InputParameter,
        second_parameter: InputParameter,
    },
    Jif {
        first_parameter: InputParameter,
        second_parameter: InputParameter,
    },
    Lst {
        first_parameter: InputParameter,
        second_parameter: InputParameter,
        result: OutputParameter,
    },
    Equ {
        first_parameter: InputParameter,
        second_parameter: InputParameter,
        result: OutputParameter,
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
                first_parameter: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
                second_parameter: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[2],
                },
                result: OutputParameter { value: memory[3] },
            }),
            2 => Ok(Self::Mul {
                first_parameter: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
                second_parameter: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[2],
                },
                result: OutputParameter { value: memory[3] },
            }),
            3 => Ok(Self::Inp {
                result: OutputParameter { value: memory[1] },
            }),
            4 => Ok(Self::Out {
                parameter: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
            }),
            5 => Ok(Self::Jit {
                first_parameter: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
                second_parameter: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[2],
                },
            }),
            6 => Ok(Self::Jif {
                first_parameter: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
                second_parameter: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[2],
                },
            }),
            7 => Ok(Self::Lst {
                first_parameter: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
                second_parameter: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[2],
                },
                result: OutputParameter { value: memory[3] },
            }),
            8 => Ok(Self::Equ {
                first_parameter: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[1],
                },
                second_parameter: InputParameter {
                    mode: parameter_modes_iter
                        .next()
                        .unwrap_or(ParameterMode::Position),
                    value: memory[2],
                },
                result: OutputParameter { value: memory[3] },
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
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(43210));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(54321));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(65210));
    }

    #[test]
    fn test_part_one_1_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 11,
        ));
        assert_eq!(result, Some(43210));
    }

    #[test]
    fn test_part_one_2_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 21,
        ));
        assert_eq!(result, Some(54321));
    }

    #[test]
    fn test_part_one_3_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 31,
        ));
        assert_eq!(result, Some(65210));
    }

    //#[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(139629729));
    }
}
