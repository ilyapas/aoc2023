fn process(input: &str) -> usize {
    let race_data = input
        .lines()
        .map(|line| {
            line.split(':')
                .last()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let times = race_data[0].clone();
    let distances = race_data[1].clone();

    times
        .iter()
        .enumerate()
        .map(|(i, t)| {
            (0..*t).fold(0, |acc, time| {
                let distance = time * (*t - time);
                if distance > distances[i] {
                    acc + 1
                } else {
                    acc
                }
            })
        })
        .product::<usize>()
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 288);
    }
}
