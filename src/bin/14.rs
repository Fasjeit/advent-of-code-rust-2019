use std::collections::HashMap;

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

    let mut conventions = HashMap::new();

    for line in input.lines() {
        let mut splitted = line.split(" => ");
        let inputs_str = splitted.next();
        let individual_inputs_str = inputs_str.unwrap().split(", ");

        let mut inputs = Vec::new();
        for input_str in individual_inputs_str {
            let input = ItemCount::from(input_str);
            inputs.push(input);
        }

        let output_str = splitted.next().unwrap();
        let output = ItemCount::from(output_str);

        // output - (List<input-count>, output_count)
        conventions.insert(output.name, (inputs, output.count));
    }

    let mut store = HashMap::new();
    let res = create_requested("FUEL", 1, &conventions, &mut store);
    Some(res)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

fn create_requested(
    requested_resource_name: &str,
    requested_resource_count: u64,
    conventions: &HashMap<String, (Vec<ItemCount>, u64)>,
    store: &mut HashMap<String, u64>,
) -> u64 {
    //dbg!(requested_resource_count);
    //dbg!(requested_resource_name);

    if requested_resource_name == "ORE" {
        //println!("Requested ore");
        return requested_resource_count;
    }

    // check if store have it
    // ? have resource_count in store
    // - subtract from store
    // - return
    // find conversion for resource
    // run conversion 'n = (resource_count - stored) / conversion_output' times
    // store resource_count - n leftovers in store
    // return

    let mut total_result = 0;

    let stored = *store
        .entry(requested_resource_name.to_string())
        .or_default();
    if stored >= requested_resource_count {
        store
            .entry(requested_resource_name.to_string())
            .and_modify(|r| *r -= requested_resource_count);
        // println!(
        //     "return with stored [{}] [{}]",
        //     requested_resource_name, requested_resource_count
        // );
        return 0;
    } else {
        // Will use all leftovers now.
        store.insert(requested_resource_name.to_string(), 0);
    }

    let (required_conversion, conversion_output) =
        conventions.get(requested_resource_name).unwrap();
    let required_new_resources = requested_resource_count - stored;
    let required_iterations = required_new_resources.div_ceil(*conversion_output);

    for conversion in required_conversion {
        // println!(
        //     "requesting more [{}] - [{}]",
        //     &conversion.name,
        //     conversion.count * required_iterations
        // );

        total_result += create_requested(
            &conversion.name,
            conversion.count * required_iterations,
            conventions,
            store,
        );
    }

    // store leftovers
    //dbg!(&conversion_output);
    //dbg!(&required_iterations);
    //dbg!(&requested_resource_count);
    let leftovers = conversion_output * required_iterations - required_new_resources;
    // println!(
    //     "storing leftovers [{}] - [{}]",
    //     requested_resource_name, leftovers
    // );
    store
        .entry(requested_resource_name.to_string())
        .and_modify(|r| *r += leftovers)
        .or_insert(leftovers);

    total_result
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct ItemCount {
    name: String,
    count: u64,
}

impl From<&str> for ItemCount {
    fn from(value: &str) -> Self {
        let mut splitted = value.split(" ");
        let count = splitted
            .next()
            .expect("Expected uint as first item")
            .parse()
            .expect("Expected uint as first item");
        let name = splitted.next().expect("Expected name as second item");

        ItemCount {
            name: name.to_string(),
            count,
        }
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(20));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_one_4() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(165));
    }

    #[test]
    fn test_part_one_5() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(13312));
    }

    #[test]
    fn test_part_one_6() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(180697));
    }

    #[test]
    fn test_part_one_7() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 7,
        ));
        assert_eq!(result, Some(2210736));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
