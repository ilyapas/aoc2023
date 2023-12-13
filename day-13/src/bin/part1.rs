fn process(input: &str) -> usize {
    let blocks = input.split("\n\n").collect::<Vec<&str>>();
    blocks
        .iter()
        .map(|b| {
            if let Some(result) = mirror(b) {
                return 100 * result;
            } else {
                let bb = rotate(b);
                return mirror(&bb).unwrap();
            }
        })
        .sum()
}

fn rotate(b: &str) -> String {
    let w = b.lines().next().unwrap().len();
    let h = b.lines().count();
    let mut result = String::new();
    for x in 0..w {
        for y in (0..h).rev() {
            result.push(b.lines().nth(y).unwrap().chars().nth(x).unwrap());
        }
        result.push('\n');
    }
    result
}

fn mirror(b: &str) -> Option<usize> {
    let size = b.lines().count();
    for i in 0..size {
        for j in 1..size - i {
            if b.lines().nth(i + j).unwrap() == b.lines().nth(1 + i - j).unwrap() {
                if i + j == size - 1 || 1 + i - j == 0 {
                    return Some(i + 1);
                }
            } else {
                break;
            }
        }
    }
    None
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 405);
    }
}
