use bitvec::prelude::*;
use rayon::prelude::*;

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn part1(input: String) -> TaskResult {
    let red_tiles: Vec<[u64; 2]> = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();

            [x.parse().unwrap(), y.parse().unwrap()]
        })
        .collect();

    red_tiles
        .iter()
        .enumerate()
        .flat_map(|(i, [x1, y1])| {
            red_tiles
                .iter()
                .take(i)
                .map(|&[x2, y2]| (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1))
        })
        .max()
        .unwrap()
        .into()
}

fn is_all_on(
    grid: &BitSlice,
    w: usize,
    x0: usize,
    x1: usize,
    y0: usize,
    y1: usize,
) -> bool {
    (y0..=y1).all(|y| grid[x0 + y * w..=x1 + y * w].all())
}

fn part2(input: String) -> TaskResult {
    let mut minx = usize::MAX;
    let mut miny = usize::MAX;
    let mut maxx = 0;
    let mut maxy = 0;

    let mut red_tiles: Vec<[usize; 2]> = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();

            let x = x.parse().unwrap();
            let y = y.parse().unwrap();

            minx = minx.min(x);
            miny = miny.min(y);
            maxx = maxx.max(x);
            maxy = maxy.max(y);

            [x, y]
        })
        .collect();

    for [x, y] in red_tiles.iter_mut() {
        *x -= minx;
        *y -= miny;
    }

    let w = maxx - minx + 1;
    let h = maxy - miny + 1;

    let mut grid = bitvec![0; w * h];

    let &[mut px, mut py] = red_tiles.last().unwrap();

    let mut up_walls = Vec::new();
    let mut down_walls = Vec::new();

    for &[x, y] in &red_tiles {
        if x != px {
            let x0 = x.min(px);
            let x1 = x.max(px);

            let i0 = x0 + y * w;
            let i1 = x1 + y * w;

            grid[i0..=i1].fill(true);
        } else if y > py {
            up_walls.push((x, [py, y]));
        } else if y < py {
            down_walls.push((x, [y, py]));
        }

        px = x;
        py = y;
    }

    up_walls.sort_unstable_by_key(|&(x, _)| x);
    down_walls.sort_unstable_by_key(|&(x, _)| x);

    let (in_walls, out_walls) = if up_walls.first().unwrap().0 == 0 {
        (up_walls, down_walls)
    } else {
        (down_walls, up_walls)
    };

    // TODO: make this faster
    for (x, [y0, y1]) in in_walls {
        for y in y0..=y1 {
            let x_end = out_walls
                .iter()
                .cloned()
                .skip_while(|&(ox, _)| ox <= x)
                .find_map(|(ox, [oy0, oy1])| {
                    (oy0..=oy1).contains(&y).then_some(ox)
                })
                .unwrap();

            grid[x + y * w..=x_end + y * w].fill(true);
        }
    }

    red_tiles
        .par_iter()
        .cloned()
        .enumerate()
        .flat_map(|(i, [x1, y1])| {
            red_tiles
                .par_iter()
                .cloned()
                .take(i)
                .map(move |[x2, y2]| {
                    [[x1.min(x2), x1.max(x2)], [y1.min(y2), y1.max(y2)]]
                })
                .filter(|&[[x0, x1], [y0, y1]]| {
                    is_all_on(&grid, w, x0, x1, y0, y1)
                })
                .map(move |[[x0, x1], [y0, y1]]| (x1 - x0 + 1) * (y1 - y0 + 1))
        })
        .max()
        .unwrap()
        .into()
}
