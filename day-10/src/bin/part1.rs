use std::collections::{BTreeMap, BTreeSet};

fn process(input: &str) -> usize {
    let mut neighbors: BTreeMap<(isize, isize), BTreeSet<(isize, isize)>> = BTreeMap::new();
    let mut start: (isize, isize) = (0, 0);
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let x = x as isize;
            let y = y as isize;
            let mut neighbors_set: BTreeSet<(isize, isize)> = BTreeSet::new();
            match c {
                '|' => {
                    neighbors_set.insert((x, y - 1));
                    neighbors_set.insert((x, y + 1));
                }
                '-' => {
                    neighbors_set.insert((x - 1, y));
                    neighbors_set.insert((x + 1, y));
                }
                'L' => {
                    neighbors_set.insert((x + 1, y));
                    neighbors_set.insert((x, y - 1));
                }
                'J' => {
                    neighbors_set.insert((x - 1, y));
                    neighbors_set.insert((x, y - 1));
                }
                'F' => {
                    neighbors_set.insert((x + 1, y));
                    neighbors_set.insert((x, y + 1));
                }
                '7' => {
                    neighbors_set.insert((x - 1, y));
                    neighbors_set.insert((x, y + 1));
                }
                'S' => {
                    start = (x, y);
                }
                _ => {}
            }
            neighbors.insert((x, y), neighbors_set);
        });
    });

    let mut start_neighbors: BTreeSet<(isize, isize)> = BTreeSet::new();
    neighbors.iter().for_each(|(k, v)| {
        if v.contains(&(start.0, start.1)) {
            start_neighbors.insert(*k);
        }
    });
    neighbors.insert(start, start_neighbors);

    let mut queue: BTreeSet<(isize, isize)> = BTreeSet::new();
    queue.insert(start);
    let mut visited: BTreeSet<(isize, isize)> = BTreeSet::new();
    let mut num_steps = 0;
    while !queue.is_empty() {
        num_steps += 1;
        let mut next_queue: BTreeSet<(isize, isize)> = BTreeSet::new();
        for node in queue.iter() {
            visited.insert(*node);
            let neighbors = neighbors.get(node).unwrap();
            for neighbor in neighbors.iter() {
                if next_queue.contains(neighbor) {
                    return num_steps;
                } else {
                    if !visited.contains(neighbor) {
                        next_queue.insert(*neighbor);
                    }
                }
            }
        }
        queue = next_queue;
    }
    num_steps
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 8);
    }
}
