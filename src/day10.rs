use std::{collections::HashMap, time::Instant};

use rayon::prelude::*;

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn combine_buttons(buttons: &[u16], mut x: usize) -> u16 {
    let mut ans = 0;

    for b in buttons {
        if x & 1 == 1 {
            ans ^= b;
        }

        x >>= 1;
    }

    ans
}

fn find_best_solution(lights: u16, buttons: &[u16]) -> usize {
    let mut best_sol = 0;
    let mut best_num = 64;

    for x in 0usize..(1 << buttons.len()) {
        let num_buttons = x.count_ones();

        if num_buttons < best_num && combine_buttons(buttons, x) == lights {
            best_sol = x;
            best_num = num_buttons;
        }
    }

    best_sol
}

fn part1(input: String) -> TaskResult {
    input
        .par_lines()
        .map_with(Vec::new(), |buttons, l| {
            let (lights, rest) = l.split_once(' ').unwrap();

            let lights: u16 = lights.as_bytes()[1..lights.len() - 1]
                .iter()
                .rev()
                .fold(0, |mask, &x| {
                    (mask << 1) | if x == b'#' { 1 } else { 0 }
                });

            buttons.clear();
            buttons.extend(rest.split_ascii_whitespace().rev().skip(1).map(
                |s| {
                    s[1..s.len() - 1]
                        .split(',')
                        .map(|x| x.parse().unwrap())
                        .fold(0u16, |mask, i: u8| mask | (1 << i))
                },
            ));

            find_best_solution(lights, buttons).count_ones()
        })
        .sum::<u32>()
        .into()
}

fn add_to_sum(cur_sum: &[u8], mut mask: u16) -> Vec<u8> {
    let mut new_sum = cur_sum.to_owned();

    let mut i = 0;

    while mask != 0 {
        if mask & 1 == 1 {
            new_sum[i] += 1;
        }

        mask >>= 1;
        i += 1;
    }

    new_sum
}

fn find_best_solution2(
    memo: &mut HashMap<(Vec<u8>, Option<usize>), Option<usize>>,
    limit: Option<usize>,
    joltages: &[u8],
    buttons: &[u16],
    cur_sum: &[u8],
) -> Option<usize> {
    if let Some(&x) = memo.get(&(cur_sum.to_owned(), limit)) {
        return x;
    }

    let n = joltages.len();
    assert_eq!(cur_sum.len(), n);

    let mut best = None;

    for &b in buttons {
        let new_sum = add_to_sum(cur_sum, b);

        let candidate =
            if new_sum.iter().zip(joltages.iter()).any(|(a, b)| a > b) {
                continue;
            } else if new_sum == joltages {
                Some(1)
            } else {
                find_best_solution2(
                    memo,
                    best.map(|x| x - 1),
                    joltages,
                    buttons,
                    &new_sum,
                )
                .map(|x| x + 1)
            };

        if let Some(x) = best
            && let Some(y) = candidate
            && y < x
        {
            best = candidate;

            if y == 1 {
                break;
            }
        } else if best.is_none()
            && let Some(y) = candidate
        {
            best = candidate;

            if y == 1 {
                break;
            }
        }
    }

    memo.insert((cur_sum.to_owned(), limit), best);

    best
}

fn part2(input: String) -> TaskResult {
    let lines: Vec<_> = input.lines().collect();

    let ans = lines
        .par_iter()
        .enumerate()
        .map_with((Vec::new(), Vec::new()), |(joltages, buttons), (i, l)| {
            let (_, rest) = l.split_once(' ').unwrap();

            let mut parts = rest.split_ascii_whitespace().rev();

            let j = parts.next().unwrap();
            joltages.clear();
            joltages.extend(
                j[1..j.len() - 1]
                    .split(',')
                    .map(|x| x.parse::<u8>().unwrap()),
            );

            buttons.clear();
            buttons.extend(parts.rev().map(|s| {
                s[1..s.len() - 1]
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .fold(0u16, |mask, i: u8| mask | (1 << i))
            }));

            let t = Instant::now();

            let mut memo = HashMap::new();
            let init_sum = vec![0; joltages.len()];

            let sol = find_best_solution2(
                &mut memo, None, joltages, buttons, &init_sum,
            )
            .unwrap();

            let t = t.elapsed();

            println!("{} => {sol} {t:.2?}", i + 1);

            sol
        })
        .sum::<usize>();

    ans.into()
}
