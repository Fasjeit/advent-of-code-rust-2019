use advent_of_code::intcode::Machine;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u64> {
    let firmware: Vec<i64> = input
        .split(",")
        .map(|i| i.parse().expect("Expected i64 list"))
        .collect();

    let ext_memory_size = 4096;
    let mut ext_memory: Vec<i64> = Vec::with_capacity(ext_memory_size);
    for i in 0..ext_memory_size {
        if i < firmware.len() {
            ext_memory.push(firmware[i]);
        } else {
            ext_memory.push(0);
        }
    }

    // Can jump over something and can land, jump len is 4.
    // (NOT(A) OR NOT(B) OR (NOT (C)) AND NOT (D)
    let input = "NOT A J\n NOT B T\n OR T J\n NOT C T\n OR T J\n AND D J\n WALK\n";

    let mut machine = Machine::new_with_ascii_input(ext_memory, input);
    machine.execute();
    let output = machine.output;

    Some(output[output.len() - 1] as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let firmware: Vec<i64> = input
        .split(",")
        .map(|i| i.parse().expect("Expected i64 list"))
        .collect();

    let ext_memory_size = 4096;
    let mut ext_memory: Vec<i64> = Vec::with_capacity(ext_memory_size);
    for i in 0..ext_memory_size {
        if i < firmware.len() {
            ext_memory.push(firmware[i]);
        } else {
            ext_memory.push(0);
        }
    }

    // 1. Can jump over something and can land, jump len is 4.
    // 2. Can jump further imidietly or going one step and jump - 8 is non empty, or 5 and 9 is non empty
    // 2.1 Or simply non empty 8 or non-empty 5 (don't need to check the second jump, we cannot compute the landing).
    // (NOT(A) OR NOT(B) OR (NOT (C)) AND NOT (D) AND (H OR E)
    let input = "NOT A T\n".to_owned() // part 1 but need AND!
        + "OR T J\n"
        + "NOT B T\n"
        + "OR T J\n"
        + "NOT C T\n"
        + "OR T J\n"
        + "AND D J\n"

        + "NOT H T\n" // see below
        + "AND H T\n" // place 0 at t
        + "OR H T\n" // PLACE H at T
        + "OR E T\n" // H OR E
        + "AND T J\n"
        + "RUN\n";

    let mut machine = Machine::new_with_ascii_input(ext_memory, &input);
    machine.execute();
    let output = machine.output;

    Some(output[output.len() - 1] as u64)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_part_one() {
//         let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, None);
//     }

//     #[test]
//     fn test_part_two() {
//         let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, None);
//     }
// }
