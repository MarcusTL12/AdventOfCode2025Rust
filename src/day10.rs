use rayon::prelude::*;

use z3::{Optimize, ast::Int};

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
    input
        .par_lines()
        .map(|l| {
            let s = Int::new_const("s");

            let (_, rest) = l.split_once(' ').unwrap();

            let mut parts = rest.split_ascii_whitespace().rev();

            let j = parts.next().unwrap();
            let joltages: Vec<_> = j[1..j.len() - 1]
                .split(',')
                .map(|x| x.parse::<u8>().unwrap())
                .collect();

            let mut w = vec![Int::from_u64(0); joltages.len()];

            let mut sv = Int::from_u64(0);

            let o = Optimize::new();

            for x in parts.rev().map(|s| {
                s[1..s.len() - 1]
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
            }) {
                let v = Int::fresh_const("v");

                for i in x {
                    w[i] += &v;
                }

                o.assert(&v.ge(0));

                sv += v;
            }

            for (w, j) in w.into_iter().zip(joltages) {
                o.assert(&w.eq(j));
            }

            o.assert(&s.eq(sv));

            o.minimize(&s);
            o.check(&[]);

            o.get_model()
                .unwrap()
                .eval(&s, true)
                .unwrap()
                .as_u64()
                .unwrap()
        })
        .sum::<u64>()
        .into()
}
