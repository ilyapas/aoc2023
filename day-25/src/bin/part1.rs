use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, VecDeque},
};

fn process(input: &str) -> usize {
    let mut nodes: BTreeSet<&str> = BTreeSet::new();
    let mut edges: BTreeSet<(&str, &str)> = BTreeSet::new();
    let mut neighbors: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    input.lines().for_each(|line| {
        let mut parts = line.split(": ");
        let parent = parts.next().unwrap();
        let children = parts.next().unwrap().split(" ");
        nodes.insert(parent);
        neighbors
            .entry(parent)
            .or_insert_with(BTreeSet::new)
            .extend(children.clone());
        for child in children {
            nodes.insert(child);
            if !edges.contains(&(parent, child)) {
                edges.insert((child, parent));
            }
            neighbors
                .entry(child)
                .or_insert_with(BTreeSet::new)
                .insert(parent);
        }
    });

    let mut alt_distances: Vec<((&str, &str), usize)> = edges
        .iter()
        .map(|(a, b)| {
            let mut visited: BTreeSet<&str> = BTreeSet::new();
            let mut queue: VecDeque<(&str, usize)> = VecDeque::new();
            queue.push_back((*a, 0));
            let mut new_neighbors: BTreeMap<&str, BTreeSet<&str>> = neighbors.clone();
            new_neighbors.get_mut(*a).unwrap().remove(*b);
            new_neighbors.get_mut(*b).unwrap().remove(*a);
            while let Some((node, distance)) = queue.pop_front() {
                if node == *b {
                    return ((*a, *b), distance);
                }
                if visited.contains(node) {
                    continue;
                }
                visited.insert(node);
                let nb = new_neighbors.get(node).unwrap();
                for n in nb {
                    if visited.contains(n) {
                        continue;
                    }
                    queue.push_back((*n, distance + 1));
                }
            }
            unreachable!()
        })
        .collect();

    alt_distances.sort_by_key(|(_, d)| Reverse(*d));

    let removed_edges = alt_distances
        .iter()
        .take(3)
        .map(|((a, b), _)| (*a, *b))
        .collect::<Vec<_>>();

    removed_edges.iter().for_each(|(a, b)| {
        neighbors.get_mut(a).unwrap().remove(b);
        neighbors.get_mut(b).unwrap().remove(a);
    });

    let mut result = vec![];

    for node in [removed_edges[0].0, removed_edges[0].1].iter() {
        let mut visited: BTreeSet<&str> = BTreeSet::new();
        let mut queue: VecDeque<&str> = VecDeque::new();
        queue.push_back(node);
        while let Some(node) = queue.pop_front() {
            if visited.contains(node) {
                continue;
            }
            visited.insert(node);
            let nb = neighbors.get(node).unwrap();
            for n in nb {
                if visited.contains(n) {
                    continue;
                }
                queue.push_back(*n);
            }
        }
        result.push(visited.len());

    }

    result.iter().product()
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 54);
    }
}
