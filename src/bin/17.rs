use advent_of_code::advent_stdlib::Direction;
use advent_of_code::advent_stdlib::Matrix;
use advent_of_code::intcode::*;
use std::fmt::Debug;
use std::fmt::Display;

advent_of_code::solution!(17);

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

    let input = Vec::<i64>::new();

    let mut machine = Machine::new_with_input(ext_memory, input);
    machine.execute();
    let output_string = machine.get_ascii_output();

    let mut map = Matrix::<MapCell>::from_char_input(&output_string);
    // trim last line by changing size
    map.size.y -= 1;

    //map.print();

    let result = compute_parameters(map);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    // path by hand - going forward until space, then turn the only possible way
    // Possible to create simple simulator alg to create paths for general case.
    // see build_path for such an algo.
    // function splitting is still done by hand here.

    /*
    a(R4 R10 R8 R4) c(R10 R6 R4) a(R4 R10 R8 R4) c(R10 R6 R4) b(R4 L12 R6 L12)
    c(R10 R6 R4) b(R4 L12 R6 L12) a(R4 R10 R8 R4) c(R10 R6 R4) b(R4 L12 R6 L12)

    A: R4 R10 R8 R4
    B: R10 R6 R4 R4 L12 R6 L12
    C: R10 R6 R4
     */
    let main_routine = "A,C,A,C,B,C,B,A,C,B\n";
    let function_a = "R,4,R,10,R,8,R,4\n";
    let function_b = "R,4,L,12,R,6,L,12\n";
    let function_c = "R,10,R,6,R,4\n";
    let no = "n\n";

    let mut firmware: Vec<i64> = input
        .split(",")
        .map(|i| i.parse().expect("Expected i64 list"))
        .collect();

    firmware[0] = 2;

    let ext_memory_size = 4096;
    let mut ext_memory: Vec<i64> = Vec::with_capacity(ext_memory_size);
    for i in 0..ext_memory_size {
        if i < firmware.len() {
            ext_memory.push(firmware[i]);
        } else {
            ext_memory.push(0);
        }
    }

    let input = main_routine.to_owned() + function_a + function_b + function_c + no;
    let mut machine = Machine::new_with_ascii_input(ext_memory, &input);

    // run

    if let ExecuteResult::Halt(_a) = machine.execute() {
        // Machine also outputs input data and final feed.
        // if error - machine will output string with it.

        //let output_string = machine.get_ascii_output();
        //println!("{}", output_string);

        Some(machine.output[machine.output.len() - 1] as u64)
    } else {
        panic!("Unexpected execution result!")
    }
}

#[allow(dead_code)]
enum TurnDirection {
    Left,
    Right,
}

#[allow(dead_code)]
fn build_path(map: Matrix<MapCell>) -> String {
    let mut result = "".to_string();

    let mut dir = Direction::Up;
    let pos_index = map.data.iter().position(|c| c.has_robot).unwrap();
    let mut pos = map.get_index_from_position(pos_index);

    let mut curr_forward_counter = 0;
    let mut last_turn_direction = None;

    let mut max_loop_counter = 1000;

    loop {
        max_loop_counter -= 1;
        if max_loop_counter <= 0 {
            panic!("INF LOOP");
        }

        if let Some(next_pos) = pos.navigate_to(&map, &dir) {
            if !map[next_pos.y][next_pos.x].empty_space {
                // move forward
                curr_forward_counter += 1;
                pos = next_pos;

                //dbg!("forward!");

                continue;
            }
        }

        // Cannot go further - save and rotate!
        // If we have rotated before - print it, if first rotation - skip.
        let dir_str = match last_turn_direction {
            Some(TurnDirection::Left) => Some("L"),
            Some(TurnDirection::Right) => Some("R"),
            None => None,
        };
        if let Some(str) = dir_str {
            result = format!("{},{},{}", result, str, curr_forward_counter);
        }
        curr_forward_counter = 0;

        if let Some(right_pos) = pos.navigate_to(&map, &dir.turn_right()) {
            if !map[right_pos.y][right_pos.x].empty_space {
                dir = dir.turn_right();
                last_turn_direction = Some(TurnDirection::Right);
                //dbg!("right!");
                continue;
            }
        }
        if let Some(left_pos) = pos.navigate_to(&map, &dir.turn_left()) {
            if !map[left_pos.y][left_pos.x].empty_space {
                dir = dir.turn_left();
                last_turn_direction = Some(TurnDirection::Left);
                //dbg!("left!");
                continue;
            }
        }
        // skip first ','
        return result[1..].to_string();
    }
}

fn compute_parameters(map: Matrix<MapCell>) -> u64 {
    let mut total = 0;
    for y in 1..map.size.y - 1 {
        for x in 1..map.size.x - 1 {
            // check if intersection
            if !map[y][x].empty_space
                && !map[y - 1][x].empty_space
                && !map[y][x - 1].empty_space
                && !map[y + 1][x].empty_space
                && !map[y][x + 1].empty_space
            {
                total += y * x;
            }
        }
    }
    total as u64
}

#[derive(Debug, Clone)]
pub struct MapCell {
    pub empty_space: bool,
    has_robot: bool,
}

impl MapCell {
    pub fn new(empty_space: bool) -> Self {
        MapCell {
            empty_space,
            has_robot: false,
        }
    }

    pub fn new_robot() -> Self {
        MapCell {
            empty_space: false,
            has_robot: true,
        }
    }
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            '.' => MapCell::new(true),
            '#' => MapCell::new(false),
            '^' => MapCell::new_robot(),
            _ => panic!("Unknown char [{value}] in map data!"),
        }
    }
}

impl Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = if self.has_robot {
            '^'
        } else if self.empty_space {
            '.'
        } else if !self.empty_space {
            '#'
        } else {
            panic!()
        };
        write!(f, "{}", ch)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        let map = Matrix::<MapCell>::from_char_input(input);
        let result = compute_parameters(map);
        assert_eq!(result, 76);
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        let map = Matrix::<MapCell>::from_char_input(input);
        let result = build_path(map);
        assert_eq!(
            result,
            "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2"
        );
    }
}
