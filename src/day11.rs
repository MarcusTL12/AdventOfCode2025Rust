use std::{collections::HashMap, mem};

use arrayvec::ArrayVec;

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

#[derive(Debug, Clone)]
enum HybVec<T, const N: usize> {
    Short(ArrayVec<T, N>),
    Long(Vec<T>),
}

impl<T, const N: usize> HybVec<T, N> {
    const fn new() -> Self {
        Self::Short(ArrayVec::new_const())
    }

    fn push(&mut self, v: T) {
        match self {
            Self::Short(s) => {
                if let Err(v) = s.try_push(v) {
                    let v = v.element();

                    let Self::Short(old_self) =
                        mem::replace(self, Self::Long(Vec::new()))
                    else {
                        unreachable!()
                    };

                    let Self::Long(new_vec) = self else {
                        unreachable!()
                    };

                    new_vec.extend(old_self);
                    new_vec.push(v);
                }
            }
            Self::Long(s) => s.push(v),
        }
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        match self {
            Self::Short(s) => s.iter(),
            Self::Long(s) => s.iter(),
        }
    }
}

impl<T, const N: usize> Extend<T> for HybVec<T, N> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for v in iter {
            self.push(v);
        }
    }
}

fn parse_input<const N: usize>(
    input: &str,
) -> (Vec<HybVec<usize, N>>, HashMap<&str, usize>) {
    let mut translation = HashMap::new();

    fn translate<'a>(
        trans: &mut HashMap<&'a str, usize>,
        name: &'a str,
    ) -> usize {
        if let Some(&i) = trans.get(name) {
            i
        } else {
            let i = trans.len();
            trans.insert(name, i);
            i
        }
    }

    let mut network = Vec::new();

    for l in input.lines() {
        let (from, to) = l.split_once(": ").unwrap();

        let from = translate(&mut translation, from);

        loop {
            if let Some(v) = network.get_mut(from) {
                break v;
            } else {
                network.push(HybVec::new());
            }
        }
        .extend(
            to.split_ascii_whitespace()
                .map(|x| translate(&mut translation, x)),
        );
    }

    (network, translation)
}

fn count_paths<const N: usize>(
    memo: &mut [Option<usize>],
    network: &[HybVec<usize, N>],
    from: usize,
    to: usize,
) -> usize {
    if let Some(n) = memo[from] {
        return n;
    }

    if from == to {
        return 1;
    }

    let n = network[from]
        .iter()
        .map(|&x| count_paths(memo, network, x, to))
        .sum();

    memo[from] = Some(n);

    n
}

fn part1(input: String) -> TaskResult {
    let (network, trans) = parse_input::<8>(&input);

    let mut memo = vec![None; network.len()];

    count_paths(&mut memo, &network, trans["you"], trans["out"]).into()
}

fn count_paths2<const N: usize, const M: usize>(
    memo: &mut HashMap<(usize, [bool; N]), usize>,
    network: &[HybVec<usize, M>],
    mut k: (usize, [bool; N]),
    stops: [usize; N],
    to: usize,
) -> usize {
    for (been, i) in k.1.iter_mut().zip(stops) {
        if k.0 == i {
            *been = true;
        }
    }

    if k.0 == to {
        return if k.1 == [true; N] { 1 } else { 0 };
    }

    if let Some(&n) = memo.get(&k) {
        return n;
    }

    let n = network[k.0]
        .iter()
        .map(|&x| count_paths2(memo, network, (x, k.1), stops, to))
        .sum();

    memo.insert(k, n);

    n
}

fn part2(input: String) -> TaskResult {
    let (network, trans) = parse_input::<2>(&input);

    let svr = trans["svr"];
    let dac = trans["dac"];
    let fft = trans["fft"];
    let out = trans["out"];

    let mut memo = HashMap::new();

    count_paths2(&mut memo, &network, (svr, [false; 2]), [dac, fft], out).into()
}
