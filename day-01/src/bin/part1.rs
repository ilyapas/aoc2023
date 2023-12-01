fn process(input: &str) -> &str {
    input
}

fn main() {
    let input = include_str!("../input.txt");
    process(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), "");
    }
}
