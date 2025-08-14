use advent_of_code::intcode::*;
use console::Term;
use std::fmt::Debug;
use std::str::FromStr;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u64> {
    let (_result, output) = run_machine_with_extended_memory(input, "").unwrap();

    let mut cells: Vec<MapCell> = Vec::<MapCell>::new();
    for _i in 0..(38 * 38) {
        cells.push(MapCell {
            cell_type: CellType::Empty,
        });
    }

    let mut map = Matrix {
        size: Size { x: 38, y: 38 },
        data: cells,
    };

    let mut output_iterator = output.iter().peekable();
    while output_iterator.peek().is_some() {
        let x = *output_iterator.next().unwrap() as usize;
        let y = *output_iterator.next().unwrap() as usize;
        let tile_id = *output_iterator.next().unwrap();

        map[y][x] = MapCell::from(tile_id);

        //map.print();
    }

    let total_block = map.data.iter().fold(0, |acc, c| {
        if let CellType::Block = c.cell_type {
            acc + 1
        } else {
            acc
        }
    });

    Some(total_block)

    //Some(1337)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cells: Vec<MapCell> = Vec::<MapCell>::new();
    let screen_size = Size { x: 38, y: 22 };

    for _i in 0..(screen_size.x * screen_size.y) {
        cells.push(MapCell {
            cell_type: CellType::Empty,
        });
    }

    let mut map = Matrix {
        size: Size {
            x: screen_size.x,
            y: screen_size.y,
        },
        data: cells,
    };

    let firmware = &format!("2{}", &input[1..]);

    let firmware: Vec<i64> = firmware
        .split(",")
        .map(|i| i.parse().expect("Expected i64 list"))
        .collect();

    let input: Vec<i64> = Vec::new();

    // extend memory to at least 4096
    let ext_memory_size = 4096;
    let mut ext_memory: Vec<i64> = Vec::with_capacity(ext_memory_size);
    for i in 0..ext_memory_size {
        if i < firmware.len() {
            ext_memory.push(firmware[i]);
        } else {
            ext_memory.push(0);
        }
    }

    let mut machine = Machine::new_with_input(ext_memory, input);

    // events
    let stdout = Term::buffered_stdout();

    let mut score = 0;

    let ai_mode = true;
    let mut input: i64;

    let mut game_running = true;

    // game loop
    while game_running {
        if !ai_mode {
            // Get inputs
            input = if let Ok(character) = stdout.read_char() {
                match character {
                    'a' => -1,
                    'd' => 1,
                    _ => 0,
                }
            } else {
                panic!("Cannot get input!")
            };
        } else {
            let (ball_x, bar_x) = map.get_ball_and_bar_pos_x();
            input = if ball_x > bar_x {
                1
            } else if ball_x < bar_x {
                -1
            } else {
                0
            };

            //// just for tests.
            // sleep(Duration::new(0, 100_000_000));
        }
        //

        machine.input.push(input);
        //dbg!(&input);
        let output = match machine.execute() {
            ExecuteResult::WaitingInput => &machine.output,
            ExecuteResult::Halt(_result) => {
                // get final score and exit
                game_running = false;
                &machine.output
            }
            ExecuteResult::Panic(m) => panic!("{}", m),
            _ => panic!("Unexpected result!"),
        };
        let mut output_iterator = output.iter().peekable();
        while output_iterator.peek().is_some() {
            //println!("{:#?}", &output_iterator.peek());
            let x = *output_iterator.next().unwrap() as usize;
            let y = *output_iterator.next().unwrap() as usize;
            let tile_id = *output_iterator.next().unwrap();

            if x == usize::MAX {
                score = tile_id as u64;
            } else {
                map[y][x] = MapCell::from(tile_id);
            }
        }

        //clean output
        machine.output = Vec::new();

        //println!("{}\n{}", map.print_to_string(), &score);
    }
    Some(score)
}

#[derive(Debug, Clone)]
enum CellType {
    Empty,
    Wall,
    Block,
    Horizontal,
    Ball,
}

#[derive(Debug, Clone)]
struct MapCell {
    cell_type: CellType,
}

// impl MapCell {
//     fn new_empty(color_black: bool) -> Self {
//         MapCell {
//             color_black,
//             has_robot: false,
//             has_been_colored: false,
//         }
//     }

//     fn new_robot() -> Self {
//         MapCell {
//             color_black: true,
//             has_robot: true,
//             has_been_colored: false,
//         }
//     }
// }

// struct Game {
//     machine: Machine,
//     map: Matrix<MapCell>,
// }

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            ' ' => MapCell {
                cell_type: CellType::Empty,
            },
            '@' => MapCell {
                cell_type: CellType::Wall,
            },
            '#' => MapCell {
                cell_type: CellType::Block,
            },
            '^' => MapCell {
                cell_type: CellType::Horizontal,
            },
            'o' => MapCell {
                cell_type: CellType::Ball,
            },
            _ => panic!("Unknown char in map data!"),
        }
    }
}

impl From<i64> for MapCell {
    fn from(value: i64) -> Self {
        match value {
            0 => MapCell {
                cell_type: CellType::Empty,
            },
            1 => MapCell {
                cell_type: CellType::Wall,
            },
            2 => MapCell {
                cell_type: CellType::Block,
            },
            3 => MapCell {
                cell_type: CellType::Horizontal,
            },
            4 => MapCell {
                cell_type: CellType::Ball,
            },
            _ => panic!("Unknown type in map data!"),
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
    fn get_ball_and_bar_pos_x(&self) -> (usize, usize) {
        let mut ball_x = 0;
        let mut bar_x = 0;
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                if ball_x != 0 && bar_x != 0 {
                    break;
                }
                match self[y][x].cell_type {
                    CellType::Horizontal => bar_x = x,
                    CellType::Ball => ball_x = x,
                    _ => (),
                };
            }
        }
        (ball_x, bar_x)
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let ch = match self[y][x].cell_type {
                    CellType::Empty => ' ',
                    CellType::Wall => '@',
                    CellType::Block => '#',
                    CellType::Horizontal => '^',
                    CellType::Ball => 'o',
                };
                print!("{ch}");
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print_to_string(&self) -> String {
        let mut result = "".to_string();
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let ch = match self[y][x].cell_type {
                    CellType::Empty => ' ',
                    CellType::Wall => '@',
                    CellType::Block => '#',
                    CellType::Horizontal => '^',
                    CellType::Ball => 'o',
                };
                result.push(ch);
            }
            result.push('\n');
        }
        result
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        // as part 2 set first instruction 104 -> 204 we need to
        // consider it using relative mode for first arg (see 13-2).
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1337));
    }
}
