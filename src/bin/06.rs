use std::collections::HashMap;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();

    let mut orbits = Orbits {
        nodes: HashMap::new(),
    };

    for line in lines {
        let mut split = line.split(')');
        let first = split.next().expect("Expected name 1");
        let second = split.next().expect("Expected name 2");

        orbits.add_node(second.to_string(), first.to_string());
    }

    let result = orbits.calc_depth("COM", 0);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();

    let mut orbits = Orbits {
        nodes: HashMap::new(),
    };

    for line in lines {
        let mut split = line.split(')');
        let first = split.next().expect("Expected name 1");
        let second = split.next().expect("Expected name 2");

        orbits.add_node(second.to_string(), first.to_string());
    }

    orbits.calc_depth("COM", 0);

    let common_parent_depth = orbits.find_common_parent_depth("YOU", "SAN");
    let you_depth = orbits.depth_for_node("YOU");
    let san_depth = orbits.depth_for_node("SAN");
    let result = you_depth + san_depth - 2 * common_parent_depth;

    Some(result)
}

struct Node {
    children: Vec<String>,
    parent: Option<String>,
    depth: u64,
}

struct Orbits {
    nodes: HashMap<String, Node>,
}

impl Orbits {
    fn add_node(&mut self, name: String, parent: String) {
        // add node
        self.nodes
            .entry(name.clone())
            .and_modify(|n| n.parent = Some(parent.clone()))
            .or_insert(Node {
                parent: Some(parent.clone()),
                children: vec![],
                depth: 0,
            });

        // add parent
        self.nodes
            .entry(parent.clone())
            .and_modify(|n| n.children.push(name.clone()))
            .or_insert(Node {
                parent: None,
                children: vec![name],
                depth: 0,
            });
    }

    fn calc_depth(&mut self, node_name: &str, current_depth: u64) -> u64 {
        let mut result = 0;

        self.nodes
            .entry(node_name.to_string())
            .and_modify(|n| n.depth = current_depth);

        let node = &self.nodes.get(node_name).unwrap();
        for child in &node.children.clone() {
            result += self.calc_depth(child, current_depth + 1);
        }

        result + current_depth
    }

    fn depth_for_node(&self, node_name: &str) -> u64 {
        let node = &self.nodes.get(node_name).unwrap();
        node.depth
    }

    fn find_common_parent_depth(&self, src_node_name: &str, dst_node_name: &str) -> u64 {
        let mut visited = HashMap::<String, u64>::new();

        let mut first = src_node_name;
        let mut second = dst_node_name;

        //let mut stdin = io::stdin().lock();

        loop {
            // let mut buffer = "".to_string();
            // stdin.read_line(&mut buffer);

            //dbg!(&first);
            //dbg!(&second);

            // step first node
            let node = self.nodes.get(first).expect("Expected node");
            if let Some(parent) = &node.parent {
                if visited.contains_key(parent) {
                    //dbg!(&parent);
                    return visited[parent];
                } else {
                    let parent_node = self.nodes.get(first).expect("Expected Node");
                    visited
                        .entry(parent.to_string())
                        .or_insert(parent_node.depth);
                    first = parent;
                }
            } else {
                panic!("No parent node for node name [{first}] set")
            }

            // step second node
            let node = self.nodes.get(second).expect("Expected node");
            if let Some(parent) = &node.parent {
                if visited.contains_key(parent) {
                    //dbg!(&parent);
                    return visited[parent];
                } else {
                    let parent_node = self.nodes.get(second).expect("Expected Node");
                    visited
                        .entry(parent.to_string())
                        .or_insert(parent_node.depth);
                    second = parent;
                }
            } else {
                panic!("No parent node for node name [{first}] set")
            }
        }
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
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }
}
