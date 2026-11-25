use std::{collections::HashSet, fmt::Display};

use advent_of_code::advent_stdlib::{Direction, Index, Matrix};

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = Matrix::<MapCell>::from_char_input(input.trim_end());
    let mut set = HashSet::new();

    loop {
        map = tick(&map);
        //map.print();
        //println!("============");
        let hash = hash(&map);
        if set.contains(&hash) {
            //println!("RESULT:{}", hash);
            //map.print();
            return Some(hash);
        } else {
            set.insert(hash);
        }
    }
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

fn tick(map: &Matrix<MapCell>) -> Matrix<MapCell> {
    let mut next_map: Matrix<MapCell> = map.clone();
    for pos in 0..map.data.len() {
        let index = map.get_index_from_position(pos);
        next_map[index] = map[index].get_next_state(index, map);
    }
    next_map
}

fn hash(map: &Matrix<MapCell>) -> u64 {
    let mut hash = 0;
    for pos in (0..map.data.len()).rev() {
        hash <<= 1;
        let index = map.get_index_from_position(pos);
        if map[index].has_bug {
            hash |= 1;
        }
    }
    hash
}

#[derive(Debug, Clone)]
pub struct MapCell {
    pub has_bug: bool,
}

impl MapCell {
    pub fn new(has_bug: bool) -> Self {
        MapCell { has_bug }
    }

    pub fn get_next_state(&self, pos: Index, map: &Matrix<MapCell>) -> MapCell {
        let adjacent_bugs = MapCell::count_adjacent_bugs(&pos, map);
        if self.has_bug && adjacent_bugs != 1 {
            MapCell { has_bug: false }
        } else if adjacent_bugs == 1 || adjacent_bugs == 2 {
            MapCell { has_bug: true }
        } else {
            self.clone()
        }
    }

    fn count_adjacent_bugs(pos: &Index, map: &Matrix<MapCell>) -> u64 {
        let mut bugs = 0;
        for dir in [
            Direction::Down,
            Direction::Up,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(adj_pos) = pos.navigate_to(map, &dir)
                && map[adj_pos].has_bug
            {
                bugs += 1;
            }
        }

        bugs
    }
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            '.' => MapCell::new(false),
            '#' => MapCell::new(true),
            _ => panic!("Unknown char in map data!"),
        }
    }
}

impl Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ch = '.';
        if self.has_bug {
            ch = '#'
        }
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
        assert_eq!(result, Some(2129920));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
