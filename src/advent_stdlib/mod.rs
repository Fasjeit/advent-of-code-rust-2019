use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::str::FromStr;

#[allow(dead_code)]
pub fn pseudo_dijkstra(
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
            if !matrix[next_index.y][next_index.x].has_wall()
                && matrix[next_index.y][next_index.x].cost >= (cost + 1)
            {
                to_visit_set.push(Reverse((cost + 1, next_index)));
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Right) {
            if !matrix[next_index.y][next_index.x].has_wall()
                && matrix[next_index.y][next_index.x].cost >= (cost + 1)
            {
                to_visit_set.push(Reverse((cost + 1, next_index)));
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Left) {
            if !matrix[next_index.y][next_index.x].has_wall()
                && matrix[next_index.y][next_index.x].cost >= (cost + 1)
            {
                to_visit_set.push(Reverse((cost + 1, next_index)));
            }
        }

        if let Some(next_index) = index.navigate_to(matrix, &Direction::Up) {
            if !matrix[next_index.y][next_index.x].has_wall()
                && matrix[next_index.y][next_index.x].cost >= (cost + 1)
            {
                to_visit_set.push(Reverse((cost + 1, next_index)));
            }
        }
    }

    None
}

#[derive(Debug, Clone)]
pub struct MapCell {
    has_wall: bool,
    source: bool,
    target: bool,
    cost: u64,
}

impl MapCell {
    pub fn new(has_wall: bool) -> Self {
        MapCell {
            has_wall,
            source: false,
            target: false,
            cost: { u64::MAX },
        }
    }

    pub fn new_source() -> Self {
        MapCell {
            has_wall: false,
            source: true,
            target: false,
            cost: { u64::MAX },
        }
    }

    pub fn new_target() -> Self {
        MapCell {
            has_wall: false,
            source: false,
            target: true,
            cost: { u64::MAX },
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
        if let Some(matrix) = matrix {
            if self.y == matrix.size.y - 1 {
                return None;
            }
        }
        Some(Index {
            x: self.x,
            y: self.y + 1,
        })
    }

    #[allow(dead_code)]
    pub fn right<T>(&self, matrix: Option<&Matrix<T>>) -> Option<Index> {
        if let Some(matrix) = matrix {
            if self.x == matrix.size.x - 1 {
                return None;
            }
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
    pub fn get_index_from_position(&self, indx: usize) -> Index {
        let y = indx / self.size.x;
        let x = indx - y * self.size.x;
        Index { x, y }
    }
}

impl Matrix<MapCell> {
    #[allow(dead_code)]
    pub fn has_index(&self, index: &Index) -> bool {
        self.size.x > index.x && self.size.y > index.y
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let mut ch = '.';
                if self[y][x].has_wall() {
                    ch = '#'
                } else if self[y][x].source {
                    ch = 'S'
                } else if self[y][x].target {
                    ch = 'E'
                } else if self[y][x].cost < 10 {
                    ch = self[y][x].cost.to_string().chars().collect::<Vec<char>>()[0]
                }
                print!("{}", ch);
            }
            println!();
        }
    }
}

impl Matrix<bool> {
    #[allow(dead_code)]
    pub fn print(&self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let mut ch = '.';
                if self[y][x] {
                    ch = '0'
                }
                print!("{}", ch);
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

#[allow(dead_code)]
pub fn parse_row_input_as_data_array<T>(input: &str) -> (Vec<T>, Size)
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
