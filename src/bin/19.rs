use advent_of_code::advent_stdlib::Direction;
use advent_of_code::advent_stdlib::Index;
use advent_of_code::advent_stdlib::Matrix;
use advent_of_code::advent_stdlib::Size;
use advent_of_code::intcode::*;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Display;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u64> {
    let firmware: Vec<i64> = input
        .split(",")
        .map(|i| i.parse().expect("Expected i64 list"))
        .collect();

    let ext_memory_size = 512;
    let mut ext_memory: Vec<i64> = vec![0; ext_memory_size];
    ext_memory[..firmware.len()].copy_from_slice(&firmware[..]);

    let dim_size = 50;

    let map_data = vec![
        MapCell {
            has_traction: false
        };
        dim_size * dim_size
    ];
    let mut map = Matrix::<MapCell> {
        data: map_data,
        size: Size {
            x: dim_size,
            y: dim_size,
        },
    };

    for possition in 0..dim_size * dim_size {
        let index = map.get_index_from_position(possition);

        let input = vec![index.x as i64, index.y as i64];
        let mut machine = Machine::new_with_input(ext_memory.clone(), input);
        machine.execute();
        let output = machine.output;

        if output[0] == 1 {
            map[index].has_traction = true;
        }
    }

    //map.print();

    let result = map.data.iter().filter(|c| c.has_traction).count();
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    // see part_two_dbg for comments.

    let firmware: Vec<i64> = input
        .split(",")
        .map(|i| i.parse().expect("Expected i64 list"))
        .collect();

    let ext_memory_size = 512;
    let mut ext_memory: Vec<i64> = vec![0; ext_memory_size];
    ext_memory[..firmware.len()].copy_from_slice(&firmware[..]);

    // Got by looking at beam output.
    let beam_start_pos = Index { x: 4, y: 3 };
    //map[beam_start_pos].has_traction = true;

    let max_iterations = 10_000;
    let mut queue = VecDeque::new();

    let target_box_size = 100;
    queue.push_front(beam_start_pos);

    let mut iteration = 0;

    while let Some(index) = queue.pop_back() {
        iteration += 1;
        if iteration > max_iterations {
            break;
        }

        if let Some(box_x) = index.x.checked_sub(target_box_size - 1) {
            let box_pos = Index {
                x: box_x,
                y: index.y + target_box_size - 1,
            };

            if check_index_has_traction(box_pos, ext_memory.clone()) {
                // return upper-left box position.
                return Some((box_x * 10_000 + index.y) as u64);
            }
        }

        let first_chec_index = Index {
            x: index.x + 2,
            y: index.y + 1,
        };

        let input = vec![first_chec_index.x as i64, first_chec_index.y as i64];
        let mut machine = Machine::new_with_input(ext_memory.clone(), input);
        machine.execute();
        if machine.output[0] == 1 {
            queue.push_front(first_chec_index);
        } else {
            let second_check_index = Index {
                x: index.x + 1,
                y: index.y + 1,
            };
            let input = vec![second_check_index.x as i64, second_check_index.y as i64];
            let mut machine = Machine::new_with_input(ext_memory.clone(), input);
            machine.execute();
            if machine.output[0] == 1 {
                queue.push_front(second_check_index);
            }
        }
    }
    panic!("No solution found!")
}

#[allow(dead_code)]
fn part_two_dbg(input: &str, beam_start_pos: Index) -> Option<u64> {
    // Looking at the output we see that "right" beam border always moves
    // down (y+1) and right (x+1 or x+2).
    // So we can only traverse on any border.
    // "Right" border has check priority x+2 over x+1 (outer right might not exist, while inner - exists)
    // "Left" border has check priority x+1 over x+2 (outer left might not exist, while inner - exists)
    // The code below just iterate over "Right" border, checking, if a box can fit and then checking next border position.
    // "Left" border used only as debug.
    // Queue only used when two directions being traversed during debug.

    let firmware: Vec<i64> = input
        .split(",")
        .map(|i| i.parse().expect("Expected i64 list"))
        .collect();

    let ext_memory_size = 512;
    let mut ext_memory: Vec<i64> = vec![0; ext_memory_size];
    ext_memory[..firmware.len()].copy_from_slice(&firmware[..]);

    //let dim_size = 10_000;
    // let map_data = vec![
    //     MapCell {
    //         has_traction: false
    //     };
    //     dim_size * dim_size
    // ];
    // let mut map = Matrix::<MapCell> {
    //     data: map_data,
    //     size: Size {
    //         x: dim_size,
    //         y: dim_size,
    //     },
    // };

    //map[beam_start_pos].has_traction = true;

    let max_iterations = 10_000;
    let mut queue = VecDeque::new();

    let target_box_size = 100;

    //queue.push_front((beam_start_pos, Direction::Left));
    queue.push_front((beam_start_pos, Direction::Right));

    let mut iteration = 0;

    while let Some((index, direction)) = queue.pop_back() {
        //dbg!(iteration);

        iteration += 1;
        if iteration > max_iterations {
            break;
        }

        // check box fit
        // upper right - on border
        // lower left - inside
        //................#################.......
        //.................########OOOOOOOOOO.....
        //..................#######OOOOOOOOOO#....
        //...................######OOOOOOOOOO###..
        //....................#####OOOOOOOOOO#####
        //.....................####OOOOOOOOOO#####
        //.....................####OOOOOOOOOO#####
        //......................###OOOOOOOOOO#####
        //.......................##OOOOOOOOOO#####
        //........................#OOOOOOOOOO#####
        //.........................OOOOOOOOOO#####
        //..........................##############

        if let Some(box_x) = index.x.checked_sub(target_box_size - 1) {
            let box_pos = Index {
                x: box_x,
                y: index.y + target_box_size - 1,
            };

            if check_index_has_traction(box_pos, ext_memory.clone()) {
                // return upper-left box position.
                return Some((box_x * 10_000 + index.y) as u64);
            }
        }

        let first_chec_index = Index {
            x: if direction == Direction::Left {
                index.x + 1
            } else {
                index.x + 2
            },
            y: index.y + 1,
        };

        let input = vec![first_chec_index.x as i64, first_chec_index.y as i64];
        let mut machine = Machine::new_with_input(ext_memory.clone(), input);
        machine.execute();
        if machine.output[0] == 1 {
            //map[first_chec_index].has_traction = true;
            queue.push_front((first_chec_index, direction));
            //dbg!(first_chec_index);
        } else {
            let second_check_index = Index {
                x: if direction == Direction::Left {
                    index.x + 2
                } else {
                    index.x + 1
                },
                y: index.y + 1,
            };
            let input = vec![second_check_index.x as i64, second_check_index.y as i64];
            let mut machine = Machine::new_with_input(ext_memory.clone(), input);
            machine.execute();
            if machine.output[0] == 1 {
                //map[second_check_index].has_traction = true;
                queue.push_front((second_check_index, direction));
                //dbg!(second_check_index);
            }
        }
    }

    //map.print_part(Index { x: 50, y: 30 }, Index { x: 250, y: 230 });

    panic!("No solution found!")
}

fn check_index_has_traction(index: Index, ext_machine_memory: Vec<i64>) -> bool {
    let input = vec![index.x as i64, index.y as i64];
    let mut machine = Machine::new_with_input(ext_machine_memory, input);
    machine.execute();
    machine.output[0] == 1
}

#[derive(Debug, Clone)]
pub struct MapCell {
    has_traction: bool,
}

impl MapCell {
    pub fn new(has_traction: bool) -> Self {
        MapCell { has_traction }
    }
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            '#' => MapCell::new(true),
            _ => MapCell {
                has_traction: false,
            },
        }
    }
}

impl Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = if self.has_traction { '#' } else { '.' };
        write!(f, "{}", ch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test beam is not a real beam
    // x < y < 2x
    // Only "Right" border has the same behavior as the real one (but simplier).
    // Also the starting beam pos is different.

    // ..................................................
    // ..................................................
    // ..................................................
    // ..#...............................................
    // ...#..............................................
    // ...##.............................................
    // ....##............................................
    // ....###...........................................
    // .....###..........................................
    // .....####.........................................
    // ......####........................................
    // ......#####.......................................
    // .......#####......................................

    #[test]
    fn test_custom_intcode_beam_y_l_x() {
        let input = &advent_of_code::template::read_file("examples", DAY);

        let firmware: Vec<i64> = input
            .split(",")
            .map(|i| i.parse().expect("Expected i64 list"))
            .collect();

        let ext_memory_size = 512;
        let mut ext_memory: Vec<i64> = vec![0; ext_memory_size];
        ext_memory[..firmware.len()].copy_from_slice(&firmware[..]);

        let test_pos = Index { x: 4, y: 3 };
        let inside_beam = check_index_has_traction(test_pos, ext_memory);

        assert!(!inside_beam);
    }

    #[test]
    fn test_custom_intcode_beam_y_g_x_l_3x() {
        let input = &advent_of_code::template::read_file("examples", DAY);

        let firmware: Vec<i64> = input
            .split(",")
            .map(|i| i.parse().expect("Expected i64 list"))
            .collect();

        let ext_memory_size = 512;
        let mut ext_memory: Vec<i64> = vec![0; ext_memory_size];
        ext_memory[..firmware.len()].copy_from_slice(&firmware[..]);

        let test_pos = Index { x: 3, y: 4 };
        let inside_beam = check_index_has_traction(test_pos, ext_memory);

        assert!(inside_beam);
    }

    #[test]
    fn test_custom_intcode_beam_y_g_3x() {
        let input = &advent_of_code::template::read_file("examples", DAY);

        let firmware: Vec<i64> = input
            .split(",")
            .map(|i| i.parse().expect("Expected i64 list"))
            .collect();

        let ext_memory_size = 512;
        let mut ext_memory: Vec<i64> = vec![0; ext_memory_size];
        ext_memory[..firmware.len()].copy_from_slice(&firmware[..]);

        let test_pos = Index { x: 3, y: 12 };
        let inside_beam = check_index_has_traction(test_pos, ext_memory);

        assert!(!inside_beam);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(576));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_dbg(
            &advent_of_code::template::read_file("examples", DAY),
            Index { x: 2, y: 3 },
        );
        assert_eq!(result, Some(2000300));
    }
}
