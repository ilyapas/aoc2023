use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

enum Color {
    Red,
    Green,
    Blue,
}

fn color(input: &str) -> IResult<&str, (Color, u32)> {
    let (input, number) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = map_res(
        alt((tag("red"), tag("green"), tag("blue"))),
        |s: &str| match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        },
    )(input)?;
    Ok((input, (color, number)))
}

fn round(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, colors) = separated_list1(tag(", "), color)(input)?;
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    colors.iter().for_each(|(c, n)| match c {
        Color::Red => red += n,
        Color::Green => green += n,
        Color::Blue => blue += n,
    });
    Ok((input, (red, green, blue)))
}

fn game(input: &str) -> IResult<&str, Vec<(u32, u32, u32)>> {
    let (input, _) = tag("Game ")(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, rounds) = separated_list1(tag("; "), round)(input)?;
    Ok((input, rounds))
}

fn is_possible(game: &Vec<(u32, u32, u32)>, red: u32, green: u32, blue: u32) -> bool {
    for (r, g, b) in game {
        if r > &red || g > &green || b > &blue {
            return false;
        }
    }
    true
}

fn process(input: &str) -> u32 {
    let (_, games) = separated_list1(line_ending, game)(input).unwrap();
    games
        .iter()
        .enumerate()
        .filter(|(_i, g)| is_possible(g, 12u32, 13u32, 14u32))
        .fold(0, |acc, (i, _g)| acc + (i + 1) as u32)
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 8);
    }
}
