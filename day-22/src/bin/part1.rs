use std::collections::BTreeSet;

use itertools::Itertools;

fn process(input: &str) -> usize {
    let mut grid: BTreeSet<(isize, isize, isize)> = BTreeSet::new();
    let mut bricks: Vec<BTreeSet<(isize, isize, isize)>> = vec![];
    input.lines().for_each(|line| {
        let mut brick: BTreeSet<(isize, isize, isize)> = BTreeSet::new();
        line.split('~').for_each(|edge| {
            let (x, y, z) = edge.split(',').collect_tuple().unwrap();
            let x = x.parse::<isize>().unwrap();
            let y = y.parse::<isize>().unwrap();
            let z = z.parse::<isize>().unwrap();
            brick.insert((x, y, z));
        });
        let b = brick.clone();
        let start = b.iter().next().unwrap();
        let end = b.iter().last().unwrap();
        let mut count = 0;
        if start.0 != end.0 {
            for x in start.0.min(end.0)..start.0.max(end.0) {
                brick.insert((x, start.1, start.2));
            }
            count += 1;
        }
        if start.1 != end.1 {
            for y in start.1.min(end.1)..start.1.max(end.1) {
                brick.insert((start.0, y, start.2));
            }
            count += 1;
        }
        if start.2 != end.2 {
            for z in start.2.min(end.2)..start.2.max(end.2) {
                brick.insert((start.0, start.1, z));
            }
            count += 1;
        }
        assert!(count <= 1);
        grid.extend(brick.clone());
        bricks.push(brick);
    });

    bricks.sort_by_key(|b| *b.iter().map(|(_, _, z)| z).min().unwrap());

    let mut new_bricks: BTreeSet<BTreeSet<(isize, isize, isize)>> = BTreeSet::new();
    for b in bricks.iter() {
        let mut brick = b.clone();
        for cube in b.iter() {
            grid.remove(cube);
        }
        loop {
            let min_z = brick.iter().map(|(_, _, z)| z).min().unwrap();
            if *min_z == 1 {
                break;
            }
            let moved_brick = brick
                .iter()
                .map(|(x, y, z)| (*x, *y, z - 1))
                .collect::<BTreeSet<_>>();
            if moved_brick.intersection(&grid).count() > 0 {
                break;
            } else {
                brick = moved_brick;
            }
        }
        grid.extend(brick.clone());
        new_bricks.insert(brick);
    }

    let mut removable: Vec<BTreeSet<(isize, isize, isize)>> = vec![];
    for b in new_bricks.iter() {
        let mut moves = 0;
        let mut bricks = new_bricks.clone();
        bricks.remove(b);
        let mut grid = grid.clone();
        for cube in b.iter() {
            grid.remove(cube);
        }

        for brick in bricks.iter() {
            let min_z = brick.iter().map(|(_, _, z)| z).min().unwrap();
            if *min_z == 1 {
                continue;
            }
            for cube in brick.iter() {
                grid.remove(cube);
            }
            let moved_brick = brick
                .iter()
                .map(|(x, y, z)| (*x, *y, z - 1))
                .collect::<BTreeSet<_>>();
            if moved_brick.intersection(&grid).count() == 0 {
                moves += 1;
            }
            grid.extend(brick.clone());
        }
        if moves == 0 {
            removable.push(b.clone());
        }
    }

    removable.len()
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 5);
    }
}
