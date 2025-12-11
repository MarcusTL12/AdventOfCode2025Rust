use std::collections::HashMap;

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn parse_input(input: &str) -> (Vec<Vec<usize>>, usize, usize) {
    let mut translation = HashMap::new();

    fn translate<'a>(
        trans: &mut HashMap<&'a str, usize>,
        name: &'a str,
    ) -> usize {
        if let Some(&i) = trans.get(name) {
            i
        } else {
            let i = trans.len();
            trans.insert(name, i);
            i
        }
    }

    let mut network = Vec::new();

    for l in input.lines() {
        let (from, to) = l.split_once(": ").unwrap();

        let from = translate(&mut translation, from);

        loop {
            if let Some(v) = network.get_mut(from) {
                break v;
            } else {
                network.push(Vec::new());
            }
        }
        .extend(
            to.split_ascii_whitespace()
                .map(|x| translate(&mut translation, x)),
        );
    }

    let you = translation["you"];
    let out = translation["out"];

    (network, you, out)
}

fn count_paths(
    memo: &mut [Option<usize>],
    network: &[Vec<usize>],
    from: usize,
    to: usize,
) -> usize {
    if let Some(n) = memo[from] {
        return n;
    }

    if from == to {
        return 1;
    }

    let n = network[from]
        .iter()
        .map(|&x| count_paths(memo, network, x, to))
        .sum();

    memo[from] = Some(n);

    n
}

fn part1(input: String) -> TaskResult {
    let (network, you, out) = parse_input(&input);

    let mut memo = vec![None; network.len()];

    count_paths(&mut memo, &network, you, out).into()
}

fn part2(input: String) -> TaskResult {
    todo!("{input}")
}
