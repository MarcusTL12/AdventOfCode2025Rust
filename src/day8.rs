use std::collections::HashMap;

use arrayvec::ArrayVec;
use rayon::prelude::*;

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn find_root(forest: &mut [Option<usize>], i: usize) -> usize {
    if let Some(j) = forest[i] {
        let k = find_root(forest, j);

        if j != k {
            forest[i] = Some(k);
        }

        k
    } else {
        i
    }
}

fn join_trees(forest: &mut [Option<usize>], i: usize, j: usize) -> bool {
    let ri = find_root(forest, i);
    let rj = find_root(forest, j);

    if ri == rj {
        false
    } else {
        forest[rj] = Some(ri);
        true
    }
}

fn part1(input: String) -> TaskResult {
    let coords: Vec<[u64; 3]> = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|x| x.parse().unwrap())
                .collect::<ArrayVec<_, 3>>()
                .into_inner()
                .unwrap()
        })
        .collect();

    let mut pairs: Vec<_> = (0..coords.len())
        .flat_map(|i| (0..i).map(move |j| [i, j]))
        .collect();

    pairs.par_sort_unstable_by_key(|&[i, j]| {
        coords[i]
            .iter()
            .zip(&coords[j])
            .map(|(&x1, &x2)| x1.abs_diff(x2).pow(2))
            .sum::<u64>()
    });

    let mut forest = vec![None; coords.len()];

    for &[i, j] in pairs.iter().take(1000) {
        join_trees(&mut forest, i, j);
    }

    let mut sizes = HashMap::new();

    for i in 0..forest.len() {
        let ri = find_root(&mut forest, i);

        if let Some(x) = sizes.get_mut(&ri) {
            *x += 1;
        } else {
            sizes.insert(ri, 1u64);
        }
    }

    let mut sizes: Vec<_> = sizes.values().collect();

    sizes.sort_unstable();

    let ans: u64 = sizes.into_iter().rev().take(3).product();

    ans.into()
}

fn part2(input: String) -> TaskResult {
    let coords: Vec<[u64; 3]> = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|x| x.parse().unwrap())
                .collect::<ArrayVec<_, 3>>()
                .into_inner()
                .unwrap()
        })
        .collect();

    let mut pairs: Vec<_> = (0..coords.len())
        .flat_map(|i| (0..i).map(move |j| [i, j]))
        .collect();

    pairs.par_sort_unstable_by_key(|&[i, j]| {
        coords[i]
            .iter()
            .zip(&coords[j])
            .map(|(&x1, &x2)| x1.abs_diff(x2).pow(2))
            .sum::<u64>()
    });

    let mut forest = vec![None; coords.len()];

    let mut last_connection = [0, 0];

    for [i, j] in pairs {
        if join_trees(&mut forest, i, j) {
            last_connection = [i, j];
        }
    }

    (coords[last_connection[0]][0] * coords[last_connection[1]][0]).into()
}
