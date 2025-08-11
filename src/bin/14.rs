advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u64> {
    // main challenge is leftover management from multiple reactions path
    // if taken individually cost may be more
    // ex
    // 3 ORE -> 2 A
    // 3 A -> B
    // A -> C
    // A C => FUEL
    // need total 6 ore, individual path traversal gives 9
    // idea - individual path traversal + store leftovers for next traversals
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
