fn process(input: &str) -> usize {
    let input = input.replace("\n", "");
    input.split(',').map(|s| hash(s)).sum()
}

fn hash(s: &str) -> usize {
    let mut result = 0;
    s.bytes().for_each(|c| {
        result += c as usize;
        result *= 17;
        result %= 256;
    });
    result
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 1320);
    }
}
