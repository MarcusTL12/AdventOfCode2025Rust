use crate::{Day, TaskResult, util::input_to_grid};

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
    let grid = input_to_grid(input.as_bytes());
    let grid = grid.t();

    let mut op = b'+';
    let mut ans = 0;
    let mut cur_res = 0;

    for r in grid.rows() {
        if r.iter().all(|&x| x == b' ') {
            ans += cur_res;
        } else {
            let mut n = 0;
            for x in r.iter().filter(|x| x.is_ascii_digit()) {
                n = 10 * n + (x - b'0') as u64;
            }

            let &rl = r.last().unwrap();

            match rl {
                b'+' | b'*' => {
                    cur_res = n;
                    op = rl;
                }
                _ => match op {
                    b'+' => cur_res += n,
                    b'*' => cur_res *= n,
                    _ => unreachable!(),
                },
            }
        }
    }

    ans += cur_res;

    ans.into()
}
