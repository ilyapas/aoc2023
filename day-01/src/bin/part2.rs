use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alpha1, digit1, line_ending},
    combinator::map_parser,
    multi::{many1, separated_list1},
    IResult,
};

fn number(input: &str) -> IResult<&str, u32> {
    let (input, digits) = many1(alt((
        tag("one"),
        tag("two"),
        tag("three"),
        tag("four"),
        tag("five"),
        tag("six"),
        tag("seven"),
        tag("eight"),
        tag("nine"),
        map_parser(take(1u8), digit1),
        map_parser(take(1u8), alpha1),
    )))(input)?;

    let number = digits
        .iter()
        .map(|s| match *s {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => s.parse::<u32>().unwrap_or(0),
        })
        .filter(|n| *n != 0)
        .collect::<Vec<u32>>();
    let first = number.first().unwrap();
    let last = number.last().unwrap();
    Ok((input, first * 10 + last))
}

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, numbers) = separated_list1(line_ending, number)(input)?;
    Ok((input, numbers))
}

fn process(input: &str) -> u32 {
    let (_, numbers) = numbers(input).unwrap();
    numbers.iter().sum()
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("{:?}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 281);
    }
}
