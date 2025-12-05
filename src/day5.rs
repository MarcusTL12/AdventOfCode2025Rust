use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn part1(input: String) -> TaskResult {
    let mut lines = input.lines();

    let mut ranges = Vec::new();

    while let Some(l) = lines.next()
        && !l.is_empty()
    {
        let (a, b) = l.split_once('-').unwrap();

        ranges.push(a.parse::<u64>().unwrap()..=b.parse().unwrap());
    }

    let ans = lines
        .map(|l| l.parse().unwrap())
        .filter(|x| ranges.iter().any(|r| r.contains(x)))
        .count();

    ans.into()
}

fn part2(input: String) -> TaskResult {
    todo!("{input}")
}
