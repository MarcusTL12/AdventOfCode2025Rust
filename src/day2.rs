use rayon::prelude::*;

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn is_invalid(mut n: u64) -> bool {
    let Some(n_digits) = n.checked_ilog10().map(|l| l + 1) else {
        return false;
    };

    if n_digits % 2 != 0 {
        return false;
    }

    let mut bottom_half = 0;
    let mut cur_digit = 1;

    for _ in 0..n_digits / 2 {
        bottom_half += (n % 10) * cur_digit;
        n /= 10;
        cur_digit *= 10;
    }

    n == bottom_half
}

fn part1(input: String) -> TaskResult {
    let ans: u64 = input
        .lines()
        .next()
        .unwrap()
        .par_split(',')
        .flat_map(|r| {
            let (a, b) = r.split_once('-').unwrap();

            let a: u64 = a.parse().unwrap();
            let b: u64 = b.parse().unwrap();

            a..=b
        })
        .filter(|&x| is_invalid(x))
        .sum();

    ans.into()
}

fn is_invalid2(mut n: u64) -> bool {
    let mut digits = Vec::new();

    while n != 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }

    for c in 1..=digits.len() / 2 {
        if digits.len().is_multiple_of(c) {
            let mut chunks = digits.chunks_exact(c);

            let first_chunk = chunks.next().unwrap();

            if chunks.all(|x| x == first_chunk) {
                return true;
            }
        }
    }

    false
}

fn part2(input: String) -> TaskResult {
    let ans: u64 = input
        .lines()
        .next()
        .unwrap()
        .par_split(',')
        .flat_map(|r| {
            let (a, b) = r.split_once('-').unwrap();

            let a: u64 = a.parse().unwrap();
            let b: u64 = b.parse().unwrap();

            a..=b
        })
        .filter(|&x| is_invalid2(x))
        .sum();

    ans.into()
}
