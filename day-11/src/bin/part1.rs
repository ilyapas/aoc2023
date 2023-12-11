use std::collections::BTreeSet;

fn process(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut galaxy_rows: Vec<bool> = vec![false; height];
    let mut galaxy_columns: Vec<bool> = vec![false; width];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                galaxy_rows[y] = true;
                galaxy_columns[x] = true;
            }
        });
    });

    let mut new_input: Vec<Vec<char>> = vec![];
    input.lines().enumerate().for_each(|(y, line)| {
        let mut new_line: Vec<char> = vec![];
        line.chars().enumerate().for_each(|(x, c)| {
            if !galaxy_columns[x] {
                new_line.push(c);
            }
            new_line.push(c);
        });
        if !galaxy_rows[y] {
            new_input.push(new_line.clone());
        }
        new_input.push(new_line);
    });

    let mut galaxies: BTreeSet<(isize, isize)> = BTreeSet::new();
    new_input.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, c)| {
            if *c == '#' {
                galaxies.insert((x as isize, y as isize));
            }
        });
    });

    let mut pairs: BTreeSet<((isize, isize), (isize, isize))> = BTreeSet::new();
    for galaxy in galaxies.iter() {
        for other_galaxy in galaxies.iter() {
            if galaxy != other_galaxy {
                if pairs.contains(&(*other_galaxy, *galaxy)) {
                    continue;
                }
                pairs.insert((*galaxy, *other_galaxy));
            }
        }
    }

    pairs
        .iter()
        .map(|(a, b)| (a.0 - b.0).abs() + (a.1 - b.1).abs())
        .sum::<isize>() as usize
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 374);
    }
}
