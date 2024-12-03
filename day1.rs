#!/usr/bin/env cargo

---
package.edition = "2021"

[dependencies]
itertools = "*"
---

use itertools::Itertools as _;

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn input() -> (Vec<u32>, Vec<u32>) {
    std::fs::read_to_string("day1.input")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1]))
        .unzip()
}

fn part1() -> u32 {
    let (mut left, mut right) = input();

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| u32::abs_diff(l, r))
        .sum()
}

fn part2() -> u32 {
    let (inputs, counts) = input();

    let counts = counts.into_iter().counts();

    inputs
        .into_iter()
        .map(|n| n * counts.get(&n).copied().unwrap_or_default() as u32)
        .sum()
}
