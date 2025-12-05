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

fn insert_new_range(ranges: &mut Vec<[u64; 2]>, [a0, a1]: [u64; 2]) {
    if ranges.is_empty() {
        ranges.push([a0, a1]);
        return;
    }

    let mut i = 0;

    while let Some(&[_, b1]) = ranges.get(i)
        && a0 > b1 + 1
    {
        i += 1;
    }

    let mut j = i;

    while let Some(&[_, b1]) = ranges.get(j)
        && a1 > b1 + 1
    {
        j += 1;
    }

    if i >= ranges.len() {
        ranges.push([a0, a1]);
    } else {
        let new0 = ranges[i][0].min(a0);

        let (in_right, new1) = if let Some(&[h0, h1]) = ranges.get(j)
            && h0 <= a1 + 1
        {
            (true, h1.max(a1))
        } else {
            (false, a1)
        };

        if i == j && !in_right {
            ranges.insert(i, [a0, a1]);
        } else {
            ranges[i] = [new0, new1];
            for _ in 0..(j - i - ((!in_right) as usize)) {
                ranges.remove(i + 1);
            }
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

        insert_new_range(&mut ranges, [a, b]);
    }

    ranges
        .into_iter()
        .map(|[a, b]| b - a + 1)
        .sum::<u64>()
        .into()
}
