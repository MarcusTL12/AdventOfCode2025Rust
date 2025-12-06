use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn part1(input: String) -> TaskResult {
    let mut num_rows: Vec<Vec<u64>> = Vec::new();

    for l in input.lines() {
        if let Some('+' | '*') = l.chars().next() {
            let ans = l
                .split_ascii_whitespace()
                .enumerate()
                .map(|(i, x)| match x.chars().next() {
                    Some('+') => num_rows.iter().map(|r| r[i]).sum::<u64>(),
                    Some('*') => num_rows.iter().map(|r| r[i]).product::<u64>(),
                    _ => panic!(),
                })
                .sum::<u64>();

            return ans.into();
        } else {
            num_rows.push(
                l.split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            );
        }
    }

    panic!()
}

fn part2(input: String) -> TaskResult {
    todo!("{input}")
}
