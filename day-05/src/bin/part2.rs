use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, not_line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
};

fn seed_map(input: &str) -> nom::IResult<&str, Vec<Vec<usize>>> {
    preceded(
        tuple((not_line_ending, line_ending)),
        separated_list1(
            line_ending,
            separated_list1(space1, map_res(digit1, |s: &str| s.parse::<usize>())),
        ),
    )(input)
}

fn seed_ranges(input: &str) -> nom::IResult<&str, Vec<Range<usize>>> {
    let (input, seeds_ranges) = preceded(
        tag("seeds: "),
        separated_list1(space1, separated_pair(digit1, space1, digit1)),
    )(input)?;
    let seeds_ranges = seeds_ranges
        .iter()
        .map(|(start, length)| {
            let start = start.to_string().parse::<usize>().unwrap();
            let length = length.to_string().parse::<usize>().unwrap();
            start..start + length
        })
        .collect::<Vec<_>>();
    let (input, _) = tag("\n\n")(input)?;
    Ok((input, seeds_ranges))
}

fn process(input: &str) -> usize {
    let (input, seed_ranges) = seed_ranges(input).unwrap();
    let (_, seed_maps) = separated_list1(tag("\n\n"), seed_map)(input).unwrap();

    let result = seed_ranges
        .iter()
        .map(|seed_range| {
            dbg!(seed_range);
            seed_range
                .clone()
                .map(|seed| {
                    let mut next_value = seed;
                    seed_maps.iter().for_each(|seed_map| {
                        let mut value = next_value;
                        for i in 0..seed_map.len() {
                            let row = &seed_map[i];
                            let range = row[1]..row[1] + row[2];
                            if range.contains(&next_value) {
                                value = row[0] + (next_value - row[1]);
                                break;
                            }
                        }
                        next_value = value;
                    });
                    next_value
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    result
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 46);
    }
}
