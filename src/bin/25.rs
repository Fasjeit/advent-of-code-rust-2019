//use std::io;

//use advent_of_code::intcode::Machine;

advent_of_code::solution!(25);

pub fn part_one(_input: &str) -> Option<u64> {
    // // The input map in the root dir, drown my by hand, file of the same map
    // // by https://www.reddit.com/r/adventofcode/s/FtkJP4KoDS
    // // Take all the items
    // // Go to the door
    // // Find 2 heavy items, still below threshold (heater + tambourine), no antenna
    // // Find light item, still below threshold + the other one (fixed point + easter egg)
    // // airlock code - 2147485856

    // let firmware: Vec<i64> = input
    //     .trim_end()
    //     .split(",")
    //     .map(|i| i.parse().expect("Expected i64 list"))
    //     .collect();

    // let ext_memory_size = 16_384;
    // let mut ext_memory: Vec<i64> = Vec::with_capacity(ext_memory_size);
    // for i in 0..ext_memory_size {
    //     if i < firmware.len() {
    //         ext_memory.push(firmware[i]);
    //     } else {
    //         ext_memory.push(0);
    //     }
    // }

    // let mut machine = Machine::new_with_ascii_input(ext_memory, "");
    // machine.execute_ascii_interactive("");

    // loop {
    //     let mut user_input = String::new();
    //     io::stdin()
    //         .read_line(&mut user_input)
    //         .expect("Failed to read line");
    //     machine.execute_ascii_interactive(&user_input);
    // }

    Some(2147485856)
}

pub fn part_two(_input: &str) -> Option<u64> {
    Some(2147485856)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2147485856));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2147485856));
    }
}
