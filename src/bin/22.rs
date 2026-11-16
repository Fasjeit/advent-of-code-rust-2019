advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve_part_one(input, 2019, 10007))
}

pub fn part_two(input: &str) -> Option<u64> {
    //Some(solve_part_one(input, 2020, 119315717514047))
    None
}

fn solve_part_one(input: &str, pos: u64, size: u64) -> u64 {
    // Too slow for part 2 iterations...
    let operations = parse_operations(input);

    let mut pos_i = pos;

    for op in &operations {
        match op {
            Operatioin::DeaIntoNewStack => pos_i = deal(pos_i, size),
            Operatioin::Cut(n) => pos_i = cut(pos_i, *n, size),
            Operatioin::Increment(n) => pos_i = deal_with_increment(pos_i, *n, size),
        }
    }

    pos_i
}

enum Operatioin {
    DeaIntoNewStack,
    Cut(i64),
    Increment(u64),
}

fn parse_operations(input: &str) -> Vec<Operatioin> {
    let mut result = Vec::new();
    let raw_commands = input.lines();

    for c in raw_commands {
        if c == "deal into new stack" {
            result.push(Operatioin::DeaIntoNewStack);
        } else if let Some(n) = c.strip_prefix("cut ") {
            result.push(Operatioin::Cut(
                n.parse().expect("Expected u64 for an operation!"),
            ));
        } else if let Some(n) = c.strip_prefix("deal with increment ") {
            result.push(Operatioin::Increment(
                n.parse().expect("Expected u64 for an operation!"),
            ));
        }
    }

    result
}

fn deal(pos: u64, size: u64) -> u64 {
    // Which possition the requested element (as pos) is after the operation.
    size - 1 - pos
}

fn cut(pos: u64, n: i64, size: u64) -> u64 {
    // Which possition the requested element (as pos) is after the operation.
    (pos as i64 - n).rem_euclid(size as i64) as u64
}

fn deal_with_increment(pos: u64, n: u64, size: u64) -> u64 {
    // Tells the position of the element after the operation
    // n and size are co-primes
    // n = 3
    // 0 1 2 3 4 5 6 7 8 9 - init
    // 0 7 4 1 8 5 2 9 6 3 - result
    // 0 1 2 0 1 2 0 1 2 0 - mod init
    // 0 0 0 1 1 1 2 2 2 3 - rem init

    (pos * n) % size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse_deal() {
        let result_0 = deal(0, 10);
        assert_eq!(result_0, 9);

        let result_9 = deal(9, 10);
        assert_eq!(result_9, 0);

        let result_3 = deal(3, 10);
        assert_eq!(result_3, 6);
    }

    #[test]
    fn test_inverse_cut() {
        let result_0 = cut(0, 3, 10);
        assert_eq!(result_0, 7);

        let result_9 = cut(9, 3, 10);
        assert_eq!(result_9, 6);

        let result_3 = cut(3, 3, 10);
        assert_eq!(result_3, 0);
    }

    #[test]
    fn test_inverse_cut_negative() {
        let result_0 = cut(0, -4, 10);
        assert_eq!(result_0, 4);

        let result_9 = cut(9, -4, 10);
        assert_eq!(result_9, 3);

        let result_3 = cut(3, -4, 10);
        assert_eq!(result_3, 7);
    }

    #[test]
    fn test_inverse_deal_with_increment() {
        let result_0 = deal_with_increment(0, 3, 10);
        assert_eq!(result_0, 0);

        let result_9 = deal_with_increment(9, 3, 10);
        assert_eq!(result_9, 7);

        let result_4 = deal_with_increment(4, 3, 10);
        assert_eq!(result_4, 2);

        let result_3 = deal_with_increment(3, 3, 10);
        assert_eq!(result_3, 9);
    }

    #[test]
    fn test_part_one_1() {
        let input = &&advent_of_code::template::read_file_part("examples", DAY, 1);
        let result_3 = solve_part_one(input, 3, 10);
        assert_eq!(result_3, 1);

        let result_9 = solve_part_one(input, 9, 10);
        assert_eq!(result_9, 3);

        let result_7 = solve_part_one(input, 7, 10);
        assert_eq!(result_7, 9);
    }

    #[test]
    fn test_part_one_2() {
        let input = &&advent_of_code::template::read_file_part("examples", DAY, 2);
        let result_3 = solve_part_one(input, 3, 10);
        assert_eq!(result_3, 0);

        let result_9 = solve_part_one(input, 9, 10);
        assert_eq!(result_9, 8);

        let result_7 = solve_part_one(input, 7, 10);
        assert_eq!(result_7, 2);
    }

    #[test]
    fn test_part_one_3() {
        let input = &&advent_of_code::template::read_file_part("examples", DAY, 3);
        let result_3 = solve_part_one(input, 3, 10);
        assert_eq!(result_3, 1);

        let result_9 = solve_part_one(input, 9, 10);
        assert_eq!(result_9, 9);

        let result_7 = solve_part_one(input, 7, 10);
        assert_eq!(result_7, 3);
    }

    #[test]
    fn test_part_one_4() {
        let input = &&advent_of_code::template::read_file_part("examples", DAY, 4);
        let result_3 = solve_part_one(input, 3, 10);
        assert_eq!(result_3, 8);

        let result_9 = solve_part_one(input, 9, 10);
        assert_eq!(result_9, 0);

        let result_7 = solve_part_one(input, 7, 10);
        assert_eq!(result_7, 6);
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
