use advent_of_code::advent_stdlib::Direction;
use advent_of_code::advent_stdlib::Index;
use advent_of_code::advent_stdlib::Matrix;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Display;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u64> {
    // The main idea is pre compute shortest paths from key to key
    // using BFS, keeping track of required keys (finding keys for the shortest path only, which seems ok for a task).
    // Then create a new graph and traverse with Dijkstera respecting
    // closed doors and saving collected keys for a state we are in.
    // some ideas taken from https://www.reddit.com/r/adventofcode/comments/ec8090/comment/fhiwgh4
    solve_part_1(input, 26)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve_part_2(input, 26)
}

pub fn solve_part_1(input: &str, target_key_count: usize) -> Option<u64> {
    let map = Matrix::<MapCell>::from_char_input(input);
    //map.print();

    let robot_pos = map
        .data
        .iter()
        .enumerate()
        .find(|e| e.1.has_robot.is_some())
        .unwrap()
        .0;

    let start_index = map.get_index_from_position(robot_pos);
    let graph = build_key_graph(&map, start_index);

    let min_steps = search_graph(&graph, '@', target_key_count);
    Some(min_steps)
}

pub fn solve_part_2(input: &str, target_key_count: usize) -> Option<u64> {
    // Same as part 1, but storing 4-state and travesing each robot in each state.
    // Also making vec<bool> for doors and keys just u32.

    let mut map = Matrix::<MapCell>::from_char_input(input);

    // Patch the map
    let center_pos = map
        .data
        .iter()
        .enumerate()
        .find(|e| e.1.has_robot.is_some())
        .unwrap()
        .0;

    // Center and ortogonal - walls
    let center_index = map.get_index_from_position(center_pos);
    map[center_index] = MapCell::new(true);

    for dir in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        if let Some(index) = center_index.navigate_to(&map, &dir) {
            map[index.y][index.x] = MapCell::new(true);
        }
    }

    // Diagonals - new robots
    let labels = ['@', '$', '%', '&'];
    let mut start_indexes = Vec::new();

    let index_0 = Index {
        y: center_index.y - 1,
        x: center_index.x - 1,
    };
    map[index_0] = MapCell::new_robot_with_name(labels[0]);
    start_indexes.push(index_0);
    let index_1 = Index {
        y: center_index.y + 1,
        x: center_index.x - 1,
    };
    map[index_1] = MapCell::new_robot_with_name(labels[1]);
    start_indexes.push(index_1);
    let index_2 = Index {
        y: center_index.y - 1,
        x: center_index.x + 1,
    };
    map[index_2] = MapCell::new_robot_with_name(labels[2]);
    start_indexes.push(index_2);
    let index_3 = Index {
        y: center_index.y + 1,
        x: center_index.x + 1,
    };
    map[index_3] = MapCell::new_robot_with_name(labels[3]);
    start_indexes.push(index_3);

    //map.print();

    let graph = build_key_graph4(&map, &start_indexes);

    let min_steps = search_graph4(&graph, labels, target_key_count);
    Some(min_steps)
}

struct DijkstraState {
    steps_taken: u64,
    current_position: char,
    collected_keys: Vec<bool>,
}

impl PartialEq for DijkstraState {
    fn eq(&self, other: &Self) -> bool {
        self.steps_taken == other.steps_taken
    }
}

impl Eq for DijkstraState {}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse cmp to make the Heap min-Heap
        other.steps_taken.cmp(&self.steps_taken)
    }
}

struct DijkstraState4 {
    steps_taken: u64,
    positions: [char; 4],
    collected_keys: u32,
}

impl PartialEq for DijkstraState4 {
    fn eq(&self, other: &Self) -> bool {
        self.steps_taken == other.steps_taken
    }
}

impl Eq for DijkstraState4 {}

impl PartialOrd for DijkstraState4 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DijkstraState4 {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse cmp to make the Heap min-Heap
        other.steps_taken.cmp(&self.steps_taken)
    }
}

fn search_graph(graph: &HashMap<char, Vec<Edge>>, start: char, key_count: usize) -> u64 {
    let mut heap = BinaryHeap::new();

    // Mininal distance to the current key also having other keys.
    let mut distances: HashMap<(char, Vec<bool>), u64> = HashMap::new();

    let collected = vec![false; key_count];

    distances.insert((start, collected.clone()), 0);
    heap.push(DijkstraState {
        steps_taken: 0,
        current_position: start,
        collected_keys: collected,
    });

    while let Some(current_state) = heap.pop() {
        // State: current_key + collected_keys
        let current_key_state = (
            current_state.current_position,
            current_state.collected_keys.clone(),
        );

        // Already have better state
        if current_state.steps_taken > distances[&current_key_state] {
            continue;
        }

        // Found all keys, optimal result garanteed by poping from priority queue
        if current_state.collected_keys.iter().filter(|&&v| v).count() == key_count {
            return current_state.steps_taken;
        }

        // Getting edges for current position.
        let edges = match graph.get(&current_state.current_position) {
            Some(edges) => edges,
            None => continue,
        };

        for edge in edges {
            let key_index = (edge.to as u8 - b'a') as usize;

            // Already collected the key in the current state.
            if current_state.collected_keys[key_index] {
                continue;
            }

            // Do not traverce closed door path
            let can_open_edge_doors = edge.doors_needed.iter().all(|door| {
                let door_index = (*door as u8 - b'A') as usize;
                current_state.collected_keys[door_index]
            });

            if !can_open_edge_doors {
                continue;
            }

            // Add to path cost + collect the key
            let next_state_steps = current_state.steps_taken + edge.dist;
            let mut next_state_collected = current_state.collected_keys.clone();
            next_state_collected[key_index] = true;

            // Next key to collect using the edge
            let next_state = (edge.to, next_state_collected.clone());

            // Test if we already have a better distance for next state
            let should_visit = match distances.get(&next_state) {
                Some(&best_for_state) => next_state_steps < best_for_state,
                None => true,
            };

            if should_visit {
                distances.insert(next_state, next_state_steps);

                heap.push(DijkstraState {
                    steps_taken: next_state_steps,
                    current_position: edge.to,
                    collected_keys: next_state_collected,
                });
            }
        }
    }

    u64::MAX
}

fn search_graph4(graph: &HashMap<char, Vec<Edge4>>, start: [char; 4], key_count: usize) -> u64 {
    let mut heap = BinaryHeap::new();

    let mut distances: HashMap<([char; 4], u32), u64> = HashMap::new();

    distances.insert((start, 0), 0);

    heap.push(DijkstraState4 {
        steps_taken: 0,
        positions: start,
        collected_keys: 0,
    });

    let target_mask = (1u32 << key_count) - 1; // mask for finding all keys

    while let Some(current_state) = heap.pop() {
        // State: current_key + collected_keys
        let current_key_state = (current_state.positions, current_state.collected_keys);

        // Already have better state
        if current_state.steps_taken > distances[&current_key_state] {
            continue;
        }

        // Found all keys, optimal result garanteed by poping from priority queue
        if current_state.collected_keys == target_mask {
            return current_state.steps_taken;
        }

        // Traversing each robot
        for robot_index in 0..4 {
            let current_position = current_state.positions[robot_index];
            // Getting edges for current position.
            let edges = match graph.get(&current_position) {
                Some(edges) => edges,
                None => continue,
            };

            for edge in edges {
                let key_index = (edge.to as u8 - b'a') as usize;
                let key_bit = 1u32 << key_index;

                // Already collected the key in the current state.
                if current_state.collected_keys & key_bit != 0 {
                    continue;
                }

                // Do not traverce closed door path
                let can_open_edge_doors = edge.doors_needed & !current_state.collected_keys == 0;

                if !can_open_edge_doors {
                    continue;
                }

                // Add to path cost + collect the key
                let next_state_steps = current_state.steps_taken + edge.dist;

                // Move current robot to make next_possitions
                let mut next_positions = current_state.positions;
                next_positions[robot_index] = edge.to;

                let next_state_collected = current_state.collected_keys | key_bit;

                // Next key to collect using the edge for a robot
                let next_state = (next_positions, next_state_collected);

                // Test if we already have a better distance for next state
                let should_visit = match distances.get(&next_state) {
                    Some(&best_for_state) => next_state_steps < best_for_state,
                    None => true,
                };

                if should_visit {
                    distances.insert(next_state, next_state_steps);

                    heap.push(DijkstraState4 {
                        steps_taken: next_state_steps,
                        positions: next_positions,
                        collected_keys: next_state_collected,
                    });
                }
            }
        }
    }

    u64::MAX
}

#[derive(Debug, Clone)]
struct Edge {
    to: char,
    dist: u64,
    doors_needed: Vec<char>, // doors required to reach the key,
}

#[derive(Debug, Clone)]
struct Edge4 {
    to: char,
    dist: u64,
    doors_needed: u32, // doors required to reach the key,
}

// Graph maps a node (start `@` or a key) to all reachable keys
type KeyGraph = HashMap<char, Vec<Edge>>;

// Graph maps a node (start `@` or a key) to all reachable keys
type KeyGraph4 = HashMap<char, Vec<Edge4>>;

fn build_key_graph(matrix: &Matrix<MapCell>, start_index: Index) -> KeyGraph {
    let mut graph: KeyGraph = HashMap::new();

    // BFS from start position '@'
    let start_edges = bfs_from_index(matrix, start_index);
    graph.insert(
        '@',
        start_edges
            .into_iter()
            .map(|(_, c, d, doors)| Edge {
                to: c,
                dist: d,
                doors_needed: doors,
            })
            .collect(),
    );

    // BFS from each key
    for y in 0..matrix.size.y {
        for x in 0..matrix.size.x {
            let cell = &matrix[y][x];
            if let Some(c) = cell.door_or_key
                && c.is_ascii_lowercase()
            {
                let edges = bfs_from_index(matrix, Index { x, y });
                graph.insert(
                    c,
                    edges
                        .into_iter()
                        .map(|(_, kc, d, doors)| Edge {
                            to: kc,
                            dist: d,
                            doors_needed: doors,
                        })
                        .collect(),
                );
            }
        }
    }

    graph
}

fn build_key_graph4(matrix: &Matrix<MapCell>, start_indexes: &[Index]) -> KeyGraph4 {
    let mut graph: KeyGraph4 = HashMap::new();

    // BFS from start positions
    for start_index in start_indexes {
        let start_edges = bfs_from_index4(matrix, *start_index);
        graph.insert(
            matrix[*start_index]
                .has_robot
                .expect("Expected robot name at map possition!"),
            start_edges
                .into_iter()
                .map(|(_, c, d, doors)| Edge4 {
                    to: c,
                    dist: d,
                    doors_needed: doors,
                })
                .collect(),
        );
    }

    // BFS from each key
    for y in 0..matrix.size.y {
        for x in 0..matrix.size.x {
            let cell = &matrix[y][x];
            if let Some(c) = cell.door_or_key
                && c.is_ascii_lowercase()
            {
                let edges = bfs_from_index4(matrix, Index { x, y });
                graph.insert(
                    c,
                    edges
                        .into_iter()
                        .map(|(_, kc, d, doors)| Edge4 {
                            to: kc,
                            dist: d,
                            doors_needed: doors,
                        })
                        .collect(),
                );
            }
        }
    }

    graph
}

// BFS from a start_index, returns a vector of reachable keys with distances and doors needed
fn bfs_from_index(
    matrix: &Matrix<MapCell>,
    start_index: Index,
) -> Vec<(Index, char, u64, Vec<char>)> {
    let mut queue: VecDeque<(Index, u64, Vec<char>)> = VecDeque::new();
    let mut visited: HashSet<Index> = HashSet::new();

    queue.push_back((start_index, 0, Vec::new()));
    visited.insert(start_index);

    let mut result = Vec::new();

    while let Some((current_index, dist, mut doors)) = queue.pop_front() {
        let cell = &matrix[current_index.y][current_index.x];

        // If it's a door, add it to the doors vector
        if let Some(c) = cell.door_or_key
            && c.is_ascii_uppercase()
        {
            doors.push(c);
        }

        // If it's a key (but not the start), add to the result
        if let Some(c) = cell.door_or_key
            && c.is_ascii_lowercase()
            && current_index != start_index
        {
            result.push((current_index, c, dist, doors.clone()));
        }

        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(next_index) = current_index.navigate_to(matrix, &dir) {
                let next_cell = &matrix[next_index.y][next_index.x];
                if !next_cell.has_wall && !visited.contains(&next_index) {
                    visited.insert(next_index);
                    queue.push_back((next_index, dist + 1, doors.clone()));
                }
            }
        }
    }

    result
}

// BFS from a start_index, returns a vector of reachable keys with distances and doors needed
fn bfs_from_index4(matrix: &Matrix<MapCell>, start_index: Index) -> Vec<(Index, char, u64, u32)> {
    let mut queue: VecDeque<(Index, u64, u32)> = VecDeque::new();
    let mut visited: HashSet<Index> = HashSet::new();

    queue.push_back((start_index, 0, 0));
    visited.insert(start_index);

    let mut result = Vec::new();

    while let Some((current_index, dist, mut doors)) = queue.pop_front() {
        let cell = &matrix[current_index.y][current_index.x];

        // If it's a door, add it to the doors vector
        if let Some(c) = cell.door_or_key
            && c.is_ascii_uppercase()
        {
            doors |= 1u32 << (c as u8 - b'A');
        }

        // If it's a key (but not the start), add to the result
        if let Some(c) = cell.door_or_key
            && c.is_ascii_lowercase()
            && current_index != start_index
        {
            result.push((current_index, c, dist, doors));
        }

        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(next_index) = current_index.navigate_to(matrix, &dir) {
                let next_cell = &matrix[next_index.y][next_index.x];
                if !next_cell.has_wall && !visited.contains(&next_index) {
                    visited.insert(next_index);
                    queue.push_back((next_index, dist + 1, doors));
                }
            }
        }
    }

    result
}

#[derive(Debug, Clone)]
pub struct MapCell {
    has_wall: bool,
    has_robot: Option<char>,
    door_or_key: Option<char>,
}

impl MapCell {
    pub fn new(has_wall: bool) -> Self {
        MapCell {
            has_wall,
            has_robot: None,
            door_or_key: None,
        }
    }

    pub fn new_robot() -> Self {
        MapCell {
            has_wall: false,
            has_robot: Some('@'),
            door_or_key: None,
        }
    }

    pub fn new_robot_with_name(name: char) -> Self {
        MapCell {
            has_wall: false,
            has_robot: Some(name),
            door_or_key: None,
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
                has_robot: None,
                door_or_key: Some(value),
            },
        }
    }
}

impl Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = if let Some(robot_name) = self.has_robot {
            robot_name
        } else if !self.has_wall {
            self.door_or_key.unwrap_or('.')
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
    fn test_part_one_6() {
        let result = solve_part_1(
            &advent_of_code::template::read_file_part("examples", DAY, 6),
            2,
        );
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_one_7() {
        let result = solve_part_1(
            &advent_of_code::template::read_file_part("examples", DAY, 7),
            4,
        );
        assert_eq!(result, Some(26));
    }

    #[test]
    fn test_part_two_7() {
        let result = solve_part_2(
            &advent_of_code::template::read_file_part("examples", DAY, 7),
            4,
        );
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_8() {
        let result = solve_part_2(
            &advent_of_code::template::read_file_part("examples", DAY, 8),
            4,
        );
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two_9() {
        let result = solve_part_2(
            &advent_of_code::template::read_file_part("examples", DAY, 9),
            12,
        );
        assert_eq!(result, Some(32));
    }

    #[test]
    fn test_part_two_10() {
        let result = solve_part_2(
            &advent_of_code::template::read_file_part("examples", DAY, 10),
            15,
        );
        assert_eq!(result, Some(72));
    }
}
