use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

use advent_of_code::advent_stdlib::{Direction, Index, Matrix};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = Matrix::<MapCell>::from_char_input(input);
    //map.print();

    let (portal_links, start, end) = link_portals_part_1(&mut map);
    //dbg!(&portal_links);

    let mut to_visit_set = BinaryHeap::<Reverse<(u64, Index)>>::new();
    to_visit_set.push(Reverse((0_u64, start)));
    let cost = pseudo_dijkstra_part_1(&mut map, Some(&end), &mut to_visit_set, &portal_links);

    //map.print();
    cost
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = Matrix::<MapCell>::from_char_input(input);

    let (portal_links, start, end) = link_portals_part_2(&mut map);

    pseudo_dijkstra_part_2(&mut map, start, end, portal_links)
}

pub fn pseudo_dijkstra_part_1(
    matrix: &mut Matrix<MapCell>,
    ending_position: Option<&Index>,
    to_visit_set: &mut BinaryHeap<Reverse<(u64, Index)>>,
    portal_links: &HashMap<Index, Index>,
) -> Option<u64> {
    let mut safe_counter = 100_000;

    while let Some(Reverse((cost, index))) = to_visit_set.pop() {
        //matrix.print();

        if safe_counter <= 0 {
            panic!("Safe counter stop.");
        }
        safe_counter -= 1;

        if matrix[index].cost[&0] != u64::MAX {
            assert!(matrix[index].cost[&0] <= cost);
            continue;
        }

        matrix[index].cost.insert(0, cost);

        //dbg!(&index);

        if let Some(ending_position) = ending_position
            && index == *ending_position
        {
            return Some(cost);
        }

        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(next_index) = index.navigate_to(matrix, &dir) {
                if matrix[index].is_portal.is_some() && matrix[next_index].is_portal.is_some() {
                    //XY.
                    //|^- we are here
                    //^ - the next index
                    // just step out of teleport, no need to go back (also we find no links as we still "inside")
                    continue;
                }

                //dbg!(&next_index);

                if matrix[next_index].is_path && matrix[next_index].cost[&0] >= (cost + 1) {
                    // Just a path
                    to_visit_set.push(Reverse((cost + 1, next_index)));
                } else if matrix[next_index].is_portal.is_some()
                    && !matrix[next_index].is_wall
                    && matrix[portal_links[&next_index]].cost[&0] >= cost
                {
                    // Portal path - set use link for next index to use
                    let portal_exit_link = portal_links[&next_index];
                    to_visit_set.push(Reverse((cost, portal_exit_link)));
                }
            }
        }
    }

    None
}

pub fn pseudo_dijkstra_part_2(
    matrix: &mut Matrix<MapCell>,
    start: Index,
    end: Index,
    portal_links: HashMap<Index, (Index, i64)>,
) -> Option<u64> {
    let mut to_visit_set = BinaryHeap::<Reverse<(u64, u64, Index)>>::new();

    let mut safe_counter = 1_000_000;

    // (cost, recursion_level, position)
    to_visit_set.push(Reverse((0, 0, start)));

    while let Some(Reverse((cost, level, index))) = to_visit_set.pop() {
        //matrix.print();

        if safe_counter <= 0 {
            panic!("Safe counter stop.");
        }
        safe_counter -= 1;

        // This position was already visited at this recursion level.
        let previous_cost = matrix[index].cost.get(&level).copied().unwrap_or(u64::MAX);

        // can just check as before
        // if previous_cost != u64::MAX {
        //       assert!(previous_cost <= cost);
        //       continue;
        // }
        if previous_cost <= cost {
            continue;
        }

        matrix[index].cost.insert(level, cost);

        //dbg!(&index);

        if index == end && level == 0 {
            return Some(cost);
        }

        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(next_index) = index.navigate_to(matrix, &dir) {
                if matrix[index].is_portal.is_some() && matrix[next_index].is_portal.is_some() {
                    //XY.
                    //|^- we are here
                    //^ - the next index
                    // just step out of teleport, no need to go back (also we find no links as we still "inside")
                    continue;
                }

                //dbg!(&next_index);

                // Ordinary path.
                if matrix[next_index].is_path {
                    // AA and ZZ are walls on recursive levels.
                    if level > 0 && (next_index == start || next_index == end) {
                        continue;
                    }

                    let next_cost = cost + 1;

                    let previous_cost = matrix[next_index]
                        .cost
                        .get(&level)
                        .copied()
                        .unwrap_or(u64::MAX);

                    if previous_cost > next_cost {
                        to_visit_set.push(Reverse((next_cost, level, next_index)));
                    }
                }
                // Portal.
                else if matrix[next_index].is_portal.is_some() && !matrix[next_index].is_wall {
                    let Some(&(portal_exit, level_change)) = portal_links.get(&next_index) else {
                        continue;
                    };

                    let next_level = level as i64 + level_change;

                    // Cannot go outside the outermost level.
                    if next_level < 0 {
                        continue;
                    }

                    let next_level = next_level as u64;

                    let previous_cost = matrix[portal_exit]
                        .cost
                        .get(&next_level)
                        .copied()
                        .unwrap_or(u64::MAX);

                    // Portal traversal itself has zero cost here.
                    if previous_cost > cost {
                        to_visit_set.push(Reverse((cost, next_level, portal_exit)));
                    }
                }
            }
        }
    }

    None
}

fn link_portals_part_1(map: &mut Matrix<MapCell>) -> (HashMap<Index, Index>, Index, Index) {
    // check cell
    // - is portal
    // - next to portal (x)
    // - next to entrance (y) oppisite other portal
    // if so - add the the portal links
    //   ########
    //   ........BC
    //   ########
    // BC.......
    //   ########
    // portal links
    // pos - pos (and revelse links)
    let mut portal_links_to_add = HashMap::<(char, char), Index>::new();
    let mut portal_links = HashMap::new();
    let mut start = Index { x: 0, y: 0 };
    let mut end = Index { x: 0, y: 0 };
    let mut edge_portals = Vec::new();

    for (pos, _cell) in map.data.iter().enumerate() {
        let index = map.get_index_from_position(pos);
        // if index.x == 11 && index.y == 11 {
        //     dbg!("HERE!");
        // }
        // if index.x == 1 && index.y == 15 {
        //     dbg!("HERE2!");
        // }
        if let Some(current_portal) = map[index].is_portal {
            for dir in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                if let Some(x) = index.navigate_to(map, &dir)
                    && let Some(other_potral) = map[x].is_portal
                    && let Some(y) = index.navigate_to(map, &dir.reverse())
                    && map[y].is_path
                {
                    // If Start or end
                    if current_portal == other_potral {
                        if current_portal == 'A' {
                            start = y;
                            edge_portals.push(index);
                            edge_portals.push(x);
                            continue;
                        } else if current_portal == 'Z' {
                            end = y;
                            edge_portals.push(index);
                            edge_portals.push(x);
                            continue;
                        }
                        //dbg!(&current_portal);
                    }
                    // Add this one to portal links if already exists other link
                    // Links can be both .CD  DC. and .CD  CD.
                    if portal_links_to_add
                        .contains_key(&normalize_portal_pair(other_potral, current_portal))
                    {
                        let other_side_index = portal_links_to_add
                            [&normalize_portal_pair(other_potral, current_portal)];
                        portal_links_to_add
                            .remove(&normalize_portal_pair(other_potral, current_portal));
                        // create the links
                        portal_links.insert(index, other_side_index);
                        portal_links.insert(other_side_index, index);
                    }
                    // Or just save it
                    else {
                        portal_links_to_add
                            .insert(normalize_portal_pair(current_portal, other_potral), index);
                    }
                }
            }
        }
    }

    if !portal_links_to_add.is_empty() {
        dbg!(&portal_links_to_add);
        panic!("Not all portal linked!")
    }

    // Making edge portals walls
    for index in edge_portals {
        map[index].is_wall = true;
    }

    (portal_links, start, end)
}

fn link_portals_part_2(map: &mut Matrix<MapCell>) -> (HashMap<Index, (Index, i64)>, Index, Index) {
    // check cell
    // - is portal
    // - next to portal (x)
    // - next to entrance (y) oppisite other portal
    // if so - add the the portal links
    //   ########
    //   ........BC
    //   ########
    // BC.......
    //   ########
    // portal links
    // pos - pos (and revelse links)
    let mut portal_links_to_add = HashMap::<(char, char), Index>::new();
    let mut portal_links = HashMap::new();
    let mut start = Index { x: 0, y: 0 };
    let mut end = Index { x: 0, y: 0 };
    let mut edge_portals = Vec::new();

    for (pos, _cell) in map.data.iter().enumerate() {
        let index = map.get_index_from_position(pos);
        // if index.x == 11 && index.y == 11 {
        //     dbg!("HERE!");
        // }
        // if index.x == 1 && index.y == 15 {
        //     dbg!("HERE2!");
        // }
        if let Some(current_portal) = map[index].is_portal {
            for dir in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                if let Some(x) = index.navigate_to(map, &dir)
                    && let Some(other_potral) = map[x].is_portal
                    && let Some(y) = index.navigate_to(map, &dir.reverse())
                    && map[y].is_path
                {
                    // If Start or end
                    if current_portal == other_potral {
                        if current_portal == 'A' {
                            start = y;
                            edge_portals.push(index);
                            edge_portals.push(x);
                            continue;
                        } else if current_portal == 'Z' {
                            end = y;
                            edge_portals.push(index);
                            edge_portals.push(x);
                            continue;
                        }
                        //dbg!(&current_portal);
                    }
                    // Add this one to portal links if already exists other link
                    // Links can be both .CD  DC. and .CD  CD.
                    if portal_links_to_add
                        .contains_key(&normalize_portal_pair(other_potral, current_portal))
                    {
                        let other_side_index = portal_links_to_add
                            [&normalize_portal_pair(other_potral, current_portal)];
                        portal_links_to_add
                            .remove(&normalize_portal_pair(other_potral, current_portal));
                        // create the links

                        let is_outer = index.x <= 2
                            || index.y <= 2
                            || index.x >= map.size.x - 3
                            || index.y >= map.size.y - 3;

                        let recursion_level_change = if !is_outer {
                            // INNER TO OUTER
                            1
                        } else {
                            // OUTER TO INNER
                            -1
                        };
                        portal_links.insert(index, (other_side_index, recursion_level_change));
                        portal_links.insert(other_side_index, (index, -recursion_level_change));
                    }
                    // Or just save it
                    else {
                        portal_links_to_add
                            .insert(normalize_portal_pair(current_portal, other_potral), index);
                    }
                }
            }
        }
    }

    if !portal_links_to_add.is_empty() {
        dbg!(&portal_links_to_add);
        panic!("Not all portal linked!")
    }

    // Making edge portals walls
    for index in edge_portals {
        map[index].is_wall = true;
    }

    (portal_links, start, end)
}

fn normalize_portal_pair(a: char, b: char) -> (char, char) {
    // AB is the same as BA, so need one key for both
    if a <= b { (a, b) } else { (b, a) }
}

#[derive(Debug, Clone)]
pub struct MapCell {
    is_void: bool,
    is_wall: bool,
    is_path: bool,
    is_portal: Option<char>,
    cost: HashMap<u64, u64>,
}

impl MapCell {
    pub fn new_void() -> Self {
        MapCell {
            is_void: true,
            is_wall: false,
            is_path: false,
            is_portal: None,
            cost: HashMap::from([(0, u64::MAX)]),
        }
    }
    pub fn new_wall() -> Self {
        MapCell {
            is_void: false,
            is_wall: true,
            is_path: false,
            is_portal: None,
            cost: HashMap::from([(0, u64::MAX)]),
        }
    }

    pub fn new_path() -> Self {
        MapCell {
            is_void: false,
            is_wall: false,
            is_path: true,
            is_portal: None,
            cost: HashMap::from([(0, u64::MAX)]),
        }
    }

    pub fn new_portal(name: char) -> Self {
        MapCell {
            is_void: false,
            is_wall: false,
            is_path: false,
            is_portal: Some(name),
            cost: HashMap::from([(0, u64::MAX)]),
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
        let ch =
        //  if self.cost != u64::MAX {
        //     char::from_digit((self.cost % 10) as u32, 10).unwrap()
        // } else
        if self.is_wall {
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
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(58));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(26));
    }

    #[test]
    #[should_panic(expected = "Safe counter stop.")]
    fn test_part_two_2() {
        part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(396))
    }
}
