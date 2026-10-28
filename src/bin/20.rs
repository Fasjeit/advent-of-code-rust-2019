use std::fmt::Display;

use advent_of_code::advent_stdlib::Matrix;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u64> {
    let map = Matrix::<MapCell>::from_char_input(input);
    map.print();

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[derive(Debug, Clone)]
pub struct MapCell {
    is_void: bool,
    is_wall: bool,
    is_path: bool,
    is_portal: Option<char>,
}

impl MapCell {
    pub fn new_void() -> Self {
        MapCell {
            is_void: true,
            is_wall: false,
            is_path: false,
            is_portal: None,
        }
    }
    pub fn new_wall() -> Self {
        MapCell {
            is_void: false,
            is_wall: true,
            is_path: false,
            is_portal: None,
        }
    }

    pub fn new_path() -> Self {
        MapCell {
            is_void: false,
            is_wall: false,
            is_path: true,
            is_portal: None,
        }
    }

    pub fn new_portal(name: char) -> Self {
        MapCell {
            is_void: false,
            is_wall: false,
            is_path: false,
            is_portal: Some(name),
        }
    }
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            '#' => MapCell::new_wall(),
            '.' => MapCell::new_path(),
            ' ' => MapCell::new_void(),
            c if c.is_uppercase() => MapCell::new_portal(c),
            _ => panic!("Unknown char in map"),
        }
    }
}

impl Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = if self.is_wall {
            '#'
        } else if self.is_void {
            ' '
        } else if self.is_path {
            '.'
        } else if let Some(c) = self.is_portal {
            c
        } else {
            panic!("Cannot print map")
        };
        write!(f, "{}", ch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
