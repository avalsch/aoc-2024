#!/usr/bin/env cargo

---
package.edition = "2021"

[dependencies]
itertools = "0.13"
---

use itertools::Itertools as _;

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn input() -> (Vec<usize>, Vec<usize>) {
    std::fs::read_to_string("day1.input")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1]))
        .unzip()
}

fn part1() -> usize {
    let (mut left, mut right) = input();

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| usize::abs_diff(l, r))
        .sum()
}

fn part2() -> usize {
    let (inputs, counts) = input();

    let counts = counts.into_iter().counts();

    inputs
        .into_iter()
        .map(|n| n * counts.get(&n).copied().unwrap_or_default())
        .sum()
}
