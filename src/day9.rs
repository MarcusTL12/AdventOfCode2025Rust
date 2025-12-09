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

fn part2(input: String) -> TaskResult {
    todo!("{input}")
}
