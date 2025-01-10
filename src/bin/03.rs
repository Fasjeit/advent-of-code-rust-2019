use advent_of_code::advent_stdlib::{Direction, Index};
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let second = lines.next().unwrap();

    // bigger than any posible negative index values in data.
    let starting_pos = Index {
        x: 1000000,
        y: 1000000,
    };

    let first_visited = parse_line_data_part_1(first, starting_pos);
    let second_visited = parse_line_data_part_1(second, starting_pos);

    let mut crosses = Vec::new();
    first_visited.iter().for_each(|f| {
        if second_visited.contains(f) {
            crosses.push(f);
        }
    });

    let mut min = u64::MAX;
    for x in &crosses {
        min = min.min(manhattan_diff(x, &starting_pos));
    }

    // dbg!(&crosses);
    // dbg!(min);

    Some(min)
}

pub fn part_two(input: &str) -> Option<u64> {
    // same as part 1 but with different cost computation.

    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let second = lines.next().unwrap();

    // bigger than any posible negative index values in data.
    let starting_pos = Index {
        x: 1000000,
        y: 1000000,
    };

    let first_visited = parse_line_data_part_2(first, starting_pos);
    let second_visited = parse_line_data_part_2(second, starting_pos);

    // Index - cost
    let mut crosses = Vec::new();
    first_visited.iter().for_each(|f| {
        if second_visited.contains_key(f.0) {
            crosses.push((f.0, f.1 + second_visited[f.0]));
        }
    });

    let mut min = u64::MAX;
    for x in &crosses {
        min = min.min(x.1);
    }

    //dbg!(&first_visited);
    //dbg!(&second_visited);
    //dbg!(&crosses);
    // dbg!(min);

    Some(min)
}

fn manhattan_diff(a: &Index, b: &Index) -> u64 {
    (a.x as u64).abs_diff(b.x as u64) + (a.y as u64).abs_diff(b.y as u64)
}

fn parse_line_data_part_1(input: &str, starting_pos: Index) -> HashSet<Index> {
    let mut visited = HashSet::new();
    let mut current_pos = starting_pos;

    let splitted = input.split(",");
    for movement in splitted {
        let movement = movement.to_string();
        let (direction, len) = movement.split_at(1);
        let len: u64 = len.parse().expect("Expected u64 len");

        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => panic!("Unknown direction [{direction}]"),
        };

        for _i in 0..len {
            current_pos = current_pos.navigate_to_no_matrix(&direction);
            visited.insert(current_pos);
        }
    }

    visited
}

fn parse_line_data_part_2(input: &str, starting_pos: Index) -> HashMap<Index, u64> {
    // Index - path cost to index
    let mut visited = HashMap::new();
    let mut current_pos = starting_pos;

    let splitted = input.split(",");

    let mut current_path_cost = 0;
    for movement in splitted {
        let movement = movement.to_string();
        let (direction, len) = movement.split_at(1);
        let len: u64 = len.parse().expect("Expected u64 len");

        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => panic!("Unknown direction [{direction}]"),
        };

        for _i in 0..len {
            current_path_cost += 1;
            current_pos = current_pos.navigate_to_no_matrix(&direction);
            visited.entry(current_pos).or_insert(current_path_cost);
        }
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(159));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(135));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(610));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(410));
    }
}
