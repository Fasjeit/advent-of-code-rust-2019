advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve_part_one(input, 2019, 10007))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve_part_two(
        input,
        2020,
        119315717514047,
        101741582076661,
        true,
    ))
}

fn solve_part_one(input: &str, pos: u64, size: u64) -> u64 {
    // // Too slow for part 2 iterations...
    // let operations = parse_operations(input);

    // let mut pos_i = pos;

    // for op in &operations {
    //     match op {
    //         Operatioin::Deal => pos_i = deal(pos_i, size),
    //         Operatioin::Cut(n) => pos_i = cut(pos_i, *n, size),
    //         Operatioin::Increment(n) => pos_i = deal_with_increment(pos_i, *n, size),
    //     }
    // }

    // pos_i

    solve_part_two(input, pos, size, 1, false)
}

fn solve_part_two(input: &str, pos: u64, size: u64, iterations: u64, need_invert: bool) -> u64 {
    // All oerations are linear in a form of
    // (a*pos + b) mod size
    // deal: f(pos) = size - 1 - f(pos)
    //       f(pos) = (-1)(a * pos) + (size - 1 - b)
    // cut:  f(pos) = f(pos) - n
    //       f(pos) = a*pos + (b-n)
    // inc:  f(pos) = f(pos) * n
    //       f(pos) = a * n * pos + b * n
    //
    // We can go thougth all operations, maintaining a and b values to construct f(x)
    let operations = parse_operations(input);
    let mut a: i128 = 1;
    let mut b: i128 = 0;

    for op in &operations {
        (a, b) = match op {
            Operatioin::Deal => (
                (-a).rem_euclid(size as i128),
                (size as i128 - 1 - b).rem_euclid(size as i128),
            ),
            Operatioin::Cut(n) => (a, (b - n).rem_euclid(size as i128)),
            Operatioin::Increment(n) => (
                (a * (*n as i128)).rem_euclid(size as i128),
                (b * (*n as i128)).rem_euclid(size as i128),
            ),
        }
    }

    // looks like we need fast exp here...
    (a, b) = fast_power(a, b, iterations, size);

    if need_invert {
        // Now need to invert function
        (a, b) = invert(a, b, size).unwrap();
    }

    // compute the result
    ((a * pos as i128 + b).rem_euclid(size as i128)) as u64
}

fn invert(a: i128, b: i128, size: u64) -> Option<(i128, i128)> {
    // GPT assist
    let n = size as i128;
    let inv_a = mod_inverse(a, n)?;

    let inv_b = (-((inv_a * b) % n)).rem_euclid(n);

    Some((inv_a, inv_b))
}

fn mod_inverse(a: i128, n: i128) -> Option<i128> {
    // Euclid alg for finding modulo inverse
    // GPT assist
    let (mut old_r, mut r) = (a.rem_euclid(n), n);
    let (mut old_s, mut s) = (1, 0);

    while r != 0 {
        let q = old_r / r;

        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }

    if old_r != 1 {
        return None;
    }

    Some(old_s.rem_euclid(n))
}

fn fast_power(a: i128, b: i128, iterations: u64, size: u64) -> (i128, i128) {
    // Identity function: f(x) = x
    let (mut result_a, mut result_b) = (1, 0);

    let mut exp = iterations;
    let (mut expo_f_a, mut expo_f_b) = (a, b);

    while exp > 0 {
        // Test bit
        // if have one -
        if exp & 1 == 1 {
            (result_a, result_b) = function_compose(result_a, result_b, expo_f_a, expo_f_b, size);
        }

        // f = f(f)
        (expo_f_a, expo_f_b) = function_compose(expo_f_a, expo_f_b, expo_f_a, expo_f_b, size);

        exp >>= 1;
    }

    (result_a, result_b)
}

fn function_compose(a: i128, b: i128, c: i128, d: i128, size: u64) -> (i128, i128) {
    // f(g(pow)) = a*(c*pow + d) + b
    // a*c * pow + a*d + b

    // compute b first, as it depends on old a value.
    let b_new = (a * d + b).rem_euclid(size as i128);
    let a_new = (a * c).rem_euclid(size as i128);

    (a_new, b_new)
}

enum Operatioin {
    Deal,
    Cut(i128),
    Increment(u64),
}

fn parse_operations(input: &str) -> Vec<Operatioin> {
    let mut result = Vec::new();
    let raw_commands = input.lines();

    for c in raw_commands {
        if c == "deal into new stack" {
            result.push(Operatioin::Deal);
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

// for slow part_1

#[allow(dead_code)]
fn deal(pos: u64, size: u64) -> u64 {
    // Which possition the requested element (as pos) is after the operation.
    size - 1 - pos
}

#[allow(dead_code)]
fn cut(pos: u64, n: i128, size: u64) -> u64 {
    // Which possition the requested element (as pos) is after the operation.
    (pos as i128 - n).rem_euclid(size as i128) as u64
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn slow_power(a: i128, b: i128, iterations: u64, size: u64) -> (i128, i128) {
    // Test-only
    let mut result = (1, 0);

    for _ in 0..iterations {
        result = function_compose(a, b, result.0, result.1, size);
    }

    result
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

    #[test]
    fn test_fast_power() {
        assert_eq!(fast_power(3, 7, 13, 10), slow_power(3, 7, 13, 10));
    }

    #[test]
    fn test_inverse() {
        let f = (3, 7);
        let inv = invert(f.0, f.1, 10).unwrap();

        assert_eq!(function_compose(f.0, f.1, inv.0, inv.1, 10), (1, 0));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
