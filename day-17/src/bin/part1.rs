use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

fn process(input: &str) -> isize {
    let mut grid: BTreeMap<(isize, isize), isize> = BTreeMap::new();
    let width = input.lines().next().unwrap().len() as isize;
    let height = input.lines().count() as isize;
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let digit = c.to_digit(10).unwrap() as isize;
            grid.insert((x as isize, y as isize), digit);
        });
    });

    let mut queue: BinaryHeap<(isize, isize, isize, isize, isize, isize)> = BinaryHeap::new();
    let mut visited: BTreeSet<(isize, isize, isize, isize, isize)> = BTreeSet::new();
    queue.push((0, 0, 0, 0, 0, 0));
    while queue.len() > 0 {
        let (heat, x, y, dx, dy, straight) = queue.pop().unwrap();
        let heat = -heat;
        if x == width - 1 && y == height - 1 {
            return heat;
        }
        if visited.contains(&(x, y, dx, dy, straight)) {
            continue;
        }
        visited.insert((x, y, dx, dy, straight));
        if straight < 3 && (dx, dy) != (0, 0) {
            let xx = x + dx;
            let yy = y + dy;
            if xx >= 0 && xx < width && yy >= 0 && yy < height {
                let heat = heat + grid.get(&(xx, yy)).unwrap();
                queue.push((-heat, xx, yy, dx, dy, straight + 1));
            }
        }
        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dxx, dyy) in directions {
            if (dxx, dyy) == (-dx, -dy) || (dxx, dyy) == (dx, dy) {
                continue;
            }
            let xx = x + dxx;
            let yy = y + dyy;
            if xx >= 0 && xx < width && yy >= 0 && yy < height {
                let heat = heat + grid.get(&(xx, yy)).unwrap();
                queue.push((-heat, xx, yy, dxx, dyy, 1));
            }
        }
    }
    unreachable!()
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 102);
    }
}
