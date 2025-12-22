use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;

/// Dijkstra path cost computation.
///
/// # Examples
/// Find costs from start to end.
/// ```
/// use advent_of_code::advent_stdlib::*;
/// use std::cmp::Reverse;
/// use std::collections::BinaryHeap;
/// use std::fmt::Debug;
/// use std::str::FromStr;
///
/// let input = ".#.\n.#.\n...";
/// let mut matrix = Matrix::<MapCell>::from_string(input);
///
/// let mut start_index = Index {x: 0, y: 0};
/// let mut end_index = Index {x: 2, y: 0};
///
/// let mut to_visit_set = BinaryHeap::new();
/// to_visit_set.push(Reverse((0_u64, start_index)));
///
/// let result = pseudo_dijkstra(&mut matrix, Some(&end_index), &mut to_visit_set);
/// assert_eq!(result, Some(6));
/// ```
/// Find all costs.
/// ```
/// use advent_of_code::advent_stdlib::*;
/// use std::cmp::Reverse;
/// use std::collections::BinaryHeap;
/// use std::fmt::Debug;
/// use std::str::FromStr;
///
/// let input = ".#.\n.#.\n...";
/// let mut matrix = Matrix::<MapCell>::from_string(input);
///
/// let mut start_index = Index {x: 0, y: 0};
///
/// let mut to_visit_set = BinaryHeap::new();
/// to_visit_set.push(Reverse((0_u64, start_index)));
///
/// let result = pseudo_dijkstra(&mut matrix, None, &mut to_visit_set);
/// assert_eq!(result, None);
/// assert_eq!(matrix[0][0].cost, 0);
/// assert_eq!(matrix[2][1].cost, 3);
/// assert_eq!(matrix[0][2].cost, 6);
/// ```
#[allow(dead_code)]
pub fn pseudo_dijkstra(
    matrix: &mut Matrix<MapCell>,
    ending_position: Option<&Index>,
    to_visit_set: &mut BinaryHeap<Reverse<(u64, Index)>>,
) -> Option<u64> {
    // Usage:
    // let mut to_visit_set = BinaryHeap::new();
    // to_visit_set.push(Reverse((0_u64, start_index)));
    // let result = pseudo_dijkstra(&mut robot.map, Some(&end_index), &mut to_visit_set);
    // or without end_index, if need to visit all cells.
    //
    // Also can be rewritten to use with custom cost object with custom comparer.
    // Example with Cost state containing a HashSet:
    //
    // #[derive(PartialEq, Eq)]
    // struct CostState {
    //     cost: u64,
    //     state: OtherType
    // }
    //
    // impl Ord for CostState {
    //     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    //         self.cost.cmp(&other.cost)
    //     }
    // }
    //
    // impl CostState {
    //     fn new(data: &(u64, Index, HashSet<char>)) -> Self {
    //         CostState {
    //             cost: data.0,
    //             index: data.1,
    //             state: data.2.clone(),
    //         }
    //     }
    // }
    //
    // impl PartialOrd for CostState {
    //     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    //         Some(self.cmp(other))
    //     }
    // }
    //
    // and then
    // to_visit_set: &mut BinaryHeap<Reverse<CostState>>

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

        if let Some(ending_position) = ending_position
            && index == *ending_position
        {
            return Some(cost);
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Down)
            && !matrix[next_index.y][next_index.x].has_wall()
            && matrix[next_index.y][next_index.x].cost >= (cost + 1)
        {
            to_visit_set.push(Reverse((cost + 1, next_index)));
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Right)
            && !matrix[next_index.y][next_index.x].has_wall()
            && matrix[next_index.y][next_index.x].cost >= (cost + 1)
        {
            to_visit_set.push(Reverse((cost + 1, next_index)));
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Left)
            && !matrix[next_index.y][next_index.x].has_wall()
            && matrix[next_index.y][next_index.x].cost >= (cost + 1)
        {
            to_visit_set.push(Reverse((cost + 1, next_index)));
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Up)
            && !matrix[next_index.y][next_index.x].has_wall()
            && matrix[next_index.y][next_index.x].cost >= (cost + 1)
        {
            to_visit_set.push(Reverse((cost + 1, next_index)));
        }
    }

    None
}

#[derive(Debug, Clone)]
pub struct MapCell {
    pub has_wall: bool,
    pub source: bool,
    pub target: bool,
    pub cost: u64,
}

impl MapCell {
    pub fn new(has_wall: bool) -> Self {
        MapCell {
            has_wall,
            source: false,
            target: false,
            cost: u64::MAX,
        }
    }

    pub fn new_source() -> Self {
        MapCell {
            has_wall: false,
            source: true,
            target: false,
            cost: u64::MAX,
        }
    }

    pub fn new_target() -> Self {
        MapCell {
            has_wall: false,
            source: false,
            target: true,
            cost: u64::MAX,
        }
    }

    pub fn has_wall(&self) -> bool {
        self.has_wall
    }
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            '.' => MapCell::new(false),
            '#' => MapCell::new(true),
            'S' => MapCell::new_source(),
            'E' => MapCell::new_target(),
            _ => panic!("Unknown char in map data!"),
        }
    }
}

impl Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ch = '.';
        if self.has_wall() {
            ch = '#'
        } else if self.source {
            ch = 'S'
        } else if self.target {
            ch = 'E'
        } else if self.cost < 10 {
            ch = self.cost.to_string().chars().collect::<Vec<char>>()[0]
        }
        write!(f, "{}", ch)
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Size {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Index {
    pub x: usize,
    pub y: usize,
}

impl Index {
    #[allow(dead_code)]
    pub fn up<T>(&self, _matrix: Option<&Matrix<T>>) -> Option<Index> {
        if self.y == 0 {
            return None;
        }
        Some(Index {
            x: self.x,
            y: self.y - 1,
        })
    }

    #[allow(dead_code)]
    pub fn left<T>(&self, _matrix: Option<&Matrix<T>>) -> Option<Index> {
        if self.x == 0 {
            return None;
        }
        Some(Index {
            x: self.x - 1,
            y: self.y,
        })
    }

    #[allow(dead_code)]
    pub fn down<T>(&self, matrix: Option<&Matrix<T>>) -> Option<Index> {
        if let Some(matrix) = matrix
            && self.y == matrix.size.y - 1
        {
            return None;
        }
        Some(Index {
            x: self.x,
            y: self.y + 1,
        })
    }

    #[allow(dead_code)]
    pub fn right<T>(&self, matrix: Option<&Matrix<T>>) -> Option<Index> {
        if let Some(matrix) = matrix
            && self.x == matrix.size.x - 1
        {
            return None;
        }
        Some(Index {
            x: self.x + 1,
            y: self.y,
        })
    }

    #[allow(dead_code)]
    pub fn navigate_to<T>(&self, matrix: &Matrix<T>, direction: &Direction) -> Option<Index> {
        match direction {
            Direction::Up => self.up(Some(matrix)),
            Direction::Down => self.down(Some(matrix)),
            Direction::Left => self.left(Some(matrix)),
            Direction::Right => self.right(Some(matrix)),
        }
    }

    #[allow(dead_code)]
    pub fn navigate_to_no_matrix(&self, direction: &Direction) -> Index {
        match direction {
            Direction::Up => self.up::<()>(None).unwrap(),
            Direction::Down => self.down::<()>(None).unwrap(),
            Direction::Left => self.left::<()>(None).unwrap(),
            Direction::Right => self.right::<()>(None).unwrap(),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    #[allow(dead_code)]
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }

    #[allow(dead_code)]
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    #[allow(dead_code)]
    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub size: Size,
    pub data: Vec<T>,
}

impl<T> Matrix<T> {
    #[allow(dead_code)]
    pub fn get_index_from_position(&self, index: usize) -> Index {
        let y = index / self.size.x;
        let x = index - y * self.size.x;
        Index { x, y }
    }

    #[allow(dead_code)]
    pub fn has_index(&self, index: &Index) -> bool {
        self.size.x > index.x && self.size.y > index.y
    }
}

impl<T: From<char>> Matrix<T> {
    #[allow(dead_code)]
    pub fn from_char_input(input: &str) -> Self {
        let (data, size) = parse_row_input_as_data_array_matrix::<char>(input);
        let data_cells: Vec<T> = data.into_iter().map(T::from).collect();

        Matrix {
            size,
            data: data_cells,
        }
    }
}

impl<T: Display> Matrix<T> {
    #[allow(dead_code)]
    pub fn print(&self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                print!("{}", self[y][x]);
            }
            println!();
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

impl<T> std::ops::Index<Index> for Matrix<T> {
    type Output = T;

    fn index(&self, index: Index) -> &T {
        &self[index.y][index.x]
    }
}

impl<T> std::ops::IndexMut<Index> for Matrix<T> {
    fn index_mut(&mut self, index: Index) -> &mut T {
        &mut self[index.y][index.x]
    }
}

#[allow(dead_code)]
pub fn parse_row_input_as_data_array_matrix<T>(input: &str) -> (Vec<T>, Size)
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: Debug,
{
    let splitted_lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    let size_y = splitted_lines.len();
    let size_x = splitted_lines[0].len();

    let result: Vec<T> = splitted_lines
        .iter()
        .flat_map(|line| line.chars())
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

#[allow(dead_code)]
pub fn parse_row_input_as_data_array<T>(input: &str) -> Vec<T>
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: Debug,
{
    let splitted_lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();

    let result: Vec<T> = splitted_lines
        .iter()
        .map(|c| c.to_string().parse().expect("T values expected"))
        .collect();

    result
}
