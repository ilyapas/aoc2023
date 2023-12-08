use std::collections::{BTreeMap, VecDeque};

fn process(input: &str) -> usize {
    let blocks = input.split("\n\n").collect::<Vec<_>>();
    let steps = blocks[0].chars().collect::<Vec<_>>();
    let mut paths: BTreeMap<String, Vec<String>> = BTreeMap::new();
    blocks[1].lines().for_each(|line| {
        let line = line.replace(" = (", " ").replace(")", "").replace(",", " ");
        let elements = line.split_whitespace().collect::<Vec<_>>();
        paths.insert(
            elements[0].to_string(),
            elements[1..]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        );
    });

    let mut queue: VecDeque<char> = steps.iter().cloned().collect();
    let mut next_nodes = paths
        .keys()
        .cloned()
        .filter(|s| s.ends_with("A"))
        .collect::<Vec<_>>();
    let mut loop_lengths: BTreeMap<usize, usize> = BTreeMap::new();
    let mut num_steps = 0;
    while !queue.is_empty() {
        let step = queue.pop_front().unwrap();
        next_nodes = next_nodes
            .iter()
            .enumerate()
            .map(|(i, next_node)| {
                let options = paths.get(next_node).unwrap();
                let next_node = match step {
                    'L' => options[0].clone(),
                    'R' => options[1].clone(),
                    _ => panic!("Unknown step: {}", step),
                };
                if next_node.ends_with("Z") {
                    loop_lengths.insert(i, num_steps + 1);
                }
                if queue.is_empty() {
                    steps.iter().for_each(|step| queue.push_back(*step));
                }
                next_node
            })
            .collect::<Vec<_>>();
        num_steps += 1;
        if loop_lengths.len() == next_nodes.len() {
            break;
        }
    }
    let mut result = 1;
    loop_lengths
        .values()
        .for_each(|v| result = num::integer::lcm(result, *v));
    result
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 6);
    }
}
