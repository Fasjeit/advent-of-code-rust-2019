advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut memory: Vec<u64> = input
        .split(",")
        .map(|i| i.parse().expect("Expected u64 list"))
        .collect();

    memory[1] = 12;
    memory[2] = 2;

    let mut machine = Machine {
        memory,
        instruction_pointer: 0,
    };
    match machine.execute() {
        Ok(result) => Some(result),
        Err(m) => panic!("{}", m),
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    part_two_with_target(input, 19690720)
}

pub fn part_two_with_target(input: &str, target: u64) -> Option<u64> {
    let memory: Vec<u64> = input
        .split(",")
        .map(|i| i.parse().expect("Expected u64 list"))
        .collect();

    let machine = Machine {
        memory,
        instruction_pointer: 0,
    };

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut current_machine = machine.clone_with_noun_verb(noun, verb);
            if let Ok(result) = current_machine.execute() {
                if result == target {
                    return Some(100 * noun + verb);
                }
            }
        }
    }

    todo!()
}

pub fn part_one_no_replace(input: &str) -> Option<u64> {
    let memory: Vec<u64> = input
        .split(",")
        .map(|i| i.parse().expect("Expected u64 list"))
        .collect();

    let mut machine = Machine {
        memory,
        instruction_pointer: 0,
    };

    match machine.execute() {
        Ok(result) => Some(result),
        Err(m) => panic!("{}", m),
    }
}

struct Machine {
    memory: Vec<u64>,
    instruction_pointer: usize,
}

#[allow(clippy::let_and_return)]
impl Machine {
    // fn get_noun(&self) -> u64 {
    //     self.memory[1]
    // }

    // fn get_verb(&self) -> u64 {
    //     self.memory[2]
    // }

    fn clone_with_noun_verb(&self, noun: u64, verb: u64) -> Machine {
        let mut new_memory = self.memory.clone();
        new_memory[1] = noun;
        new_memory[2] = verb;

        Machine {
            memory: new_memory,
            instruction_pointer: 0,
        }
    }

    // execute until halt
    fn execute(&mut self) -> Result<u64, String> {
        let mut exe_result;
        loop {
            exe_result = self.execute_step();
            match exe_result {
                ExecuteResult::None => (), // go on
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

        let opcode = self.memory[self.instruction_pointer];
        let instruction = Instruction::from(opcode);

        let result = match instruction {
            Instruction::Add | Instruction::Mul => self.execute_operands_instruction(instruction),
            Instruction::Halt => ExecuteResult::Halt(self.memory[0]),
            _ => ExecuteResult::Panic(
                format!("Unsupported instruction [{:?}] executed!", instruction).to_string(),
            ),
        };

        result
    }

    fn execute_operands_instruction(&mut self, instruction: Instruction) -> ExecuteResult {
        if self.memory.len() < self.instruction_pointer + 3 {
            return ExecuteResult::Panic("Unexpected memory overflow!".to_string());
        }
        let first_operand_address = self.memory[self.instruction_pointer + 1];
        let second_operand_address = self.memory[self.instruction_pointer + 2];
        let result_operand_address = self.memory[self.instruction_pointer + 3];

        if first_operand_address as usize >= self.memory.len() {
            return ExecuteResult::Panic(
                format!("Unexpected memory for first operand address [{first_operand_address}]!")
                    .to_string(),
            );
        }
        if second_operand_address as usize >= self.memory.len() {
            return ExecuteResult::Panic(
                format!("Unexpected memory for second operand address [{second_operand_address}]!")
                    .to_string(),
            );
        }
        if result_operand_address as usize >= self.memory.len() {
            return ExecuteResult::Panic(
                format!("Unexpected memory for result operand address [{result_operand_address}]!")
                    .to_string(),
            );
        }

        let first_operand_value = self.memory[first_operand_address as usize];
        let second_operand_value = self.memory[second_operand_address as usize];

        let result = instruction.execute(first_operand_value, second_operand_value);
        self.memory[result_operand_address as usize] = result;

        self.instruction_pointer += 4;
        ExecuteResult::None
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("Instruction pointer C: {}", self.instruction_pointer);
        println!("Memory: {:?}", self.memory);
        println!();
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Instruction {
    Add,
    Mul,
    Halt,
    Reserved,
}

impl Instruction {
    fn execute(&self, first: u64, second: u64) -> u64 {
        match self {
            Instruction::Add => first + second,
            Instruction::Mul => first * second,
            _ => panic!("Cannot execute non-operand instruction [{:?}]!", self),
        }
    }
}

impl From<u64> for Instruction {
    fn from(value: u64) -> Self {
        match value {
            1 => Self::Add,
            2 => Self::Mul,
            99 => Self::Halt,
            _ => panic!("Unknown instruction opcode [{value}]!"),
        }
    }
}

#[derive(PartialEq)]
enum ExecuteResult {
    None,
    Halt(u64),
    Panic(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one_no_replace(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(3500));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one_no_replace(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_with_target(
            &advent_of_code::template::read_file_part("examples", DAY, 3),
            198,
        );
        assert_eq!(result, Some(4));
    }
}
