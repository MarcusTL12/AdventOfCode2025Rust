use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn part1(input: String) -> TaskResult {
    let ans = input
        .lines()
        .map(|l| {
            let rl = l.chars().next().unwrap();

            let n: i32 = l[1..].parse().unwrap();

            (rl, n)
        })
        .scan(50, |dial, (rl, n)| {
            *dial = match rl {
                'R' => *dial + n,
                'L' => *dial - n,
                _ => panic!(),
            } % 100;

            if *dial < 0 {
                *dial += 100;
            }

            Some(*dial)
        })
        .filter(|&x| x == 0)
        .count();

    ans.into()
}

fn part2(input: String) -> TaskResult {
    let mut dial = 50;
    let mut clicks = 0;

    for (rl, n) in input.lines().map(|l| {
        let rl = l.chars().next().unwrap();

        let n: u32 = l[1..].parse().unwrap();

        (rl, n)
    }) {
        match rl {
            'R' => {
                for _ in 0..n {
                    if dial == 99 {
                        dial = 0;
                        clicks += 1;
                    } else {
                        dial += 1;
                    }
                }
            }
            'L' => {
                for _ in 0..n {
                    if dial == 1 {
                        clicks += 1;
                        dial = 0;
                    } else if dial == 0 {
                        dial = 99;
                    } else {
                        dial -= 1;
                    }
                }
            }
            _ => panic!(),
        }
    }

    clicks.into()
}
