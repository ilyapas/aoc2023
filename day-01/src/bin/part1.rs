fn process(input: &str) -> &str {
    input
}

fn main() {
    let input = include_str!("../../input.txt");
    process(input);
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
        assert_eq!(process(TEST), "");
    }
}
