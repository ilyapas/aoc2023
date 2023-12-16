use std::collections::{BTreeMap, BTreeSet, VecDeque};

fn process(input: &str) -> usize {
    let mut redirects: BTreeMap<((isize, isize), (isize, isize)), Vec<(isize, isize)>> =
        BTreeMap::new();
    let mut grid: BTreeMap<(isize, isize), char> = BTreeMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x as isize;
            let y = y as isize;
            grid.insert((x, y), c);
            match c {
                '|' => {
                    redirects
                        .entry(((x - 1, y), (x, y)))
                        .or_default()
                        .push((x, y - 1));
                    redirects
                        .entry(((x - 1, y), (x, y)))
                        .or_default()
                        .push((x, y + 1));
                    redirects
                        .entry(((x + 1, y), (x, y)))
                        .or_default()
                        .push((x, y - 1));
                    redirects
                        .entry(((x + 1, y), (x, y)))
                        .or_default()
                        .push((x, y + 1));
                    redirects
                        .entry(((x, y - 1), (x, y)))
                        .or_default()
                        .push((x, y + 1));
                    redirects
                        .entry(((x, y + 1), (x, y)))
                        .or_default()
                        .push((x, y - 1));
                }
                '-' => {
                    redirects
                        .entry(((x, y - 1), (x, y)))
                        .or_default()
                        .push((x - 1, y));
                    redirects
                        .entry(((x, y - 1), (x, y)))
                        .or_default()
                        .push((x + 1, y));
                    redirects
                        .entry(((x, y + 1), (x, y)))
                        .or_default()
                        .push((x - 1, y));
                    redirects
                        .entry(((x, y + 1), (x, y)))
                        .or_default()
                        .push((x + 1, y));
                    redirects
                        .entry(((x - 1, y), (x, y)))
                        .or_default()
                        .push((x + 1, y));
                    redirects
                        .entry(((x + 1, y), (x, y)))
                        .or_default()
                        .push((x - 1, y));
                }
                '/' => {
                    redirects
                        .entry(((x - 1, y), (x, y)))
                        .or_default()
                        .push((x, y - 1));
                    redirects
                        .entry(((x + 1, y), (x, y)))
                        .or_default()
                        .push((x, y + 1));
                    redirects
                        .entry(((x, y - 1), (x, y)))
                        .or_default()
                        .push((x - 1, y));
                    redirects
                        .entry(((x, y + 1), (x, y)))
                        .or_default()
                        .push((x + 1, y));
                }
                '\\' => {
                    redirects
                        .entry(((x - 1, y), (x, y)))
                        .or_default()
                        .push((x, y + 1));
                    redirects
                        .entry(((x + 1, y), (x, y)))
                        .or_default()
                        .push((x, y - 1));
                    redirects
                        .entry(((x, y - 1), (x, y)))
                        .or_default()
                        .push((x + 1, y));
                    redirects
                        .entry(((x, y + 1), (x, y)))
                        .or_default()
                        .push((x - 1, y));
                }
                _ => {
                    redirects
                        .entry(((x - 1, y), (x, y)))
                        .or_default()
                        .push((x + 1, y));
                    redirects
                        .entry(((x + 1, y), (x, y)))
                        .or_default()
                        .push((x - 1, y));
                    redirects
                        .entry(((x, y - 1), (x, y)))
                        .or_default()
                        .push((x, y + 1));
                    redirects
                        .entry(((x, y + 1), (x, y)))
                        .or_default()
                        .push((x, y - 1));
                }
            }
        }
    }

    let width = input.lines().next().unwrap().len() as isize;
    let height = input.lines().count() as isize;
    let mut starts: Vec<((isize, isize), (isize, isize))> = vec![];
    for x in 0..width {
        starts.push(((x, -1), (x, 0)));
        starts.push(((x, height), (x, height - 1)));
    }
    for y in 0..height {
        starts.push(((-1, y), (0, y)));
        starts.push(((width, y), (width - 1, y)));
    }

    let mut result = 0;
    for start in starts {
        let mut queue: VecDeque<((isize, isize), (isize, isize))> = VecDeque::new();
        let mut visited: BTreeSet<((isize, isize), (isize, isize))> = BTreeSet::new();
        let mut energized: BTreeSet<(isize, isize)> = BTreeSet::new();
        queue.push_back(start);
        while queue.len() > 0 {
            let (prev, current) = queue.pop_front().unwrap();
            if visited.contains(&(prev, current)) {
                continue;
            }
            visited.insert((prev, current));
            if let Some(redirects) = redirects.get(&(prev, current)) {
                energized.insert(current);
                for redirect in redirects {
                    if visited.contains(&(current, *redirect)) {
                        continue;
                    }
                    queue.push_back((current, *redirect));
                }
            }
        }
        result = result.max(energized.len());
    }
    result
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
        assert_eq!(process(TEST), 51);
    }
}
