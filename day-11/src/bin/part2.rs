use std::collections::BTreeSet;

fn process(input: &str, expansion: usize) -> usize {
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

    let mut galaxies: BTreeSet<(isize, isize)> = BTreeSet::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
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
        .map(|(a, b)| {
            let mut distance = 0;
            for x in a.0.min(b.0)..a.0.max(b.0) {
                if !galaxy_columns[x as usize] {
                    distance += expansion;
                } else {
                    distance += 1;
                }
            }
            for y in a.1.min(b.1)..a.1.max(b.1) {
                if !galaxy_rows[y as usize] {
                    distance += expansion;
                } else {
                    distance += 1;
                }
            }
            distance
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input, 1000000));
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
        assert_eq!(process(TEST, 2), 374);
        assert_eq!(process(TEST, 10), 1030);
        assert_eq!(process(TEST, 100), 8410);
    }
}
