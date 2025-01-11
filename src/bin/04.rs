advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let mut splitted = input.split("-");
    let from: u32 = splitted
        .next()
        .expect("Wrong input!")
        .parse()
        .expect("Expected u32");
    let to: u32 = splitted
        .next()
        .expect("Wrong input!")
        .parse()
        .expect("Expected u32");

    let mut res = 0;

    for i in from..=to {
        let mut digits = Vec::new();
        let mut n = i;
        while n != 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.reverse();

        if check_digits_part1(digits) {
            res += 1;
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut splitted = input.split("-");
    let from: u32 = splitted
        .next()
        .expect("Wrong input!")
        .parse()
        .expect("Expected u32");
    let to: u32 = splitted
        .next()
        .expect("Wrong input!")
        .parse()
        .expect("Expected u32");

    let mut res = 0;

    for i in from..=to {
        let mut digits = Vec::new();
        let mut n = i;
        while n != 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.reverse();

        if check_digits_part2(digits) {
            res += 1;
        }
    }
    Some(res)
}

#[allow(clippy::comparison_chain)]
fn check_digits_part1(digits: Vec<u32>) -> bool {
    let mut have_double = false;
    for d in 1..digits.len() {
        if digits[d] < digits[d - 1] {
            //println!("{digits:?} is decreasing at some point!");
            return false;
        } else if digits[d] == digits[d - 1] {
            have_double = true;
        }
    }
    // if !have_double {
    //     println!("{digits:?} no double!");
    // } else {
    //     println!("{digits:?} is ok!");
    // }
    have_double
}

#[allow(clippy::comparison_chain)]
fn check_digits_part2(digits: Vec<u32>) -> bool {
    let mut have_double = false;

    // we will simply remember last "group" digit
    // which will be enough as digits are only increasing
    let mut skip_group_digit = -1;
    for d in 1..digits.len() {
        if digits[d] < digits[d - 1] {
            //println!("{digits:?} is decreasing at some point!");
            return false;
        } else if digits[d] == digits[d - 1] {
            // have double
            if d != digits.len() - 1 && digits[d] == digits[d + 1] {
                // have at least triple
                skip_group_digit = digits[d] as i64;
            } else if digits[d] as i64 != skip_group_digit {
                have_double = true;
            }
        }
    }
    // if !have_double {
    //     println!("{digits:?} no double!");
    // } else {
    //     println!("{digits:?} is ok!");
    // }
    have_double
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(495));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(394));
    }

    #[test]
    fn test_part_two_single_number_1() {
        let result = check_digits_part2(vec![1, 1, 4, 4, 4, 5]);
        assert!(result);
    }

    #[test]
    fn test_part_two_single_number_2() {
        let result = check_digits_part2(vec![0, 1, 4, 4, 4, 5]);
        assert!(!result);
    }
}
