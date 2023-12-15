use std::collections::BTreeMap;

fn process(input: &str) -> usize {
    let input = input.replace("\n", "");
    let cmds: Vec<Vec<&str>> = input
        .split(',')
        .map(|s| s.split(&['=', '-']).filter(|s| s.len() != 0).collect())
        .collect();

    let mut boxes: BTreeMap<usize, Vec<(&str, &str)>> = BTreeMap::new();
    cmds.iter().for_each(|cmd| {
        let box_id = hash(cmd[0]);
        let b = boxes.entry(box_id).or_insert(vec![]);
        if cmd.len() > 1 {
            let mut found = false;
            for item in b.iter_mut() {
                if item.0 == cmd[0] {
                    item.1 = cmd[1];
                    found = true;
                }
            }
            if !found {
                b.push((cmd[0], cmd[1]));
            }
        } else {
            let b = b.clone();
            boxes.insert(
                box_id,
                b.iter()
                    .filter(|item| item.0 != cmd[0])
                    .map(|v| *v)
                    .collect(),
            );
        }
    });

    let mut result = 0;
    boxes.iter().for_each(|(k, v)| {
        v.iter().enumerate().for_each(|(i, (_, value))| {
            result += (k + 1) * (i + 1) * value.parse::<usize>().unwrap();
        });
    });

    result
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
        assert_eq!(process(TEST), 145);
    }
}
