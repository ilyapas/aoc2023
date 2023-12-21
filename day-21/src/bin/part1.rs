use std::collections::{BTreeMap, BTreeSet};

fn process(input: &str, steps: usize) -> usize {
    let mut grid: BTreeMap<(isize, isize), char> = BTreeMap::new();
    let mut start: (isize, isize) = (0, 0);
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            grid.insert((x as isize, y as isize), c);
            if c == 'S' {
                start = (x as isize, y as isize);
                grid.insert((x as isize, y as isize), '.');
            }
        });
    });

    let mut plots: BTreeSet<(isize, isize)> = BTreeSet::new();
    plots.insert(start);
    for _ in 0..steps {
        let mut next_plots: BTreeSet<(isize, isize)> = BTreeSet::new();
        for (x, y) in plots.iter() {
            let neighbors = vec![(*x, *y - 1), (*x, *y + 1), (*x - 1, *y), (*x + 1, *y)];
            for neighbor in neighbors {
                if grid.contains_key(&neighbor) {
                    if grid.get(&neighbor).unwrap() == &'.' {
                        next_plots.insert(neighbor);
                    }
                }
            }
        }
        plots = next_plots;
    }

    plots.len()
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input, 64));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST, 6), 16);
    }
}
