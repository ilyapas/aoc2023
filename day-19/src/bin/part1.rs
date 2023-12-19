use std::collections::BTreeMap;

use itertools::Itertools;

fn process(input: &str) -> usize {
    let (wf, p) = input.split("\n\n").take(2).collect_tuple().unwrap();
    let mut workflows: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    wf.lines().for_each(|line| {
        let (key, value) = line.split("{").collect_tuple().unwrap();
        let conditions = value[..value.len() - 1].split(",").collect::<Vec<_>>();
        workflows.insert(key, conditions);
    });

    let parts = p
        .lines()
        .map(|line| {
            line[1..line.len() - 1]
                .split(",")
                .map(|s| {
                    let (key, value) = s.split("=").collect_tuple().unwrap();
                    (
                        key.chars().next().unwrap().to_string(),
                        value.parse::<usize>().unwrap(),
                    )
                })
                .collect::<BTreeMap<_, _>>()
        })
        .collect::<Vec<_>>();

    let mut accepted = vec![];
    for part in parts {
        let mut current = "in";
        while current != "R" && current != "A" {
            for condition in workflows.get(current).unwrap() {
                if condition.contains(":") {
                    let (test, destination) = condition.split(":").collect_tuple().unwrap();
                    let field = test.chars().nth(0).unwrap();
                    let operator = test.chars().nth(1).unwrap();
                    let test_value = test
                        .chars()
                        .skip(2)
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                    let new = match operator {
                        '<' => {
                            if part.get(&field.to_string()).unwrap() < &test_value {
                                destination
                            } else {
                                current
                            }
                        }
                        '>' => {
                            if part.get(&field.to_string()).unwrap() > &test_value {
                                destination
                            } else {
                                current
                            }
                        }
                        _ => panic!("Unknown operator: {}", operator),
                    };
                    if new != current {
                        current = new;
                        break;
                    }
                } else {
                    current = condition;
                    break;
                }
            }
        }
        if current == "A" {
            accepted.push(part);
        }
    }

    accepted.iter().map(|p| p.values().sum::<usize>()).sum()
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
        assert_eq!(process(TEST), 19114);
    }
}
