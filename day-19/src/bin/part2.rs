use std::collections::BTreeMap;

use itertools::Itertools;

fn count(
    ranges: &BTreeMap<char, (usize, usize)>,
    workflows: &BTreeMap<&str, Vec<&str>>,
    current: &str,
) -> usize {
    if current == "R" {
        return 0;
    }
    if current == "A" {
        return ranges
            .iter()
            .map(|(_, (min, max))| max - min + 1)
            .product::<usize>();
    }

    let mut result = 0;
    let mut ranges = ranges.clone();
    let value = workflows.get(current).unwrap();
    let conditions = value[0..value.len() - 1].iter().collect::<Vec<_>>();
    let fallback = value[value.len() - 1];
    let mut break_loop = false;

    for condition in conditions.iter() {
        let (test, destination) = condition.split(":").collect_tuple().unwrap();
        let field = test.chars().nth(0).unwrap();
        let operator = test.chars().nth(1).unwrap();
        let test_value = test
            .chars()
            .skip(2)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let range = ranges.get(&field).unwrap();
        let (true_segment, false_segment) = match operator {
            '<' => {
                let true_segment = (range.0, test_value - 1);
                let false_segment = (test_value, range.1);
                (true_segment, false_segment)
            }
            '>' => {
                let true_segment = (test_value + 1, range.1);
                let false_segment = (range.0, test_value);
                (true_segment, false_segment)
            }
            _ => panic!("Unknown operator: {}", operator),
        };
        if true_segment.0 <= true_segment.1 {
            let mut copy = ranges.clone();
            copy.insert(field, true_segment);
            result += count(&mut copy, workflows, destination);
        }
        if false_segment.0 <= false_segment.1 {
            ranges.insert(field, false_segment);
        } else {
            break_loop = true;
            break;
        }
    }

    if !break_loop {
        result += count(&ranges, workflows, fallback);
    }

    result
}

fn process(input: &str) -> usize {
    let (wf, _) = input.split("\n\n").take(2).collect_tuple().unwrap();
    let mut workflows: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    wf.lines().for_each(|line| {
        let (key, value) = line.split("{").collect_tuple().unwrap();
        let conditions = value[..value.len() - 1].split(",").collect::<Vec<_>>();
        workflows.insert(key, conditions);
    });

    let mut ranges = BTreeMap::new();
    ranges.insert('x', (1, 4000));
    ranges.insert('m', (1, 4000));
    ranges.insert('a', (1, 4000));
    ranges.insert('s', (1, 4000));
    count(&mut ranges, &workflows, "in")
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 167409079868000);
    }
}
