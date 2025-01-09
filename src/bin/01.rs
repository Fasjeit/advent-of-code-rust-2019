advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let mut masses: Vec<u64> = Vec::new();

    lines.for_each(|l| masses.push(l.parse().expect("Expected u64")));

    let result = masses.iter().fold(0, |acc, m| acc + compute_fuel(m));

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();
    let mut masses: Vec<u64> = Vec::new();

    lines.for_each(|l| masses.push(l.parse().expect("Expected u64")));

    let result = masses
        .iter()
        .fold(0, |acc, m| acc + compute_fuel_iterative(m));

    Some(result)
}

fn compute_fuel(mass: &u64) -> u64 {
    if mass / 3 > 2 {
        (mass / 3) - 2
    } else {
        0
    }
}

fn compute_fuel_iterative(mass: &u64) -> u64 {
    let mut total = compute_fuel(mass);
    let mut additional = compute_fuel(&total);
    while additional != 0 {
        total += additional;
        additional = compute_fuel(&additional);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34_241));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51_316));
    }
}
