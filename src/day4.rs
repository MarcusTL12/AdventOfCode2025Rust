use arrayvec::ArrayVec;

use crate::{
    Day, TaskResult,
    util::{input_to_grid, input_to_grid_mut},
};

pub const PARTS: Day = [part1, part2];

fn part1(input: String) -> TaskResult {
    let grid = input_to_grid(input.as_bytes());

    let ans = grid
        .indexed_iter()
        .filter(|&(_, &x)| x == b'@')
        .filter(|&((i, j), _)| {
            (-1..=1)
                .flat_map(|di| (-1..=1).map(move |dj| [di, dj]))
                .filter_map(move |[di, dj]| {
                    let i = (i as isize + di) as usize;
                    let j = (j as isize + dj) as usize;

                    grid.get([i, j]).cloned()
                })
                .filter(|&x| x == b'@')
                .count()
                < 5
        })
        .count();

    ans.into()
}

fn part2(mut input: String) -> TaskResult {
    let mut grid = input_to_grid_mut(unsafe { input.as_bytes_mut() });

    let mut stack: Vec<_> = grid
        .indexed_iter()
        .filter_map(|((i, j), &x)| (x == b'@').then_some([i, j]))
        .collect();

    let mut ans = 0;

    'outer: while let Some([i, j]) = stack.pop() {
        if grid[[i, j]] == b'@' {
            let mut neighbours = ArrayVec::<_, 3>::new();

            for di in -1..=1 {
                let ni = (i as isize + di) as usize;
                for dj in -1..=1 {
                    let nj = (j as isize + dj) as usize;

                    if [ni, nj] != [i, j]
                        && let Some(&x) = grid.get([ni, nj])
                        && x == b'@'
                        && neighbours.try_push([ni, nj]).is_err()
                    {
                        continue 'outer;
                    }
                }
            }

            grid[[i, j]] = b'.';
            ans += 1;
            stack.extend(neighbours);
        }
    }

    ans.into()
}
