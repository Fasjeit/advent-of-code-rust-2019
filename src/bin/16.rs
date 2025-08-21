use itertools::Itertools;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u64> {
    let res = fft(input, "0, 1, 0, -1", 100);
    let digit_res: u64 = res[0..8].parse().unwrap();
    Some(digit_res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let index: usize = input[0..7].parse().unwrap();

    // see hacky_fft 2.
    let input = &input.repeat(10_000)[index..];
    let res = hacky_fft(input, 100);
    let digit_res: u64 = res[0..8].parse().unwrap();
    Some(digit_res)
}

pub fn fft(input: &str, pattern: &str, phase_count: u8) -> String {
    let pattern_digits: Vec<i32> = pattern.split(", ").map(|i| i.parse().unwrap()).collect();
    let input_digits: Vec<i32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let mut output = input_digits.clone();
    for _phase in 0..phase_count {
        let input = output;
        output = Vec::new();

        for i in 0..input.len() {
            let mut item = 0;

            // see 2. in hacky fft
            for k in i..input.len() {
                let pattern_value = pattern_digits[((k + 1) / (i + 1)) % pattern_digits.len()];
                item += input[k] * pattern_value;
                // print!(
                //     "{:1.1} * {:2.1} ",
                //     input[k],
                //     pattern_value
                // );
            }
            output.push(item.abs() % 10);
            //println!("{}", item.abs() as u32 % 10);
            //println!("{}", i);
        }

        //println!("{}", _phase);
    }

    Itertools::join(&mut output.iter(), "")

    //dbg!(&output);
}

pub fn hacky_fft(input: &str, phase_count: u8) -> String {
    // general idea from https://dhconnelly.com/advent-of-code-2019-commentary.html#day-16

    // 1. First digits becomes zero and stays zero (triangular matrix)
    // 2. When computing position N we only use next digits (as previous ones are zero, see site examples,
    // as first 0 in pattern always affects I multiplication on row I).
    // 3. The rows in the lower half of the matrix were all ones, regardless of what size matrix
    // 3.1. This means that we can forget about the coefficients entirely and just sum up the
    // vector elements -- and if we do it starting from the last element, which is just itself,
    // we don't even need to start the sum over at each previous element, since the sum for
    // element a[n-k] is just sum(a[n-k+1], ... a[n]).
    //
    // only works with fixed pattern, and large offsets

    let input_digits: Vec<i32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let mut output = input_digits.clone();
    for _phase in 0..phase_count {
        let mut sum = 0;
        for i in (0..output.len()).rev() {
            sum += output[i];
            output[i] = sum.abs() % 10;
        }
    }

    //println!("{}", _phase);

    Itertools::join(&mut output.iter(), "")

    //dbg!(&output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = fft(
            &advent_of_code::template::read_file_part("examples", DAY, 1),
            "0, 1, 0, -1",
            1,
        );
        assert_eq!(result, "48226158");
    }

    #[test]
    fn test_part_one_2() {
        let result = fft(
            &advent_of_code::template::read_file_part("examples", DAY, 1),
            "0, 1, 0, -1",
            2,
        );
        assert_eq!(result, "34040438");
    }

    #[test]
    fn test_part_one_3() {
        let result = fft(
            &advent_of_code::template::read_file_part("examples", DAY, 1),
            "0, 1, 0, -1",
            3,
        );
        assert_eq!(result, "03415518");
    }

    #[test]
    fn test_part_one_4() {
        let result = fft(
            &advent_of_code::template::read_file_part("examples", DAY, 1),
            "0, 1, 0, -1",
            4,
        );
        assert_eq!(result, "01029498");
    }

    #[test]
    fn test_part_one_5() {
        let result = fft(
            &advent_of_code::template::read_file_part("examples", DAY, 2),
            "0, 1, 0, -1",
            100,
        );
        assert!(result.starts_with("24176176"));
    }

    #[test]
    fn test_part_one_6() {
        let result = fft(
            &advent_of_code::template::read_file_part("examples", DAY, 3),
            "0, 1, 0, -1",
            100,
        );
        assert!(result.starts_with("73745418"));
    }

    #[test]
    fn test_part_one_7() {
        let result = fft(
            &advent_of_code::template::read_file_part("examples", DAY, 4),
            "0, 1, 0, -1",
            100,
        );
        assert!(result.starts_with("52432133"));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(84462026));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(78725270));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 7,
        ));
        assert_eq!(result, Some(53553731));
    }
}
