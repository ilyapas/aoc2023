use std::collections::{BTreeMap, BTreeSet};

fn check(input: &Vec<char>, test: &Vec<usize>) -> bool {
    let mut groups: Vec<usize> = vec![];
    let mut group: usize = 0;
    input.iter().enumerate().for_each(|(i, c)| {
        if *c == '#' {
            group += 1;
        } else if group > 0 {
            groups.push(group);
            group = 0;
        }
        if i == input.len() - 1 && group > 0 {
            groups.push(group);
        }
    });
    groups == *test
}

fn process(input: &str) -> usize {
    let mut data: Vec<Vec<char>> = Vec::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    input.lines().for_each(|line| {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        data.push(parts.first().unwrap().chars().collect::<Vec<_>>());
        groups.push(
            parts
                .last()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        );
    });

    let unknowns = data
        .iter()
        .map(|row| row.iter().filter(|c| **c == '?').count())
        .collect::<Vec<_>>();
    let max_unknowns = *unknowns.iter().max().unwrap();
    let mut matches: BTreeMap<usize, BTreeSet<Vec<char>>> = BTreeMap::new();

    for unknown in 0..2usize.pow(max_unknowns as u32) {
        for (i, row) in data.iter().enumerate() {
            if unknown > 2usize.pow(unknowns[i] as u32) {
                continue;
            }
            let mut test: Vec<char> = row.clone();
            let mut q = unknown;
            row.iter().enumerate().for_each(|(j, c)| {
                if *c == '?' {
                    let bit = q & 1;
                    q >>= 1;
                    test[j] = if bit == 1 { '#' } else { '.' };
                } else {
                    test[j] = *c;
                }
            });
            if check(&test, &groups[i]) {
                matches.entry(i).or_insert(BTreeSet::new()).insert(test);
            }
        }
    }

    matches.iter().map(|(_, v)| v.len()).sum()
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
.##.?#??.#.?# 2,1,1,1";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 22);
    }
}
