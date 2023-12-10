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
    while !queue.is_empty() {
        let mut next_queue: BTreeSet<(isize, isize)> = BTreeSet::new();
        for node in queue.iter() {
            visited.insert(*node);
            let neighbors = neighbors.get(node).unwrap();
            for neighbor in neighbors.iter() {
                if next_queue.contains(neighbor) {
                    break;
                } else {
                    if !visited.contains(neighbor) {
                        next_queue.insert(*neighbor);
                    }
                }
            }
        }
        queue = next_queue;
    }

    let mut enclosed = 0;
    let mut inside = false;
    let mut from_top = false;
    let mut from_bottom = false;

    input.lines().enumerate().for_each(|(y, line)| {
        inside = false;
        from_top = false;
        from_bottom = false;
        line.chars().enumerate().for_each(|(x, c)| {
            let x = x as isize;
            let y = y as isize;
            if visited.contains(&(x, y)) {
                if c == '|' {
                    inside = !inside;
                }
                if c == 'L' {
                    from_top = true;
                }
                if c == '7' {
                    if from_top {
                        inside = !inside;
                    }
                    from_top = false;
                    from_bottom = false;
                }
                if c == 'F' {
                    from_bottom = true;
                }
                if c == 'J' {
                    if from_bottom {
                        inside = !inside;
                    }
                    from_top = false;
                    from_bottom = false;
                }
            } else {
                if inside {
                    enclosed += 1;
                }
            }
        });
    });

    enclosed
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            process(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            ),
            10
        );

        assert_eq!(
            process(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            ),
            8
        );
    }
}
