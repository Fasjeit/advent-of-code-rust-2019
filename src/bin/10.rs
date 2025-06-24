use std::{collections::HashSet, ffi::os_str::Display, fmt};

use advent_of_code::advent_stdlib::{parse_row_input_as_data_array, Index, Matrix, Size};
use fixed::{
    types::{extra::U24, I20F12},
    FixedI32,
};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<AsteroidMapCell> = data.into_iter().map(AsteroidMapCell::from).collect();

    let mut matrix = SpaceMatrix {
        size,
        data: data_cells.clone(),
    };

    matrix.print();

    test_for_possition(&matrix, Index { x: 2, y: 2 });

    None
}

fn test_for_possition(matrix: &SpaceMatrix<AsteroidMapCell>, position: Index) {
    let mut matrix = matrix.clone();

    matrix[position.y][position.x].source = true;

    let fth_quadrant_size = Size {
        x: matrix.size.x - position.x,
        y: matrix.size.y - position.y,
    };

    let mut blocked = HashSet::<(I20F12, bool)>::new();

    for y in 0..matrix.size.y {
        for x in 0..matrix.size.x {
            let dx = x as i32 - position.x as i32;
            let dy = y as i32 - position.y as i32;
            if dx == 0 || dy == 0 {
                continue;
            }
            if matrix[y][x].has_asteroid {
                println!("[x:{x} y:{y}]");
                println!("[dx:{dx} dy:{dy}]");

                // Need to block upper and lower part of the map separately!
                // Or this will happen.
                //.@...
                //.....
                //..S..
                //.....
                //...#.
                let is_upper = dy < 0;

                let view = I20F12::from_num(dy) / dx;
                if blocked.contains(&(view, is_upper)) {
                    println!("blocked! [{:?}]", &view);
                } else {
                    println!("blocking [{:?}]", &view);
                    blocked.insert((view, is_upper));
                    matrix[y][x].cost = 1;
                }
            }
        }
    }

    matrix.print();
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[derive(Debug, Clone)]
pub struct SpaceMatrix<T> {
    pub size: Size,
    pub data: Vec<T>,
}

impl<T> SpaceMatrix<T> {
    #[allow(dead_code)]
    pub fn get_index_from_position(&self, indx: usize) -> Index {
        let y = indx / self.size.x;
        let x = indx - y * self.size.x;
        Index { x, y }
    }
}

impl SpaceMatrix<AsteroidMapCell> {
    #[allow(dead_code)]
    pub fn has_index(&self, index: &Index) -> bool {
        self.size.x > index.x && self.size.y > index.y
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let mut ch = '.';
                if self[y][x].cost != 0 {
                    ch = '@'
                } else if self[y][x].source {
                    ch = 'S'
                } else if self[y][x].has_asteroid {
                    ch = '#'
                }
                print!("{}", ch);
            }
            println!();
        }
    }
}

#[derive(Debug, Clone)]
pub struct AsteroidMapCell {
    has_asteroid: bool,
    source: bool,
    cost: u64,
}

impl AsteroidMapCell {
    fn new_with_asteroid(has_asteroid: bool) -> Self {
        AsteroidMapCell {
            has_asteroid,
            source: false,
            cost: 0,
        }
    }
}

impl From<char> for AsteroidMapCell {
    fn from(value: char) -> Self {
        match value {
            '.' => AsteroidMapCell::new_with_asteroid(false),
            '#' => AsteroidMapCell::new_with_asteroid(true),
            _ => panic!("Unknown char in map data!"),
        }
    }
}

impl<T> std::ops::Index<usize> for SpaceMatrix<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &[T] {
        let start = row * self.size.x;
        &self.data[start..start + self.size.x]
    }
}

impl<T> std::ops::IndexMut<usize> for SpaceMatrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut [T] {
        let start = row * self.size.x;
        &mut self.data[start..start + self.size.x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
    }
}
