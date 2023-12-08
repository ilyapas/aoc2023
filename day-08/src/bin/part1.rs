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
    let mut next_node = "AAA".to_string();
    let mut num_steps = 0;
    while !queue.is_empty() {
        let step = queue.pop_front().unwrap();
        let options = paths.get(&next_node).unwrap().clone();
        next_node = match step {
            'L' => options[0].clone(),
            'R' => options[1].clone(),
            _ => panic!("Unknown step: {}", step),
        };
        num_steps += 1;
        if next_node == "ZZZ" {
            break;
        }
        if queue.is_empty() {
            queue = steps.iter().cloned().collect();
        }
    }
    num_steps
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 6);
    }
}
