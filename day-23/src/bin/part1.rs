use std::collections::{BTreeMap, BTreeSet, VecDeque};

fn process(input: &str) -> usize {
    let mut grid: BTreeMap<(isize, isize), char> = BTreeMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            grid.insert((x as isize, y as isize), c);
            if y == 0 && c == '.' {
                start = (x as isize, y as isize);
            }
            if y == height - 1 && c == '.' {
                end = (x as isize, y as isize);
            }
        });
    });

    let mut queue: VecDeque<(isize, isize, isize, isize, usize, BTreeSet<(isize, isize)>)> =
        VecDeque::new();
    let mut path_lengths: BTreeMap<usize, BTreeSet<(isize, isize)>> = BTreeMap::new();
    queue.push_back((start.0, start.1, 0, 0, 0, BTreeSet::new()));
    while queue.len() > 0 {
        let (x, y, dx, dy, steps, visited) = queue.pop_front().unwrap();
        let mut visited = visited.clone();
        visited.insert((x, y));
        let mut directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        if dx != 0 || dy != 0 {
            directions = vec![(dx, dy)];
        }
        for (dx, dy) in directions {
            let x = x + dx;
            let y = y + dy;
            if x < 0 || y < 0 || x >= width as isize || y >= height as isize {
                continue;
            }
            if visited.contains(&(x, y)) {
                continue;
            }
            if (x, y) == end {
                path_lengths.insert(steps + 1, visited.clone());
                continue;
            }
            match grid.get(&(x, y)) {
                Some('#') => continue,
                Some('.') => queue.push_back((x, y, 0, 0, steps + 1, visited.clone())),
                Some('>') => queue.push_back((x, y, 1, 0, steps + 1, visited.clone())),
                Some('<') => queue.push_back((x, y, -1, 0, steps + 1, visited.clone())),
                Some('^') => queue.push_back((x, y, 0, -1, steps + 1, visited.clone())),
                Some('v') => queue.push_back((x, y, 0, 1, steps + 1, visited.clone())),
                Some(_) => continue,
                None => continue,
            }
        }
    }

    path_lengths.iter().map(|(k, _)| k).max().unwrap().clone()
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 94);
    }
}
