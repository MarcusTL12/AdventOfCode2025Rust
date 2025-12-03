use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn find_highest_joltage(batteries: &[u8]) -> u8 {
    assert!(batteries.len() >= 2);

    let mut a = batteries[0];
    let mut b = batteries[1];

    for i in 1..batteries.len() - 1 {
        if batteries[i] > a {
            a = batteries[i];
            b = batteries[i + 1];
        } else if batteries[i] > b {
            b = batteries[i];
        }
    }

    if let Some(&x) = batteries.last()
        && x > b
    {
        b = x;
    }

    10 * (a - b'0') + (b - b'0')
}

fn part1(input: String) -> TaskResult {
    input
        .lines()
        .map(|l| l.as_bytes())
        .map(|x| find_highest_joltage(x) as u32)
        .sum::<u32>()
        .into()
}

fn part2(input: String) -> TaskResult {
    todo!("{input}")
}
