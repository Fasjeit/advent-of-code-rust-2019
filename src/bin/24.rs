use std::{collections::HashSet, fmt::Display};

use advent_of_code::advent_stdlib::{Direction, Index, Matrix, Size};

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

pub fn part_two(input: &str) -> Option<u64> {
    solve_part_two_with_ticks(input, 200)
}

fn solve_part_two_with_ticks(input: &str, ticks: u64) -> Option<u64> {
    //      |     |         |     |
    //   1  |  2  |    3    |  4  |  5
    //      |     |         |     |
    // -----+-----+---------+-----+-----
    //      |     |         |     |
    //   6  |  7  |    8    |  9  |  10
    //      |     |         |     |
    // -----+-----+---------+-----+-----
    //      |     |A|B|C|D|E|     |
    //      |     |-+-+-+-+-|     |
    //      |     |F|G|H|I|J|     |
    //      |     |-+-+-+-+-|     |
    //  11  | 12  |K|L|?|N|O|  14 |  15
    //      |     |-+-+-+-+-|     |
    //      |     |P|Q|R|S|T|     |
    //      |     |-+-+-+-+-|     |
    //      |     |U|V|W|X|Y|     |
    // -----+-----+---------+-----+-----
    //      |     |         |     |
    //  16  | 17  |    18   |  19 |  20
    //      |     |         |     |
    // -----+-----+---------+-----+-----
    //      |     |         |     |
    //  21  | 22  |    23   |  24 |  25
    //      |     |         |     |

    // 200 ticks - no more then 200 levels from base one.
    let mut maps = Vec::<Matrix<MapCell>>::new();
    for i in 0..401 {
        if i == 200 {
            maps.push(Matrix::<MapCell>::from_char_input(input.trim_end()));
        } else {
            maps.push(Matrix {
                size: Size { x: 5, y: 5 },
                data: vec![MapCell::new(false); 25],
            });
        }
    }

    for _i in 0..ticks {
        maps = tick_multi_dim(&maps);
    }

    Some(count_bugs_multi_dim(&maps))
}

fn count_bugs_multi_dim(maps: &[Matrix<MapCell>]) -> u64 {
    let mut bugs = 0;
    for map in maps {
        for cell in &map.data {
            if cell.has_bug {
                bugs += 1
            }
        }
    }
    bugs
}

fn tick_multi_dim(maps: &[Matrix<MapCell>]) -> Vec<Matrix<MapCell>> {
    let mut next_maps = maps.to_vec();
    for level in 0..maps.len() {
        for pos in 0..maps[level].data.len() {
            if pos == 12 {
                // Skip center - never infested
                continue;
            }
            let index = maps[level].get_index_from_position(pos);
            next_maps[level][index] =
                maps[level][index].get_next_state_multi_dim(level, index, maps);
        }
    }
    next_maps
}

fn count_adjacent_bugs_multi_dim(level: usize, pos: &Index, maps: &[Matrix<MapCell>]) -> u64 {
    let mut bugs = 0;
    for dir in [
        Direction::Down,
        Direction::Up,
        Direction::Left,
        Direction::Right,
    ] {
        if let Some(adj_pos) = pos.navigate_to(&maps[level], &dir) {
            if adj_pos == (Index { x: 2, y: 2 }) {
                // inner adjacent
                if level > 0 {
                    bugs += get_inner_adjacent_bugs_for_direction(dir, level - 1, maps);
                }
            } else if maps[level][adj_pos].has_bug {
                bugs += 1;
            }
        } else if level + 1 < maps.len() {
            // outer adjacent
            bugs += get_outer_adjacent_bugs_for_direction(dir, level + 1, maps);
        }
    }

    bugs
}

fn get_outer_adjacent_bugs_for_direction(
    dir: Direction,
    level: usize,
    maps: &[Matrix<MapCell>],
) -> u64 {
    // Hard coded indexes, see table above
    match dir {
        Direction::Up => maps[level].data[7].has_bug as u64,
        Direction::Left => maps[level].data[11].has_bug as u64,
        Direction::Right => maps[level].data[13].has_bug as u64,
        Direction::Down => maps[level].data[17].has_bug as u64,
    }
}

fn get_inner_adjacent_bugs_for_direction(
    dir: Direction,
    level: usize,
    maps: &[Matrix<MapCell>],
) -> u64 {
    // Hard coded indexes, see table above
    match dir {
        Direction::Up => {
            (maps[level].data[20].has_bug as u64)
                + (maps[level].data[21].has_bug as u64)
                + (maps[level].data[22].has_bug as u64)
                + (maps[level].data[23].has_bug as u64)
                + (maps[level].data[24].has_bug as u64)
        }
        Direction::Left => {
            (maps[level].data[4].has_bug as u64)
                + (maps[level].data[9].has_bug as u64)
                + (maps[level].data[14].has_bug as u64)
                + (maps[level].data[19].has_bug as u64)
                + (maps[level].data[24].has_bug as u64)
        }
        Direction::Right => {
            (maps[level].data[0].has_bug as u64)
                + (maps[level].data[5].has_bug as u64)
                + (maps[level].data[10].has_bug as u64)
                + (maps[level].data[15].has_bug as u64)
                + (maps[level].data[20].has_bug as u64)
        }
        Direction::Down => {
            (maps[level].data[0].has_bug as u64)
                + (maps[level].data[1].has_bug as u64)
                + (maps[level].data[2].has_bug as u64)
                + (maps[level].data[3].has_bug as u64)
                + (maps[level].data[4].has_bug as u64)
        }
    }
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

    pub fn get_next_state_multi_dim(
        &self,
        level: usize,
        pos: Index,
        maps: &[Matrix<MapCell>],
    ) -> MapCell {
        let adjacent_bugs = count_adjacent_bugs_multi_dim(level, &pos, maps);
        if self.has_bug && adjacent_bugs != 1 {
            MapCell { has_bug: false }
        } else if adjacent_bugs == 1 || adjacent_bugs == 2 {
            MapCell { has_bug: true }
        } else {
            self.clone()
        }
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
        let result = solve_part_two_with_ticks(
            &advent_of_code::template::read_file_part("examples", DAY, 1),
            10,
        );
        assert_eq!(result, Some(99));
    }
}
