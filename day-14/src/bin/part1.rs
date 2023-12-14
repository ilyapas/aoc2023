use std::collections::BTreeMap;

fn process(input: &str) -> isize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut grid: BTreeMap<(isize, isize), char> = BTreeMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            grid.insert((x as isize, y as isize), c);
        });
    });

    for x in 0..width as isize {
        for y in 0..height as isize {
            if grid.get(&(x, y)).unwrap() == &'O' {
                continue;
            }
            if grid.get(&(x, y)).unwrap() == &'#' {
                continue;
            }
            if grid.get(&(x, y)).unwrap() == &'.' {
                for yy in y..height as isize {
                    if grid.get(&(x, yy)).unwrap() == &'.' {
                        continue;
                    }
                    if grid.get(&(x, yy)).unwrap() == &'#' {
                        break;
                    }
                    if grid.get(&(x, yy)).unwrap() == &'O' {
                        grid.insert((x, y), 'O');
                        grid.insert((x, yy), '.');
                        break;
                    }
                }
            }
        }
    }

    grid.iter()
        .filter(|(_, v)| **v == 'O')
        .map(|(k, _)| height as isize - k.1)
        .sum()
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 136);
    }
}
