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
                            break;
                        }
                    }
                }
            }
        }
    });

    let mut part_numbers: Vec<String> = Vec::new();

    (0..rows).for_each(|r| {
        let mut part_number: Vec<char> = Vec::new();
        let mut valid = false;
        (0..cols).for_each(|c| {
            let char = grid.get(&(r, c)).unwrap();
            if char.is_digit(10) {
                part_number.push(*char);
                if digits_with_symbols.contains(&(r, c)) {
                    valid = true;
                }
            } else {
                if !part_number.is_empty() && valid {
                    part_numbers.push(part_number.iter().collect::<String>());
                }
                part_number.clear();
                valid = false;
            }
        });
        if !part_number.is_empty() && valid {
            part_numbers.push(part_number.iter().collect::<String>());
        }
    });

    part_numbers
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
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
        assert_eq!(process(TEST), 4361);
    }
}
