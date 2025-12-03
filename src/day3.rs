use std::simd::prelude::*;

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

fn find_first_max(data: &[u8]) -> Option<(usize, u8)> {
    if data.is_empty() {
        return None;
    }

    let mut x = 0;
    let mut i = 0;

    for (j, y) in data.iter().cloned().enumerate() {
        if y > x {
            x = y;
            i = j;
        }
    }

    Some((i, x))
}

const LANES: usize = 64;

fn find_vector_with_max(body: &[Simd<u8, LANES>]) -> Option<(usize, u8)> {
    if body.is_empty() {
        return None;
    }

    assert!(body.len() <= u8::MAX as usize);

    let mut max_inds = Simd::<u8, LANES>::splat(0);
    let mut max_vals = Simd::splat(0);

    for (i, v) in body.iter().cloned().enumerate() {
        let m = v.simd_gt(max_vals);

        max_vals = m.select(v, max_vals);
        max_inds = m.select(Simd::splat(i as u8), max_inds);
    }

    let max_val = max_vals.reduce_max();
    let max_sub_ind = unsafe {
        max_vals
            .simd_eq(Simd::splat(max_val))
            .first_set()
            .unwrap_unchecked()
    };

    let max_ind = max_inds[max_sub_ind];

    let actual_ind = max_ind as usize * LANES + max_sub_ind;

    Some((actual_ind, max_val))
}

fn find_first_max_simd(batteries: &[u8]) -> Option<(usize, u8)> {
    let (head, body, tail) = batteries.as_simd::<LANES>();

    if body.len() > u8::MAX as usize {
        return None;
    }

    let (mut i, mut x) = find_first_max(head)
        .map(|(i, x)| (Some(i), Some(x)))
        .unwrap_or((None, None));

    if let Some((j, y)) = find_vector_with_max(body) {
        if let Some(x_val) = x
            && y > x_val
        {
            i = Some(head.len() + j);
            x = Some(y);
        } else if x.is_none() {
            i = Some(head.len() + j);
            x = Some(y);
        }
    }

    if let Some((j, y)) = find_first_max(tail) {
        if let Some(x_val) = x
            && y > x_val
        {
            i = Some(head.len() + body.len() * LANES + j);
            x = Some(y);
        } else if x.is_none() {
            i = Some(head.len() + body.len() * LANES + j);
            x = Some(y);
        }
    }

    i.and_then(|i| x.map(|x| (i, x)))
}

fn find_highest_joltage_simd(batteries: &[u8], n: usize) -> u64 {
    if n == 1 {
        return (batteries.iter().cloned().max().unwrap() - b'0') as u64;
    }

    let (i, a) =
        find_first_max_simd(&batteries[0..batteries.len() - (n - 1)]).unwrap();

    let a = a - b'0';

    let b = find_highest_joltage_simd(&batteries[i + 1..], n - 1);

    (a as u64) * 10u64.pow((n - 1) as u32) + b
}

fn part2(input: String) -> TaskResult {
    input
        .lines()
        .map(|l| l.as_bytes())
        .map(|x| find_highest_joltage_simd(x, 12))
        .sum::<u64>()
        .into()
}
