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

    let (h, w) = grid.dim();

    let mut done = false;

    let mut ans = 0;

    while !done {
        done = true;

        for i in 0..h {
            for j in 0..w {
                if grid[[i, j]] == b'@' {
                    let mut c = 0;

                    'count_loop: for di in -1..=1 {
                        let i = (i as isize + di) as usize;
                        for dj in -1..=1 {
                            let j = (j as isize + dj) as usize;

                            if let Some(&x) = grid.get([i, j])
                                && x == b'@'
                            {
                                c += 1;

                                if c >= 5 {
                                    break 'count_loop;
                                }
                            }
                        }
                    }

                    if c < 5 {
                        grid[[i, j]] = b'.';
                        ans += 1;
                        done = false;
                    }
                }
            }
        }
    }

    ans.into()
}
