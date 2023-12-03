use std::collections::{BTreeMap, BTreeSet};

fn process(input: &str) -> usize {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(move |(r, line)| {
            line.chars()
                .enumerate()
                .map(move |(c, char)| ((r, c), char))
        })
        .collect::<BTreeMap<_, _>>();

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();

    let mut digits_with_symbols: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut digits_with_gears: BTreeMap<(usize, usize), (usize, usize)> = BTreeMap::new();
    let mut gears_with_digits: BTreeMap<(usize, usize), Vec<(usize, usize)>> = BTreeMap::new();

    grid.iter().for_each(|((r, c), char)| {
        if char != &'.' {
            if char.is_digit(10) {
                const DIRS: [(i32, i32); 8] = [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ];
                for (dr, dc) in DIRS.iter() {
                    let nr = *r as i32 + dr;
                    let nc = *c as i32 + dc;
                    if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                        let neighbor = grid.get(&(nr as usize, nc as usize)).unwrap();
                        if !neighbor.is_digit(10) && !neighbor.is_alphabetic() && neighbor != &'.' {
                            digits_with_symbols.insert((*r as usize, *c as usize));
                            if neighbor == &'*' {
                                digits_with_gears
                                    .insert((*r as usize, *c as usize), (nr as usize, nc as usize));
                                gears_with_digits
                                    .entry((nr as usize, nc as usize))
                                    .or_insert(Vec::new())
                                    .push((*r as usize, *c as usize));
                            }
                            break;
                        }
                    }
                }
            }
        }
    });

    let mut part_numbers: Vec<String> = Vec::new();
    let mut gears_with_numbers: BTreeMap<(usize, usize), Vec<String>> = BTreeMap::new();

    (0..rows).for_each(|r| {
        let mut part_number: Vec<char> = Vec::new();
        let mut valid = false;
        let mut connected_gears: BTreeSet<(usize, usize)> = BTreeSet::new();
        (0..cols).for_each(|c| {
            let char = grid.get(&(r, c)).unwrap();
            if char.is_digit(10) {
                part_number.push(*char);
                if digits_with_symbols.contains(&(r, c)) {
                    valid = true;
                }
                if digits_with_gears.contains_key(&(r, c)) {
                    connected_gears.insert(digits_with_gears.get(&(r, c)).unwrap().clone());
                }
            } else {
                if !part_number.is_empty() && valid {
                    part_numbers.push(part_number.iter().collect::<String>());
                    connected_gears.iter().for_each(|gear| {
                        gears_with_numbers
                            .entry(*gear)
                            .or_insert(Vec::new())
                            .push(part_number.iter().collect::<String>());
                    });
                }
                part_number.clear();
                valid = false;
                connected_gears.clear();
            }
        });
        if !part_number.is_empty() && valid {
            part_numbers.push(part_number.iter().collect::<String>());
            connected_gears.iter().for_each(|gear| {
                gears_with_numbers
                    .entry(*gear)
                    .or_insert(Vec::new())
                    .push(part_number.iter().collect::<String>());
            });
        }
    });

    gears_with_numbers
        .iter()
        .filter(|(_gear, numbers)| numbers.len() == 2)
        .map(|(_gear, numbers)| {
            numbers
                .iter()
                .map(|s| s.parse::<usize>().unwrap())
                .product::<usize>()
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

    const TEST: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 467835);
    }
}
