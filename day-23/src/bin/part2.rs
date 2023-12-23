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

    let mut neighbors: BTreeMap<(isize, isize), BTreeSet<(isize, isize, usize)>> = BTreeMap::new();
    for y in 0..height as isize {
        for x in 0..width as isize {
            if grid.get(&(x, y)) == Some(&'#') {
                continue;
            }
            let mut n = BTreeSet::new();
            for (x, y) in vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)] {
                match grid.get(&(x, y)) {
                    Some('#') => continue,
                    Some(_) => n.insert((x, y, 1)),
                    None => continue,
                };
            }
            neighbors.insert((x, y), n);
        }
    }

    // Remove all nodes with only two edges
    loop {
        let mut found = false;
        let mut new_neighbors = neighbors.clone();
        for (k, v) in neighbors.iter() {
            if v.len() == 2 {
                let (x1, y1, dist1) = v.iter().next().unwrap();
                let (x2, y2, dist2) = v.iter().last().unwrap();
                if let Some(first) = new_neighbors.get_mut(&(*x1, *y1)) {
                    first.remove(&(k.0, k.1, *dist1));
                    first.insert((*x2, *y2, *dist1 + *dist2));
                }
                if let Some(second) = new_neighbors.get_mut(&(*x2, *y2)) {
                    second.remove(&(k.0, k.1, *dist2));
                    second.insert((*x1, *y1, *dist1 + *dist2));
                }
                new_neighbors.remove(k);
                found = true;
                break;
            }
        }
        neighbors = new_neighbors;
        if !found {
            break;
        }
    }

    let mut queue: VecDeque<(isize, isize, usize, BTreeSet<(isize, isize)>)> = VecDeque::new();
    let mut path_lengths: BTreeMap<usize, BTreeSet<(isize, isize)>> = BTreeMap::new();
    queue.push_back((start.0, start.1, 0, BTreeSet::new()));
    while queue.len() > 0 {
        let (x, y, steps, visited) = queue.pop_front().unwrap();
        let mut visited = visited.clone();
        visited.insert((x, y));
        if let Some(neighbors) = neighbors.get(&(x, y)) {
            for (x, y, dist) in neighbors.iter() {
                if visited.contains(&(*x, *y)) {
                    continue;
                }
                if (*x, *y) == end {
                    path_lengths.insert(steps + dist, visited.clone());
                    continue;
                }
                match grid.get(&(*x, *y)) {
                    Some('#') => continue,
                    Some(_) => queue.push_back((*x, *y, steps + dist, visited.clone())),
                    None => continue,
                }
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
        assert_eq!(process(TEST), 154);
    }
}
