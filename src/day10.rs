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

fn part2(input: String) -> TaskResult {
    todo!("{input}")
}
