use advent_of_code::advent_stdlib::Direction;
use advent_of_code::advent_stdlib::Index;
use advent_of_code::advent_stdlib::Matrix;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Display;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u64> {
    // The main idea is pre compute shortest paths from key to key
    // using BFS, keeping track of required keys.
    // Then create a new graph and traverse with Dijkstera.
    // Trick not working for all data (see test 7 as example)
    // as we are storing only one state for a path.
    solve_part_1(input, 26)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

pub fn solve_part_1(input: &str, target_key_count: usize) -> Option<u64> {
    let map = Matrix::<MapCell>::from_char_input(input);
    //map.print();

    let robot_pos = map
        .data
        .iter()
        .enumerate()
        .find(|e| e.1.has_robot)
        .unwrap()
        .0;

    let start_index = map.get_index_from_position(robot_pos);

    let graph = build_key_graph(&map, start_index);

    let mut collected = vec![false; target_key_count];
    let mut memo = Memo::new();

    let min_steps = search_graph(&graph, '@', &mut collected, &mut memo);
    Some(min_steps)
}

// Memoization key: (current node, collected keys vector)
type Memo = HashMap<(char, Vec<bool>), u64>;

fn search_graph(
    graph: &KeyGraph,
    current: char,
    collected: &mut Vec<bool>,
    memo: &mut Memo,
) -> u64 {
    if collected.iter().all(|&k| k) {
        return 0; // all keys collected
    }

    if let Some(&cached) = memo.get(&(current, collected.clone())) {
        return cached;
    }

    let mut min_cost = u64::MAX;

    if let Some(edges) = graph.get(&current) {
        'edge_loop: for edge in edges {
            let key_idx = (edge.to as u8 - b'a') as usize;
            if collected[key_idx] {
                continue; // already collected
            }

            // Check if all doors on this edge are unlocked
            for &door in &edge.doors_needed {
                let door_idx = (door.to_ascii_lowercase() as u8 - b'a') as usize;
                if !collected[door_idx] {
                    continue 'edge_loop; // can't go yet
                }
            }

            // Collect the key
            collected[key_idx] = true;
            //dbg!(edge.dist);
            //dbg!(search_graph(graph, edge.to, collected, total_keys, memo));
            let search_cost = search_graph(graph, edge.to, collected, memo);
            let cost = if search_cost == u64::MAX {
                u64::MAX
            } else {
                edge.dist + search_cost
            };

            min_cost = min_cost.min(cost);
            collected[key_idx] = false; // backtrack
        }
    }

    memo.insert((current, collected.clone()), min_cost);
    min_cost
}

#[derive(Debug, Clone)]
struct Edge {
    to: char,
    dist: u64,
    doors_needed: Vec<char>, // doors required to reach the key
}

// Graph maps a node (start `@` or a key) to all reachable keys
type KeyGraph = HashMap<char, Vec<Edge>>;

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

#[derive(Debug, Clone)]
pub struct MapCell {
    has_wall: bool,
    has_robot: bool,
    door_or_key: Option<char>,
}

impl MapCell {
    pub fn new(has_wall: bool) -> Self {
        MapCell {
            has_wall,
            has_robot: false,
            door_or_key: None,
        }
    }

    pub fn new_robot() -> Self {
        MapCell {
            has_wall: false,
            has_robot: true,
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
                has_robot: false,
                door_or_key: Some(value),
            },
        }
    }
}

impl Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = if self.has_robot {
            '@'
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

    // #[test]
    // fn test_part_one_6() {
    //     let result = solve_part_1(
    //         &advent_of_code::template::read_file_part("examples", DAY, 6),
    //         9,
    //     );
    //     assert_eq!(result, Some(9));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 7,
        ));
        assert_eq!(result, None);
    }
}
