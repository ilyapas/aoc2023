use std::collections::{BTreeMap, BTreeSet};

fn tilt_north(grid: &mut BTreeMap<(isize, isize), char>, width: isize, height: isize) {
    for x in 0..width {
        for y in 0..height {
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
}

fn tilt_south(grid: &mut BTreeMap<(isize, isize), char>, width: isize, height: isize) {
    for x in 0..width {
        for yi in 0..height {
            let y = height - yi - 1;
            if grid.get(&(x, y)).unwrap() == &'O' {
                continue;
            }
            if grid.get(&(x, y)).unwrap() == &'#' {
                continue;
            }
            if grid.get(&(x, y)).unwrap() == &'.' {
                for yyi in yi..height as isize {
                    let yy = height - yyi - 1;
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
}

fn tilt_west(grid: &mut BTreeMap<(isize, isize), char>, width: isize, height: isize) {
    for y in 0..height {
        for x in 0..width {
            if grid.get(&(x, y)).unwrap() == &'O' {
                continue;
            }
            if grid.get(&(x, y)).unwrap() == &'#' {
                continue;
            }
            if grid.get(&(x, y)).unwrap() == &'.' {
                for xx in x..width as isize {
                    if grid.get(&(xx, y)).unwrap() == &'.' {
                        continue;
                    }
                    if grid.get(&(xx, y)).unwrap() == &'#' {
                        break;
                    }
                    if grid.get(&(xx, y)).unwrap() == &'O' {
                        grid.insert((x, y), 'O');
                        grid.insert((xx, y), '.');
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_east(grid: &mut BTreeMap<(isize, isize), char>, width: isize, height: isize) {
    for y in 0..height {
        for xi in 0..width {
            let x = width - xi - 1;
            if grid.get(&(x, y)).unwrap() == &'O' {
                continue;
            }
            if grid.get(&(x, y)).unwrap() == &'#' {
                continue;
            }
            if grid.get(&(x, y)).unwrap() == &'.' {
                for xxi in xi..width as isize {
                    let xx = width - xxi - 1;
                    if grid.get(&(xx, y)).unwrap() == &'.' {
                        continue;
                    }
                    if grid.get(&(xx, y)).unwrap() == &'#' {
                        break;
                    }
                    if grid.get(&(xx, y)).unwrap() == &'O' {
                        grid.insert((x, y), 'O');
                        grid.insert((xx, y), '.');
                        break;
                    }
                }
            }
        }
    }
}

fn grid_string(grid: &BTreeMap<(isize, isize), char>, width: isize, height: isize) -> String {
    let mut s = String::new();
    for y in 0..height {
        for x in 0..width {
            s.push(grid.get(&(x, y)).unwrap().clone());
        }
        s.push('\n');
    }
    s
}

fn process(input: &str) -> isize {
    let width = input.lines().next().unwrap().len() as isize;
    let height = input.lines().count() as isize;
    let mut grid: BTreeMap<(isize, isize), char> = BTreeMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            grid.insert((x as isize, y as isize), c);
        });
    });

    let mut cycle_detector: BTreeSet<String> = BTreeSet::new();
    let mut i = 0;
    let mut cycle_detected = false;
    while i < 1000000000 {
        tilt_north(&mut grid, width, height);
        tilt_west(&mut grid, width, height);
        tilt_south(&mut grid, width, height);
        tilt_east(&mut grid, width, height);
        let grid_str = grid_string(&grid, width, height);
        if cycle_detector.contains(&grid_str) && !cycle_detected {
            let cycle_start = cycle_detector.iter().position(|x| x == &grid_str).unwrap();
            let cycle_length = cycle_detector.len() - cycle_start;
            i += ((1000000000 - i) / cycle_length * cycle_length) - cycle_length;
            cycle_detected = true;
        } else {
            cycle_detector.insert(grid_str);
        }
        i += 1;
    }

    grid.iter()
        .filter(|(_, v)| **v == 'O')
        .map(|(k, _)| height - k.1)
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
        assert_eq!(process(TEST), 64);
    }
}
