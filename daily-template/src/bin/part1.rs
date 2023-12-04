fn process(input: &str) -> usize {
    todo!()
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 0);
    }
}
