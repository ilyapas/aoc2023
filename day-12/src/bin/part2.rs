use itertools::Itertools;
use std::{
    collections::BTreeMap,
    iter::{repeat, zip},
};

fn arrangements(
    memo: &mut BTreeMap<(usize, usize, usize), usize>,
    data: &Vec<char>,
    groups: &Vec<usize>,
    data_idx: usize,
    groups_idx: usize,
    current: usize,
) -> usize {
    let key = (data_idx, groups_idx, current);
    if let Some(&value) = memo.get(&key) {
        return value;
    }
    if data_idx == data.len() {
        if groups_idx == groups.len() && current == 0 {
            return 1;
        } else if groups_idx == groups.len() - 1 && current == groups[groups_idx] {
            return 1;
        } else {
            return 0;
        }
    }
    let mut result = 0;
    let c = data[data_idx];
    if c == '#' || c == '?' {
        result += arrangements(memo, data, groups, data_idx + 1, groups_idx, current + 1);
    }
    if c == '.' || c == '?' {
        if current == 0 {
            result += arrangements(memo, data, groups, data_idx + 1, groups_idx, 0);
        } else if groups_idx < groups.len() && current == groups[groups_idx] {
            result += arrangements(memo, data, groups, data_idx + 1, groups_idx + 1, 0);
        }
    }

    memo.insert(key, result);
    result
}

fn process(input: &str) -> usize {
    let mut data: Vec<Vec<char>> = Vec::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    input.lines().for_each(|line| {
        let (d, g) = line.split_whitespace().collect_tuple().unwrap();
        let dd = repeat(d).take(5).join("?");
        let gg = repeat(g).take(5).join(",");
        data.push(dd.chars().collect::<Vec<_>>());
        groups.push(
            gg.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        );
    });

    let mut memo: BTreeMap<(usize, usize, usize), usize> = BTreeMap::new();
    zip(data, groups)
        .map(|(data, groups)| {
            let result = arrangements(&mut memo, &data, &groups, 0, 0, 0);
            memo.clear();
            result
        })
        .sum()
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
?###???????? 3,2,1";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 525152);
    }
}
