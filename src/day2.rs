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
        .split(',')
        .flat_map(|r| {
            let (a, b) = r.split_once('-').unwrap();

            let a = a.parse().unwrap();
            let b = b.parse().unwrap();

            a..=b
        })
        .filter(|&x| is_invalid(x))
        .sum();

    ans.into()
}

fn part2(input: String) -> TaskResult {
    todo!("{input}")
}
