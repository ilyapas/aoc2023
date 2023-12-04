use std::collections::{BTreeSet, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0, space0, space1},
    multi::separated_list1,
    IResult,
};

fn card(input: &str) -> IResult<&str, (BTreeSet<usize>, BTreeSet<usize>)> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space0(input)?;
    let (input, w) = separated_list1(space1, digit1)(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = space0(input)?;
    let (input, n) = separated_list1(space1, digit1)(input)?;
    let (input, _) = space0(input)?;

    let winners = w
        .into_iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<BTreeSet<_>>();

    let numbers = n
        .into_iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<BTreeSet<_>>();

    Ok((input, (winners, numbers)))
}

fn process(input: &str) -> usize {
    let (_, cards) = separated_list1(line_ending, card)(input).unwrap();
    let matches = cards
        .iter()
        .map(|(w, n)| w.intersection(n).count())
        .collect::<Vec<_>>();
    let mut total_cards = cards.len();
    let mut all_cards: VecDeque<usize> = (0..cards.len()).collect::<VecDeque<_>>();
    while all_cards.len() > 0 {
        let card = all_cards.pop_front();
        let match_count = matches[card.unwrap()];
        for i in 0..match_count {
            all_cards.push_back(card.unwrap() + i + 1);
            total_cards += 1;
        }
    }
    total_cards
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
        assert_eq!(process(TEST), 30);
    }
}
