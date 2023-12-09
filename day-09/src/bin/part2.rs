fn next_value(row: &Vec<isize>) -> isize {
    let mut values = vec![row.clone()];
    let mut iteration = 0;
    while values.last().unwrap().iter().any(|v| *v != 0) {
        let derivative = values[iteration]
            .as_slice()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();
        values.push(derivative);
        iteration += 1;
    }
    let mut next_value = 0;
    values
        .iter()
        .rev()
        .for_each(|vals| next_value = vals.first().unwrap() - next_value);
    next_value
}

fn process(input: &str) -> isize {
    let rows = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    rows.iter().map(|row| next_value(row)).sum::<isize>()
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 2);
    }
}
