use itertools::Itertools;
use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq, Ord)]
struct Hand {
    cards: Vec<usize>,
    bid: usize,
}

impl Hand {
    fn rank(&self) -> usize {
        let mut card_map: BTreeMap<usize, usize> = BTreeMap::new();
        for card in self.cards.iter() {
            *card_map.entry(*card).or_insert(0) += 1;
        }
        let mut counts = card_map.values().collect::<Vec<_>>();
        counts.sort();
        counts.reverse();
        if counts[0] == &5 {
            0
        } else if counts[0] == &4 {
            1
        } else if (counts[0] == &3) && (counts[1] == &2) {
            2
        } else if counts[0] == &3 {
            3
        } else if (counts[0] == &2) && (counts[1] == &2) {
            4
        } else if counts[0] == &2 {
            5
        } else {
            6
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut result = other.rank().partial_cmp(&self.rank());
        if result == Some(std::cmp::Ordering::Equal) {
            result = self.cards.partial_cmp(&other.cards);
        }
        result
    }
}

fn process(input: &str) -> usize {
    let hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let line = line.split_whitespace().collect::<Vec<_>>();
            let cards = line[0]
                .chars()
                .map(|c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => c.to_digit(10).unwrap() as usize,
                })
                .collect::<Vec<_>>();
            let bid = line[1].parse::<usize>().unwrap();
            Hand { cards, bid }
        })
        .collect();

    hands
        .into_iter()
        .sorted()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 6440);
    }
}
