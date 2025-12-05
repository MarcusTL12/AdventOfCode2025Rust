use std::ops::RangeInclusive;

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

fn combine_ranges(
    [a0, a1]: [u64; 2],
    [b0, b1]: [u64; 2],
) -> ([u64; 2], Option<[u64; 2]>) {
    if a1 + 1 < b0 {
        ([a0, a1], Some([b0, b1]))
    } else if b1 + 1 < a0 {
        ([b0, b1], Some([a0, a1]))
    } else {
        ([a0.min(b0), a1.max(b1)], None)
    }
}

fn insert_new_range(ranges: &mut Vec<[u64; 2]>, [a0, a1]: [u64; 2]) {
    if ranges.is_empty() {
        ranges.push([a0, a1]);
        return;
    }

    let mut lo = 0;
    let mut hi = ranges.len() - 1;

    while lo < hi {
        let mid = lo + (hi - lo) / 2;

        let [m0, m1] = ranges[mid];

        if m1 + 1 < a0 {
            lo = mid + 1;
        } else if a1 + 1 < m0 {
            hi = mid - 1;
        } else {
            // Deal with overlap
        }
    }
}

fn part2(input: String) -> TaskResult {
    let mut lines = input.lines();

    let mut ranges = Vec::new();

    while let Some(l) = lines.next()
        && !l.is_empty()
    {
        let (a, b) = l.split_once('-').unwrap();

        let a: u64 = a.parse().unwrap();
        let b: u64 = b.parse().unwrap();

        ranges.push([a, b]);
    }

    todo!()
}
