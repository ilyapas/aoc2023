use std::collections::BTreeSet;

use itertools::Itertools;

fn process(input: &str, min: f64, max: f64) -> usize {
    let stones = input
        .lines()
        .map(|line| {
            line.split(['@', ','])
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect_tuple::<(_, _, _, _, _, _)>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let size = stones.len();

    let mut count = 0;
    let mut seen: BTreeSet<(usize, usize)> = BTreeSet::new();
    for i in 0..size {
        for j in 0..size {
            if i == j {
                continue;
            }
            if seen.contains(&(i, j)) {
                continue;
            }
            let (px1, py1, _, vx1, vy1, _) = stones[i];
            let (px2, py2, _, vx2, vy2, _) = stones[j];

            // Equation: py1 + (x - px1) * vy1/vx1 = py2 + (x - px2) * vy2/vx2
            let x = (py1 * vx1 * vx2 - px1 * vy1 * vx2 - py2 * vx1 * vx2 + px2 * vy2 * vx1)
                / (vx1 * vy2 - vy1 * vx2);
            let y = (x - px1) * vy1 / vx1 + py1;
            if x >= min
                && x <= max
                && y >= min
                && y <= max
                && (x >= px1 && vx1 >= 0.0 || x <= px1 && vx1 <= 0.0)
                && (x >= px2 && vx2 >= 0.0 || x <= px2 && vx2 <= 0.0)
                && (y >= py1 && vy1 >= 0.0 || y <= py1 && vy1 <= 0.0)
                && (y >= py2 && vy2 >= 0.0 || y <= py2 && vy2 <= 0.0)
            {
                count += 1;
            }
            seen.insert((i, j));
            seen.insert((j, i));
        }
    }
    count
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input, 200000000000000., 400000000000000.));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST, 7., 27.), 2);
    }
}
