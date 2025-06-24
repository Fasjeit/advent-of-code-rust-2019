use std::collections::HashSet;

use advent_of_code::advent_stdlib::{parse_row_input_as_data_array, Index, Size};
use fixed::types::I20F12;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let (_, value) = find_best_position(input);
    Some(value as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let position = find_best_position_and_fire(input);
    Some((100 * position.x + position.y) as u64)
}

fn find_best_position(input: &str) -> (Index, u32) {
    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<AsteroidMapCell> = data.into_iter().map(AsteroidMapCell::from).collect();

    let matrix = SpaceMatrix {
        size,
        data: data_cells.clone(),
    };

    find_best_position_matrix(&matrix)
}

fn find_best_position_and_fire(input: &str) -> Index {
    let (data, size) = parse_row_input_as_data_array::<char>(input);
    let data_cells: Vec<AsteroidMapCell> = data.into_iter().map(AsteroidMapCell::from).collect();

    let mut matrix = SpaceMatrix {
        size,
        data: data_cells.clone(),
    };

    let (position, _) = find_best_position_matrix(&matrix);
    fire_for_position(&mut matrix, position);

    todo!()
}

fn find_best_position_matrix(matrix: &SpaceMatrix<AsteroidMapCell>) -> (Index, u32) {
    let mut best_index: Index = Index { x: 0, y: 0 };
    let mut best_value: u32 = 0;

    for y in 0..matrix.size.y {
        for x in 0..matrix.size.x {
            let test_result = test_for_position(matrix, Index { x, y });
            if best_value < test_result {
                best_value = test_result;
                best_index = Index { x, y };
            }
        }
    }
    (best_index, best_value)
}

fn test_for_position(matrix: &SpaceMatrix<AsteroidMapCell>, position: Index) -> u32 {
    if !matrix[position.y][position.x].has_asteroid {
        return 0;
    }

    let mut result = 0;

    let mut matrix = matrix.clone();

    matrix[position.y][position.x].source = true;

    let fth_quadrant_size = Size {
        x: matrix.size.x - position.x,
        y: matrix.size.y - position.y,
    };

    // Need to block upper and lower part of the map separately!
    // Or this will happen.
    //.@...
    //.....
    //..S..
    //.....
    //...#.

    //println!("\n Checking 4");

    let mut blocked_4 = HashSet::<I20F12>::new();
    for dy in 0..fth_quadrant_size.y as i32 {
        for dx in 0..fth_quadrant_size.x as i32 {
            result += check_quadrant(dx, dy, &position, &mut matrix, &mut blocked_4);
        }
    }

    //println!("\n Checking 3");

    let mut blocked_3 = HashSet::<I20F12>::new();
    for dy in 0..fth_quadrant_size.y as i32 {
        for dx in (-(position.x as i32)..0).rev() {
            result += check_quadrant(dx, dy, &position, &mut matrix, &mut blocked_3);
        }
    }

    //println!("\n Checking 2");

    let mut blocked_2 = HashSet::<I20F12>::new();
    for dy in (-(position.y as i32)..0).rev() {
        for dx in (-(position.x as i32)..0).rev() {
            result += check_quadrant(dx, dy, &position, &mut matrix, &mut blocked_2);
        }
    }

    //println!("\n Checking 1");

    let mut blocked_1 = HashSet::<I20F12>::new();
    for dy in (-(position.y as i32)..0).rev() {
        for dx in 0..fth_quadrant_size.x as i32 {
            result += check_quadrant(dx, dy, &position, &mut matrix, &mut blocked_1);
        }
    }

    //matrix.print();

    result
}

fn fire_for_position(matrix: &mut SpaceMatrix<AsteroidMapCell>, position: Index) -> Index {
    //let mut index200 = Index { x: 0, y: 0 };
    let mut iterations_index = 0;

    matrix[position.y][position.x].source = true;

    let fth_quadrant_size = Size {
        x: matrix.size.x - position.x,
        y: matrix.size.y - position.y,
    };

    // Need to block upper and lower part of the map separately!
    // Or this will happen.
    //.@...
    //.....
    //..S..
    //.....
    //...#.

    //println!("\n Checking 1");

    let mut blocked_1 = HashSet::<I20F12>::new();
    for dx in 0..fth_quadrant_size.x as i32 {
        for dy in -(position.y as i32)..1 {
            fire_quadrant(
                dx,
                dy,
                &position,
                matrix,
                &mut blocked_1,
                &mut iterations_index,
            );
        }
    }

    //println!("\n Checking 2");

    let mut blocked_2 = HashSet::<I20F12>::new();
    for dy in (-(position.y as i32)..0).rev() {
        for dx in (-(position.x as i32)..0).rev() {
            fire_quadrant(
                dx,
                dy,
                &position,
                matrix,
                &mut blocked_2,
                &mut iterations_index,
            );
        }
    }

    //println!("\n Checking 3");

    let mut blocked_3 = HashSet::<I20F12>::new();
    for dy in 0..fth_quadrant_size.y as i32 {
        for dx in (-(position.x as i32)..0).rev() {
            fire_quadrant(
                dx,
                dy,
                &position,
                matrix,
                &mut blocked_3,
                &mut iterations_index,
            );
        }
    }

    //println!("\n Checking 4");

    let mut blocked_4 = HashSet::<I20F12>::new();
    for dy in 0..fth_quadrant_size.y as i32 {
        for dx in 0..fth_quadrant_size.x as i32 {
            fire_quadrant(
                dx,
                dy,
                &position,
                matrix,
                &mut blocked_4,
                &mut iterations_index,
            );
        }
    }

    matrix.print();

    todo!()
}

fn check_quadrant(
    dx: i32,
    dy: i32,
    position: &Index,
    matrix: &mut SpaceMatrix<AsteroidMapCell>,
    blocked: &mut HashSet<I20F12>,
) -> u32 {
    if dx == 0 && dy == 0 {
        return 0;
    }
    let mut result = 0;

    let inf = I20F12::from_num(999);

    let x = (dx + position.x as i32) as usize;
    let y = (dy + position.y as i32) as usize;
    //println!("[x:{x} y:{y}]");
    //println!("[dx:{dx} dy:{dy}]");

    if matrix[y][x].has_asteroid {
        //println!("Asteroid detected!");

        let view = if dx != 0 {
            I20F12::from_num(dy) / dx
        } else {
            inf
        };
        if blocked.contains(&view) {
            //println!("blocked! [{:?}]", &view);
        } else {
            //println!("blocking [{:?}]", &view);
            blocked.insert(view);
            matrix[y][x].cost = 1; // evaporating!
            result += 1;
        }
    }

    result
}

fn fire_quadrant(
    dx: i32,
    dy: i32,
    position: &Index,
    matrix: &mut SpaceMatrix<AsteroidMapCell>,
    blocked: &mut HashSet<I20F12>,
    shots_left: &mut u32,
) -> Option<Index> {
    if dx == 0 && dy == 0 {
        return None;
    }

    let inf = I20F12::from_num(999);

    let x = (dx + position.x as i32) as usize;
    let y = (dy + position.y as i32) as usize;
    //println!("[x:{x} y:{y}]");
    //println!("[dx:{dx} dy:{dy}]");

    if matrix[y][x].has_asteroid && matrix[y][x].cost == 0 {
        //println!("Asteroid detected!");

        let view = if dx != 0 {
            I20F12::from_num(dy) / dx
        } else {
            inf
        };
        if blocked.contains(&view) {
            //println!("blocked! [{:?}]", &view);
        } else {
            //println!("blocking [{:?}]", &view);
            blocked.insert(view);
            matrix[y][x].cost = *shots_left as u64; // evaporating!
            *shots_left += 1;
            return Some(Index { x, y });
        }
    }
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
                if self[y][x].cost > 9 {
                    ch = '@'
                } else if self[y][x].cost != 0 {
                    ch = char::from_digit(self[y][x].cost as u32, 10).unwrap()
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
            _ => AsteroidMapCell::new_with_asteroid(true),
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
        let (data, size) = parse_row_input_as_data_array::<char>(
            &advent_of_code::template::read_file_part("examples", DAY, 1),
        );
        let data_cells: Vec<AsteroidMapCell> =
            data.into_iter().map(AsteroidMapCell::from).collect();
        let matrix = SpaceMatrix {
            size,
            data: data_cells.clone(),
        };
        let res = test_for_position(&matrix, Index { x: 2, y: 2 });
        assert_eq!(res, 12);
    }

    #[test]
    fn test_part_one_2() {
        let (data, size) = parse_row_input_as_data_array::<char>(
            &advent_of_code::template::read_file_part("examples", DAY, 2),
        );
        let data_cells: Vec<AsteroidMapCell> =
            data.into_iter().map(AsteroidMapCell::from).collect();
        let matrix = SpaceMatrix {
            size,
            data: data_cells.clone(),
        };
        let res = test_for_position(&matrix, Index { x: 3, y: 4 });
        assert_eq!(res, 8);
    }

    #[test]
    fn test_part_one_2_best() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_one_3() {
        let (data, size) = parse_row_input_as_data_array::<char>(
            &advent_of_code::template::read_file_part("examples", DAY, 3),
        );
        let data_cells: Vec<AsteroidMapCell> =
            data.into_iter().map(AsteroidMapCell::from).collect();
        let matrix = SpaceMatrix {
            size,
            data: data_cells.clone(),
        };
        let res = test_for_position(&matrix, Index { x: 0, y: 0 });
        assert_eq!(res, 7);
    }

    #[test]
    fn test_part_one_4() {
        let (data, size) = parse_row_input_as_data_array::<char>(
            &advent_of_code::template::read_file_part("examples", DAY, 4),
        );
        let data_cells: Vec<AsteroidMapCell> =
            data.into_iter().map(AsteroidMapCell::from).collect();
        let matrix = SpaceMatrix {
            size,
            data: data_cells.clone(),
        };
        let res = test_for_position(&matrix, Index { x: 5, y: 8 });
        assert_eq!(res, 33);
    }
    #[test]
    fn test_part_one_4_best() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part_one_5() {
        let (data, size) = parse_row_input_as_data_array::<char>(
            &advent_of_code::template::read_file_part("examples", DAY, 5),
        );
        let data_cells: Vec<AsteroidMapCell> =
            data.into_iter().map(AsteroidMapCell::from).collect();
        let matrix = SpaceMatrix {
            size,
            data: data_cells.clone(),
        };
        let res = test_for_position(&matrix, Index { x: 1, y: 2 });
        assert_eq!(res, 35);
    }

    #[test]
    fn test_part_one_5_best() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_one_6() {
        let (data, size) = parse_row_input_as_data_array::<char>(
            &advent_of_code::template::read_file_part("examples", DAY, 6),
        );
        let data_cells: Vec<AsteroidMapCell> =
            data.into_iter().map(AsteroidMapCell::from).collect();
        let matrix = SpaceMatrix {
            size,
            data: data_cells.clone(),
        };
        let res = test_for_position(&matrix, Index { x: 6, y: 3 });
        assert_eq!(res, 41);
    }

    #[test]
    fn test_part_one_6_best() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_one_7() {
        let (data, size) = parse_row_input_as_data_array::<char>(
            &advent_of_code::template::read_file_part("examples", DAY, 7),
        );
        let data_cells: Vec<AsteroidMapCell> =
            data.into_iter().map(AsteroidMapCell::from).collect();
        let matrix = SpaceMatrix {
            size,
            data: data_cells.clone(),
        };
        let res = test_for_position(&matrix, Index { x: 11, y: 13 });
        assert_eq!(res, 210);
    }

    #[test]
    fn test_part_one_7_best() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 7,
        ));
        assert_eq!(result, Some(210));
    }

    //#[test]
    // fn test_part_two() {
    //     let (data, size) = parse_row_input_as_data_array::<char>(
    //         &advent_of_code::template::read_file_part("examples", DAY, 8),
    //     );
    //     let data_cells: Vec<AsteroidMapCell> =
    //         data.into_iter().map(AsteroidMapCell::from).collect();
    //     let matrix = SpaceMatrix {
    //         size,
    //         data: data_cells.clone(),
    //     };
    //     //let res = fire_for_position(&mut matrix, Index { x: 8, y: 3 });
    //     //assert_eq!(res, 210);
    // }
}
