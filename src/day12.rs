use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn part1(input: String) -> TaskResult {
    let mut parts = input.split("\n\n");

    let gifts: Vec<u32> = (&mut parts)
        .take(6)
        .map(|p| {
            p.lines()
                .skip(1)
                .flat_map(|l| {
                    l.chars().map(|c| match c {
                        '#' => 1,
                        '.' => 0,
                        _ => panic!(),
                    })
                })
                .sum()
        })
        .collect();

    println!("{gifts:?}");

    parts
        .next()
        .unwrap()
        .lines()
        .filter(|l| {
            let (area, num_gifts) = l.split_once(": ").unwrap();

            let area: u32 =
                area.split('x').map(|x| x.parse::<u32>().unwrap()).product();

            let gift_areas: u32 = num_gifts
                .split_ascii_whitespace()
                .enumerate()
                .map(|(i, x)| x.parse::<u32>().unwrap() * gifts[i])
                .sum();

            gift_areas <= area
        })
        .count()
        .into()
}

fn part2(input: String) -> TaskResult {
    todo!("{input}")
}
