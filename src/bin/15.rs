use advent_of_code::intcode::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::str::FromStr;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, true)
}

fn solve(input: &str, part_two: bool) -> Option<u64> {
    // run DFS using stack of directions
    // then run bfs of Dijkstra for shortest path

    // DFS
    // forward: find unchecked direction; go
    // back: go back in stack until find unchecked direction; goto forward

    // If need turn (last direction is otr to current one)
    // mark prev and to-turn as 1

    // for part 2 - run Dijkstra from target to all cells to
    // find the cell with the biggest cost

    //println!("===========");

    let mut robot = Robot::new(input, 64, 64);
    //robot.map.print();

    // DFS

    // stack for backtracking
    let mut path_stack = Vec::<Direction>::new();

    // visited direction memory
    let mut checked_map_memory = HashMap::<(Index, Direction), bool>::new();

    let mut max_loop = 1_000_000_000;

    let mut target_pos: Index = Index { x: 0, y: 0 };

    // main loop
    'dfs: loop {
        max_loop -= 1;
        if max_loop == 0 {
            panic!("inf loop");
        }
        //println!("===========");
        //robot.map.print();
        //dbg!(&checked_map_memory);

        for command in [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ] {
            // check if direction is not visited
            let already_visited_direction =
                if let Some(e) = checked_map_memory.get(&(robot.pos, command)) {
                    *e
                } else {
                    false
                };

            // if not - mark as visited and add to stack
            if !already_visited_direction {
                // mark cell direction as checked
                checked_map_memory
                    .entry((robot.pos, command))
                    .and_modify(|e| *e = true)
                    .or_insert(true);

                // go to direction
                let robot_result = robot.step(command);
                if robot_result == 2 {
                    target_pos = robot.pos;
                }
                if robot_result != 0 {
                    // robot moved!
                    // push direction to the stack
                    path_stack.push(command);
                    continue 'dfs;
                }
            }
        }

        // all directions visited
        // pop prev direction from the stack and do back
        if let Some(prev_direction) = path_stack.pop() {
            robot.step(prev_direction.reverse());
        } else {
            // no new directions, terminate
            break 'dfs;
        }
    }

    // run Dijkstra from start to finish on robot inner map
    let start_index = Index {
        x: robot.map.size.x / 2,
        y: robot.map.size.y / 2,
    };
    let end_index = target_pos;
    let mut to_visit_set = BinaryHeap::new();
    to_visit_set.push(Reverse((0_u64, start_index)));

    let result = pseudo_dijkstra(&mut robot.map, Some(&end_index), &mut to_visit_set);

    //robot.map.print();

    if !part_two {
        return result;
    }

    // run dijkstra for all cells from O cell, to find the longest path
    // clean the costs
    for cell in &mut robot.map.data {
        cell.cost = u64::MAX
    }

    let mut to_visit_set = BinaryHeap::new();
    to_visit_set.push(Reverse((0_u64, target_pos)));
    pseudo_dijkstra(&mut robot.map, None, &mut to_visit_set);

    // just find the biggest cost
    let mut biggest_cost = 0;
    for cell in &mut robot.map.data {
        if cell.cost != u64::MAX && cell.cost > biggest_cost {
            biggest_cost = cell.cost
        }
    }
    Some(biggest_cost)
}

#[allow(dead_code)]
fn test_fake_map(input: &str, part_two: bool) -> Option<u64> {
    let fake_map = Matrix::<MapCell>::from_string(input);
    //fake_map.print();

    //println!("===========");

    let mut robot = Robot::new("99", 9, 5);
    //robot.map.print();

    // DFS

    // stack for backtracking
    let mut path_stack = Vec::<Direction>::new();

    // visited direction memory
    let mut checked_map_memory = HashMap::<(Index, Direction), bool>::new();

    let mut max_loop = 450;

    let mut target_pos: Index = Index { x: 0, y: 0 };

    // main loop
    'dfs: loop {
        max_loop -= 1;
        if max_loop == 0 {
            panic!("inf loop");
        }
        //println!("===========");
        //robot.map.print();
        //dbg!(&checked_map_memory);

        for command in [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ] {
            // check if direction is not visited
            let already_visited_direction =
                if let Some(e) = checked_map_memory.get(&(robot.pos, command)) {
                    *e
                } else {
                    false
                };

            // if not - mark as visited and add to stack
            if !already_visited_direction {
                // mark cell direction as checked
                checked_map_memory
                    .entry((robot.pos, command))
                    .and_modify(|e| *e = true)
                    .or_insert(true);

                // go to direction
                let robot_result = robot.step_fake_map(command, &fake_map);
                if robot_result == 2 {
                    target_pos = robot.pos;
                }
                if robot_result != 0 {
                    // robot moved!
                    // push direction to the stack
                    path_stack.push(command);
                    continue 'dfs;
                }
            }
        }

        // all directions visited
        // pop prev direction from the stack and do back
        if let Some(prev_direction) = path_stack.pop() {
            robot.step_fake_map(prev_direction.reverse(), &fake_map);
        } else {
            // no new directions, terminate
            break 'dfs;
        }
    }

    // run Dijkstra from start to finish on robot inner map
    let start_index = Index {
        x: robot.map.size.x / 2,
        y: robot.map.size.y / 2,
    };
    let end_index = target_pos;
    let mut to_visit_set = BinaryHeap::new();
    to_visit_set.push(Reverse((0_u64, start_index)));

    let result = pseudo_dijkstra(&mut robot.map, Some(&end_index), &mut to_visit_set);

    //robot.map.print();

    if !part_two {
        return result;
    }

    // run dijkstra for all cells from O cell, to find the longest path
    // clean the costs
    for cell in &mut robot.map.data {
        cell.cost = u64::MAX
    }

    let mut to_visit_set = BinaryHeap::new();
    to_visit_set.push(Reverse((0_u64, target_pos)));
    pseudo_dijkstra(&mut robot.map, None, &mut to_visit_set);

    // just find the biggest cost
    let mut biggest_cost = 0;
    for cell in &mut robot.map.data {
        if cell.cost != u64::MAX && cell.cost > biggest_cost {
            biggest_cost = cell.cost
        }
    }

    //robot.map.print();

    Some(biggest_cost)
    //
}

fn pseudo_dijkstra(
    matrix: &mut Matrix<MapCell>,
    ending_position: Option<&Index>,
    to_visit_set: &mut BinaryHeap<Reverse<(u64, Index)>>,
) -> Option<u64> {
    let mut safe_counter = 100000;

    while let Some(Reverse((cost, index))) = to_visit_set.pop() {
        if safe_counter <= 0 {
            panic!("Safe counter stop.");
        }
        safe_counter -= 1;

        if matrix[index.y][index.x].cost != u64::MAX {
            assert!(matrix[index.y][index.x].cost <= cost);
            continue;
        }

        matrix[index.y][index.x].cost = cost;

        //dbg!(&index);

        if let Some(ending_position) = ending_position {
            if index == *ending_position {
                return Some(cost);
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Down) {
            if !matrix[next_index.y][next_index.x].has_wall
                && matrix[next_index.y][next_index.x].cost >= (cost + 1)
            {
                to_visit_set.push(Reverse((cost + 1, next_index)));
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Right) {
            if !matrix[next_index.y][next_index.x].has_wall
                && matrix[next_index.y][next_index.x].cost >= (cost + 1)
            {
                to_visit_set.push(Reverse((cost + 1, next_index)));
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Left) {
            if !matrix[next_index.y][next_index.x].has_wall
                && matrix[next_index.y][next_index.x].cost >= (cost + 1)
            {
                to_visit_set.push(Reverse((cost + 1, next_index)));
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Up) {
            if !matrix[next_index.y][next_index.x].has_wall
                && matrix[next_index.y][next_index.x].cost >= (cost + 1)
            {
                to_visit_set.push(Reverse((cost + 1, next_index)));
            }
        }
    }

    None
}

#[derive(Debug, Clone)]
struct MapCell {
    has_wall: bool,
    has_robot: bool,
    has_oxygen_tank: bool,
    visited: bool,
    cost: u64,
}

impl MapCell {
    fn new_robot() -> Self {
        MapCell {
            has_wall: false,
            has_robot: true,
            has_oxygen_tank: false,
            visited: true,
            cost: u64::MAX,
        }
    }
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            ' ' => MapCell {
                has_wall: false,
                has_robot: false,
                has_oxygen_tank: false,
                visited: true,
                cost: u64::MAX,
            },
            '#' => MapCell {
                has_wall: true,
                has_robot: false,
                has_oxygen_tank: false,
                visited: false,
                cost: u64::MAX,
            },
            'O' => MapCell {
                has_wall: false,
                has_robot: false,
                has_oxygen_tank: true,
                visited: false,
                cost: u64::MAX,
            },
            'D' => MapCell::new_robot(),
            _ => panic!("Unknown char in map data!"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Size {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Index {
    x: usize,
    y: usize,
}

impl Index {
    fn up<T>(&self, _matrix: &Matrix<T>) -> Option<Index> {
        if self.y == 0 {
            return None;
        }
        Some(Index {
            x: self.x,
            y: self.y - 1,
        })
    }

    fn left<T>(&self, _matrix: &Matrix<T>) -> Option<Index> {
        if self.x == 0 {
            return None;
        }
        Some(Index {
            x: self.x - 1,
            y: self.y,
        })
    }

    fn down<T>(&self, matrix: &Matrix<T>) -> Option<Index> {
        if self.y == matrix.size.y - 1 {
            return None;
        }
        Some(Index {
            x: self.x,
            y: self.y + 1,
        })
    }

    fn right<T>(&self, matrix: &Matrix<T>) -> Option<Index> {
        if self.x == matrix.size.x - 1 {
            return None;
        }
        Some(Index {
            x: self.x + 1,
            y: self.y,
        })
    }

    fn navigate_to<T>(&self, matrix: &Matrix<T>, direction: &Direction) -> Option<Index> {
        match direction {
            Direction::Up => self.up(matrix),
            Direction::Down => self.down(matrix),
            Direction::Left => self.left(matrix),
            Direction::Right => self.right(matrix),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn to_int(self) -> i64 {
        match self {
            Direction::Up => 1,
            Direction::Left => 3,
            Direction::Right => 4,
            Direction::Down => 2,
        }
    }

    #[allow(dead_code)]
    fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }

    #[allow(dead_code)]
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    #[allow(dead_code)]
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Debug, Clone)]
struct Matrix<T> {
    size: Size,
    data: Vec<T>,
}

#[allow(dead_code)]
impl<T> Matrix<T> {
    fn get_index_from_position(&self, indx: usize) -> Index {
        let y = indx / self.size.x;
        let x = indx - y * self.size.x;
        Index { x, y }
    }
}

impl Matrix<MapCell> {
    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let ch;
                if self[y][x].has_oxygen_tank && self[y][x].has_robot {
                    ch = 'd'
                } else if self[y][x].has_oxygen_tank {
                    ch = 'O'
                } else if self[y][x].has_robot {
                    ch = 'D'
                } else if self[y][x].has_wall {
                    ch = '#'
                } else if self[y][x].visited {
                    if self[y][x].cost != u64::MAX {
                        ch = char::from_digit((self[y][x].cost % 10) as u32, 10).unwrap();
                    } else {
                        ch = '.'
                    }
                } else {
                    ch = ' '
                }
                print!("{ch}");
            }
            println!();
        }
    }

    pub fn from_string(input: &str) -> Self {
        let (data, size) = parse_row_input_as_data_array::<char>(input);
        let data_cells: Vec<MapCell> = data.into_iter().map(MapCell::from).collect();
        Matrix {
            size,
            data: data_cells.clone(),
        }
    }
}

impl<T> std::ops::Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &[T] {
        let start = row * self.size.x;
        &self.data[start..start + self.size.x]
    }
}

impl<T> std::ops::IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut [T] {
        let start = row * self.size.x;
        &mut self.data[start..start + self.size.x]
    }
}

#[allow(dead_code)]
fn parse_row_input_as_data_array<T>(input: &str) -> (Vec<T>, Size)
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: Debug,
{
    let splitted_lines: Vec<&str> = input.lines().collect();
    let size_y = splitted_lines.len();
    let size_x = splitted_lines[0].len();

    let result: Vec<T> = input
        .chars()
        .filter(|c| *c != '\n' && *c != '\r')
        .map(|c| c.to_string().parse().expect("T values expected"))
        .collect();

    (
        result,
        Size {
            x: size_x,
            y: size_y,
        },
    )
}

struct Robot {
    pos: Index,
    map: Matrix<MapCell>,
    cpu: Machine,
}

impl Robot {
    pub fn new(firmware: &str, map_size_x: usize, map_size_y: usize) -> Self {
        let firmware: Vec<i64> = firmware
            .split(",")
            .map(|i| {
                i.parse()
                    .unwrap_or_else(|_| panic!("Expected i64 list [{i}]"))
            })
            .collect();

        // create square map.
        let mut map_data = Vec::<MapCell>::new();
        for _ in 0..map_size_x * map_size_y {
            map_data.push(MapCell {
                has_wall: false,
                has_robot: false,
                has_oxygen_tank: false,
                visited: false,
                cost: u64::MAX,
            });
        }

        let mut map: Matrix<MapCell> = Matrix {
            size: Size {
                x: map_size_x,
                y: map_size_y,
            },
            data: map_data,
        };

        // place robot at the center
        map[map_size_y / 2][map_size_x / 2].has_robot = true;
        map[map_size_y / 2][map_size_x / 2].visited = true;

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

        let machine = Machine::new_with_input(ext_memory, input);

        Robot {
            pos: Index {
                x: map_size_x / 2,
                y: map_size_y / 2,
            },
            map,
            cpu: machine,
        }
    }

    pub fn step_fake_map(&mut self, command: Direction, fake_map: &Matrix<MapCell>) -> i64 {
        // just for debug
        // emulate the real map with provided data
        // fake map size should match robot one
        // or need to create separate index for fake map here
        let target_pos = self
            .pos
            .navigate_to(&self.map, &command)
            .expect("Out of map bounds");
        let target_cell = &fake_map[target_pos.y][target_pos.x];
        let result = if target_cell.has_wall {
            0
        } else if target_cell.has_oxygen_tank {
            2
        } else {
            1
        };

        // the rest as normal step
        // matching exe result
        match result {
            0 => {
                // Wall
                // update map
                self.map[target_pos.y][target_pos.x].has_wall = true;
            }
            1 => {
                // Empty
                // move the robot, update the map
                self.map[target_pos.y][target_pos.x].visited = true;

                self.map[self.pos.y][self.pos.x].has_robot = false;
                self.map[target_pos.y][target_pos.x].has_robot = true;

                self.pos = target_pos;
            }
            2 => {
                // found oxygen!
                // move the robot, update the map
                self.pos = target_pos;
                self.map[target_pos.y][target_pos.x].visited = true;
                self.map[target_pos.y][target_pos.x].has_oxygen_tank = true;
            }
            _ => panic!("Unknown result!"),
        }
        result
    }

    pub fn step(&mut self, command: Direction) -> i64 {
        // set input
        self.cpu.input.push(command.to_int());

        // run cpu
        let result = match self.cpu.execute() {
            ExecuteResult::WaitingInput => self.cpu.output[self.cpu.output_pointer - 1],
            _ => panic!(),
        };

        let target_pos = self
            .pos
            .navigate_to(&self.map, &command)
            .expect("Out of map bounds");

        // matching exe result
        match result {
            0 => {
                // Wall
                // update map
                self.map[target_pos.y][target_pos.x].has_wall = true;
            }
            1 => {
                // Empty
                // move the robot, update the map
                self.map[target_pos.y][target_pos.x].visited = true;

                self.map[self.pos.y][self.pos.x].has_robot = false;
                self.map[target_pos.y][target_pos.x].has_robot = true;

                self.pos = target_pos;
            }
            2 => {
                // found oxygen!
                // move the robot, update the map
                self.pos = target_pos;
                self.map[target_pos.y][target_pos.x].visited = true;
                self.map[target_pos.y][target_pos.x].has_oxygen_tank = true;
            }
            _ => panic!("Unknown result!"),
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = test_fake_map(
            &advent_of_code::template::read_file_part("examples", DAY, 1),
            false,
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_2() {
        let result = test_fake_map(
            &advent_of_code::template::read_file_part("examples", DAY, 2),
            false,
        );
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_3() {
        let result = test_fake_map(
            &advent_of_code::template::read_file_part("examples", DAY, 3),
            false,
        );
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two_1() {
        let result = test_fake_map(
            &advent_of_code::template::read_file_part("examples", DAY, 1),
            true,
        );
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two_2() {
        let result = test_fake_map(
            &advent_of_code::template::read_file_part("examples", DAY, 2),
            true,
        );
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two_3() {
        let result = test_fake_map(
            &advent_of_code::template::read_file_part("examples", DAY, 3),
            true,
        );
        assert_eq!(result, Some(8));
    }
}
