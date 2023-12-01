fn process(input: &str) -> String {
    let result = input
        .lines()
        .map(|line| {
            let digits = line.chars().filter(|c| c.is_numeric()).collect::<String>();
            let first = digits
                .chars()
                .next()
                .unwrap()
                .to_string()
                .parse::<u32>()
                .unwrap();
            let last = digits
                .chars()
                .last()
                .unwrap()
                .to_string()
                .parse::<u32>()
                .unwrap();
            first * 10 + last
        })
        .sum::<u32>();
    result.to_string()
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), "142");
    }
}
