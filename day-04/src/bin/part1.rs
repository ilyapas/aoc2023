use std::collections::BTreeSet;

fn process(input: &str) -> usize {
    let cards: Vec<(BTreeSet<usize>, BTreeSet<usize>)> = input
        .lines()
        .map(|line| {
            let s = line.split(&[':', '|']).collect::<Vec<&str>>();
            let w = s[1]
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<BTreeSet<_>>();
            let n = s[2]
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<BTreeSet<_>>();
            (w, n)
        })
        .collect();

    let base: usize = 2;
    cards
        .iter()
        .map(|(w, n)| {
            let i = w.intersection(n).count() as u32;
            if i > 0 {
                base.pow(i - 1)
            } else {
                0
            }
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

    const TEST: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 13);
    }
}
