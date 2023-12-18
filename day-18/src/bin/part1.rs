use itertools::Itertools;

fn process(input: &str) -> isize {
    let commands = input
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            (parts[0], parts[1].parse::<isize>().unwrap(), parts[2])
        })
        .collect::<Vec<_>>();

    let mut corners: Vec<(isize, isize)> = Vec::new();
    let mut pos = (0isize, 0isize);
    let mut boundary_points = 0;
    for command in commands.iter() {
        let direction = match command.0 {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!("Unknown direction"),
        };
        for _ in 0..command.1 {
            pos.0 += direction.0;
            pos.1 += direction.1;
        }
        corners.push(pos);
        boundary_points += command.1;
    }

    // shoelace formula (trapezoid method)
    let area = corners
        .iter()
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| (y2 + y1) * (x2 - x1))
        .sum::<isize>()
        .abs()
        / 2;

    // pick's theorem
    let interior_points = area - boundary_points / 2 + 1;

    boundary_points + interior_points
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 62);
    }
}
