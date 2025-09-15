use advent_of_code::advent_stdlib::Direction;
use advent_of_code::advent_stdlib::Index;
use advent_of_code::advent_stdlib::Matrix;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u64> {
    solve_part_1(input, 26)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

pub fn solve_part_1(input: &str, target_key_count: usize) -> Option<u64> {
    let mut map = Matrix::<MapCell>::from_char_input(input);
    map.print();

    let robot_pos = map
        .data
        .iter()
        .enumerate()
        .find(|e| e.1.has_robot)
        .unwrap()
        .0;

    let start_index = map.get_index_from_position(robot_pos);

    let mut to_visit_set: BinaryHeap<Reverse<CostState>> = BinaryHeap::new();
    to_visit_set.push(Reverse(CostState::new(&(
        0_u64,
        start_index,
        vec![false; target_key_count],
    ))));
    let result = pseudo_dijkstra(&mut map, None, &mut to_visit_set);
    //map.print();

    //let test_index = Index { x: 1, y: 1 };
    //dbg!(&map[test_index.y][test_index.x].cost);

    result
}

#[derive(PartialEq, Eq)]
struct CostState {
    cost: u64,
    index: Index,
    state: Vec<bool>,
}

impl CostState {
    fn new(data: &(u64, Index, Vec<bool>)) -> Self {
        CostState {
            cost: data.0,
            index: data.1,
            state: data.2.clone(),
        }
    }
}

impl Ord for CostState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for CostState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_ascii_distance_from_a(c: char) -> usize {
    c as usize - 'a' as usize
}

fn pseudo_dijkstra(
    matrix: &mut Matrix<MapCell>,
    ending_position: Option<&Index>,
    to_visit_set: &mut BinaryHeap<Reverse<CostState>>,
) -> Option<u64> {
    // Usage:
    // let mut to_visit_set = BinaryHeap::new();
    // to_visit_set.push(Reverse((0_u64, start_index)));
    // let result = pseudo_dijkstra(&mut robot.map, Some(&end_index), &mut to_visit_set);
    // or without end_index, if need to visit all cells.

    let mut safe_counter = 1_000_000;

    while let Some(Reverse(cost_state)) = to_visit_set.pop() {
        if safe_counter <= 0 {
            panic!("Safe counter stop.");
        }
        safe_counter -= 1;

        let index = cost_state.index;
        let cost = cost_state.cost;
        let mut state = cost_state.state;

        //dbg!(&index);

        // if matrix[index.y][index.x].cost != u64::MAX {
        //     assert!(matrix[index.y][index.x].cost <= cost);
        //     continue;
        // }

        if let Some(char) = matrix[index.y][index.x].door_or_key {
            if char.is_lowercase() {
                // lowercase are doors

                // also update the cost for old state to make lookups work properly
                matrix[index.y][index.x].cost.insert(state.clone(), cost);
                // update the state
                state[get_ascii_distance_from_a(char)] = true;
            }
        }

        // update the cost for current state
        matrix[index.y][index.x].cost.insert(state.clone(), cost);

        if state.iter().all(|k| *k) {
            // collected all keys!
            return Some(cost);
        }

        //dbg!(&index);

        if let Some(ending_position) = ending_position {
            if index == *ending_position {
                return Some(cost);
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Down) {
            if !matrix[next_index.y][next_index.x].has_wall(&state)
                && (!matrix[next_index.y][next_index.x].cost.contains_key(&state)
                    || matrix[next_index.y][next_index.x].cost[&state] > (cost + 1))
            {
                to_visit_set.push(Reverse(CostState::new(&(
                    cost + 1,
                    next_index,
                    state.clone(),
                ))));
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Right) {
            if !matrix[next_index.y][next_index.x].has_wall(&state)
                && (!matrix[next_index.y][next_index.x].cost.contains_key(&state)
                    || matrix[next_index.y][next_index.x].cost[&state] > (cost + 1))
            {
                to_visit_set.push(Reverse(CostState::new(&(
                    cost + 1,
                    next_index,
                    state.clone(),
                ))));
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Left) {
            if !matrix[next_index.y][next_index.x].has_wall(&state)
                && (!matrix[next_index.y][next_index.x].cost.contains_key(&state)
                    || matrix[next_index.y][next_index.x].cost[&state] > (cost + 1))
            {
                to_visit_set.push(Reverse(CostState::new(&(
                    cost + 1,
                    next_index,
                    state.clone(),
                ))));
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Up) {
            if !matrix[next_index.y][next_index.x].has_wall(&state)
                && (!matrix[next_index.y][next_index.x].cost.contains_key(&state)
                    || matrix[next_index.y][next_index.x].cost[&state] > (cost + 1))
            {
                to_visit_set.push(Reverse(CostState::new(&(
                    cost + 1,
                    next_index,
                    state.clone(),
                ))));
            }
        }
    }

    None
}

#[derive(Debug, Clone)]
pub struct MapCell {
    has_wall: bool,
    has_robot: bool,
    door_or_key: Option<char>,
    cost: HashMap<Vec<bool>, u64>,
}

impl MapCell {
    pub fn new(has_wall: bool) -> Self {
        MapCell {
            has_wall,
            has_robot: false,
            door_or_key: None,
            cost: HashMap::new(),
        }
    }

    pub fn new_robot() -> Self {
        MapCell {
            has_wall: false,
            has_robot: true,
            door_or_key: None,
            cost: HashMap::new(),
        }
    }

    pub fn has_wall(&self, state: &Vec<bool>) -> bool {
        if self.has_wall {
            true
        } else {
            // check keys
            if let Some(id) = self.door_or_key {
                if id.is_ascii_uppercase() {
                    // door!
                    // check the key
                    let index = get_ascii_distance_from_a(id.to_lowercase().next().unwrap());

                    if !state[index] {
                        return true;
                    }
                }
            }
            false
        }
    }
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            '.' => MapCell::new(false),
            '#' => MapCell::new(true),
            '@' => MapCell::new_robot(),
            _ => MapCell {
                has_wall: false,
                has_robot: false,
                door_or_key: Some(value),
                cost: HashMap::new(),
            },
        }
    }
}

impl Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = if self.has_robot {
            '@'
        } else if !self.has_wall {
            // if self.cost != u64::MAX {
            //     char::from_digit((self.cost % 10) as u32, 10).unwrap()
            // } else {
            self.door_or_key.unwrap_or('.')
            //}
        } else if self.has_wall {
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
    fn test_part_one_1() {
        let result = solve_part_1(
            &advent_of_code::template::read_file_part("examples", DAY, 1),
            2,
        );
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_one_2() {
        let result = solve_part_1(
            &advent_of_code::template::read_file_part("examples", DAY, 2),
            6,
        );
        assert_eq!(result, Some(86));
    }

    #[test]
    fn test_part_one_3() {
        let result = solve_part_1(
            &advent_of_code::template::read_file_part("examples", DAY, 3),
            7,
        );
        assert_eq!(result, Some(132));
    }

    #[test]
    fn test_part_one_4() {
        let result = solve_part_1(
            &advent_of_code::template::read_file_part("examples", DAY, 4),
            16,
        );
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_one_5() {
        let result = solve_part_1(
            &advent_of_code::template::read_file_part("examples", DAY, 5),
            9,
        );
        assert_eq!(result, Some(81));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
