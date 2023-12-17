use std::collections::{BTreeMap, BTreeSet, VecDeque};

fn process(input: &str) -> usize {
    let mut grid: BTreeMap<(isize, isize), char> = BTreeMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as isize, y as isize), c);
        }
    }

    let width = input.lines().next().unwrap().len() as isize;
    let height = input.lines().count() as isize;
    let mut queue: VecDeque<(isize, isize, isize, isize)> = VecDeque::new();
    let mut visited: BTreeSet<(isize, isize, isize, isize)> = BTreeSet::new();
    let mut energized: BTreeSet<(isize, isize)> = BTreeSet::new();
    queue.push_back((0, 0, 1, 0));
    while queue.len() > 0 {
        let (x, y, dx, dy) = queue.pop_front().unwrap();
        if visited.contains(&(x, y, dx, dy)) {
            continue;
        }
        if x < 0 || x >= width || y < 0 || y >= height {
            continue;
        }
        visited.insert((x, y, dx, dy));
        energized.insert((x, y));
        match grid.get(&(x, y)) {
            Some('|') => {
                if dx == 0 {
                    queue.push_back((x, y + dy, 0, dy));
                } else {
                    queue.push_back((x, y - 1, 0, -1));
                    queue.push_back((x, y + 1, 0, 1));
                }
            }
            Some('-') => {
                if dy == 0 {
                    queue.push_back((x + dx, y, dx, 0));
                } else {
                    queue.push_back((x - 1, y, -1, 0));
                    queue.push_back((x + 1, y, 1, 0));
                }
            }
            Some('/') => {
                if dx == 0 {
                    queue.push_back((x - dy, y, -dy, 0));
                } else {
                    queue.push_back((x, y - dx, 0, -dx));
                }
            }
            Some('\\') => {
                if dx == 0 {
                    queue.push_back((x + dy, y, dy, 0));
                } else {
                    queue.push_back((x, y + dx, 0, dx));
                }
            }
            Some('.') => {
                queue.push_back((x + dx, y + dy, dx, dy));
            }
            _ => {}
        }
    }
    energized.len()
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 46);
    }
}
