use std::collections::BTreeSet;

fn process(input: &str) -> usize {
    let blocks = input.split("\n\n").collect::<Vec<&str>>();
    blocks
        .iter()
        .map(|b| {
            let r = result(b);
            for i in 0..b.chars().count() {
                let mut bb = b.to_string();
                bb = bb
                    .chars()
                    .enumerate()
                    .map(|(j, cc)| {
                        if j == i {
                            if cc == '#' {
                                '.'
                            } else if cc == '.' {
                                '#'
                            } else {
                                cc
                            }
                        } else {
                            cc
                        }
                    })
                    .collect();
                let rr = result(&bb);
                let diff = rr.difference(&r).collect::<Vec<_>>();
                if diff.len() == 1 {
                    return *diff[0];
                }
            }
            panic!("No result found!");
        })
        .sum()
}

fn result(b: &str) -> BTreeSet<usize> {
    let mut result = BTreeSet::new();
    let r = mirror(b);
    r.iter().for_each(|i| {
        result.insert(*i * 100);
    });

    let bb = rotate(b);
    let r = mirror(&bb);
    r.iter().for_each(|i| {
        result.insert(*i);
    });
    result
}

fn rotate(b: &str) -> String {
    let w = b.lines().next().unwrap().len();
    let h = b.lines().count();
    let mut result = String::new();
    for x in 0..w {
        for y in (0..h).rev() {
            result.push(b.lines().nth(y).unwrap().chars().nth(x).unwrap());
        }
        result.push('\n');
    }
    result
}

fn mirror(b: &str) -> Vec<usize> {
    let size = b.lines().count();
    let mut results = vec![];
    for i in 0..size {
        for j in 1..size - i {
            if b.lines().nth(i + j).unwrap() == b.lines().nth(1 + i - j).unwrap() {
                if i + j == size - 1 || 1 + i - j == 0 {
                    results.push(i + 1);
                    break;
                }
            } else {
                break;
            }
        }
    }
    results
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 400);
    }
}
