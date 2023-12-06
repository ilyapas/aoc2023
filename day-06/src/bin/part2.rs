fn process(input: &str) -> usize {
    let race_data = input
        .lines()
        .map(|line| {
            line.split(':')
                .last()
                .unwrap()
                .replace(" ", "")
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    (0..race_data[0]).fold(0, |acc, time| {
        let distance = time * (race_data[0] - time);
        if distance > race_data[1] {
            acc + 1
        } else {
            acc
        }
    })
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
        assert_eq!(process(TEST), 71503);
    }
}
