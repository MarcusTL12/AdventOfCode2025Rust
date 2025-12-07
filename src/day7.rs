use std::collections::HashMap;

use ndarray::{ArrayView2, ArrayViewMut2};

use crate::{
    Day, TaskResult,
    util::{input_to_grid, input_to_grid_mut},
};

pub const PARTS: Day = [part1, part2];

fn propagate_beam(grid: &mut ArrayViewMut2<u8>, [i, j]: [usize; 2]) -> usize {
    match grid.get([i, j]) {
        Some(b'.' | b'S') => {
            grid[[i, j]] = b'|';
            propagate_beam(grid, [i + 1, j])
        }
        Some(b'^') => {
            1 + propagate_beam(grid, [i, j.wrapping_sub(1)])
                + propagate_beam(grid, [i, j + 1])
        }
        _ => 0,
    }
}

fn part1(mut input: String) -> TaskResult {
    let mut grid = input_to_grid_mut(unsafe { input.as_bytes_mut() });

    let j = grid
        .rows()
        .into_iter()
        .next()
        .unwrap()
        .into_iter()
        .enumerate()
        .find_map(|(j, &x)| (x == b'S').then_some(j))
        .unwrap();

    let ans = propagate_beam(&mut grid, [0, j]);

    ans.into()
}

fn count_timelines(
    memo: &mut HashMap<[usize; 2], usize>,
    grid: ArrayView2<u8>,
    [i, j]: [usize; 2],
) -> usize {
    if let Some(&x) = memo.get(&[i, j]) {
        return x;
    }

    match grid.get([i, j]) {
        Some(b'.' | b'S') => count_timelines(memo, grid, [i + 1, j]),
        Some(b'^') => {
            let ka = [i, j.wrapping_sub(1)];
            let kb = [i, j + 1];
            let a = count_timelines(memo, grid, ka);
            memo.insert(ka, a);
            let b = count_timelines(memo, grid, [i, j + 1]);
            memo.insert(kb, b);

            a + b
        }
        _ => 1,
    }
}

fn part2(input: String) -> TaskResult {
    let grid = input_to_grid(input.as_bytes());

    let j = grid
        .rows()
        .into_iter()
        .next()
        .unwrap()
        .into_iter()
        .enumerate()
        .find_map(|(j, &x)| (x == b'S').then_some(j))
        .unwrap();

    let ans = count_timelines(&mut HashMap::new(), grid, [0, j]);

    ans.into()
}
