use advent_of_code::intcode::*;
use std::fmt::Debug;
use std::str::FromStr;
use strum_macros::EnumIter;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut robot = Robot::new(input, false);
    loop {
        // let mut input = String::new();
        // stdin()
        //     .read_line(&mut input)
        //     .expect("Did not enter a correct string");
        let result = robot.step();
        if let Ok(1) = result {
            break;
        }
    }

    // count painted.
    let mut res = 0;
    for cell in robot.map.data {
        if cell.has_been_colored {
            res += 1;
        }
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut robot = Robot::new(input, true);
    loop {
        // let mut input = String::new();
        // stdin()
        //     .read_line(&mut input)
        //     .expect("Did not enter a correct string");
        let result = robot.step();
        if let Ok(1) = result {
            break;
        }
    }

    // uncomment to see result.
    //robot.map.print();

    // stub value
    Some(1)
}

#[derive(Debug, Clone)]
struct MapCell {
    color_black: bool,
    has_robot: bool,
    has_been_colored: bool,
}

impl MapCell {
    fn new(color_black: bool) -> Self {
        MapCell {
            color_black,
            has_robot: false,
            has_been_colored: false,
        }
    }

    fn new_robot() -> Self {
        MapCell {
            color_black: true,
            has_robot: true,
            has_been_colored: false,
        }
    }
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            '.' => MapCell::new(true),
            '#' => MapCell::new(false),
            '^' => MapCell::new_robot(),
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

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy, PartialOrd, Ord, EnumIter)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
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
                if self[y][x].has_robot {
                    ch = '^'
                } else if self[y][x].color_black {
                    ch = '.'
                } else {
                    ch = '#'
                }
                print!("{ch}");
            }
            println!();
        }
    }
}

impl Matrix<bool> {
    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let mut ch = '.';
                if self[y][x] {
                    ch = '0'
                }
                print!("{ch}");
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
    dir: Direction,
    map: Matrix<MapCell>,
    cpu: Machine,
}

impl Robot {
    pub fn new(firmware: &str, starting_panel_white: bool) -> Self {
        let firmware: Vec<i64> = firmware
            .split(",")
            .map(|i| i.parse().expect("Expected i64 list"))
            .collect();

        // create square map.
        let size = 101;
        let mut map_data = Vec::<MapCell>::new();
        for _ in 0..size * size {
            map_data.push(MapCell {
                color_black: true,
                has_robot: false,
                has_been_colored: false,
            });
        }

        let mut map: Matrix<MapCell> = Matrix {
            size: Size { x: size, y: size },
            data: map_data,
        };

        // place robot at the center
        map[size / 2][size / 2].has_robot = true;
        map[size / 2][size / 2].color_black = !starting_panel_white;

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
                x: size / 2,
                y: size / 2,
            },
            dir: Direction::Up,
            map,
            cpu: machine,
        }
    }

    pub fn step(&mut self) -> Result<i64, String> {
        // Get map data as input
        let color_black = self.map[self.pos.y][self.pos.x].color_black;
        if color_black {
            self.cpu.input.push(0);
        } else {
            self.cpu.input.push(1);
        }

        // run cpu
        let (white_color_to_paint, turn_right, result) = match self.cpu.execute() {
            ExecuteResult::WaitingInput => (
                self.cpu.output[self.cpu.output_pointer - 2],
                self.cpu.output[self.cpu.output_pointer - 1],
                Ok(-1),
            ),
            ExecuteResult::Halt(_) => (0, 0, Ok(1)),
            _ => panic!(),
        };

        //dbg!()
        //dbg!(&self.dir);

        // color pos
        self.map[self.pos.y][self.pos.x].color_black = white_color_to_paint == 0;
        self.map[self.pos.y][self.pos.x].has_been_colored = true;

        // rotate
        if turn_right == 1 {
            self.dir = self.dir.turn_right();
        } else {
            self.dir = self.dir.turn_left();
        };

        // move forward
        self.map[self.pos.y][self.pos.x].has_robot = false;
        self.pos = self.pos.navigate_to(&self.map, &self.dir).unwrap();
        self.map[self.pos.y][self.pos.x].has_robot = true;

        //self.map.print();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // Program: outputs test output, ignoring input.
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        // no test, see console output.
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
