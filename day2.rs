#!/usr/bin/env cargo

---
package.edition = "2021"

[dependencies]
itertools = "*"
---

#![feature(array_windows)]

use itertools::Itertools as _;

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn input() -> impl Iterator<Item = Vec<usize>> {
    // deal with lifetime issues
    let input = std::fs::read_to_string("day2.input").unwrap();
    let input = input.lines().map(str::to_owned).collect::<Vec<_>>();

    input.into_iter().map(|line| {
        line.split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    })
}

fn part1() -> usize {
    input().filter(|l| is_safe(&l)).count()
}

fn is_safe(line: &[usize]) -> bool {
    let cmp = cmp(&line);

    line.array_windows().all(|&[l, r]| pair_valid(l, r, &cmp))
}

fn part2() -> usize {
    // simple brute force solution using Itertools::combinations
    // and part one `is_safe`
    input().filter(|line| is_safe_p2(line)).count()

    // O(n) solution, we use filter_map to have ownership
    // input().filter_map(|line| is_safe_p2_on(line).then_some(())).count()
}

// is_safe for part 2 using Itertools::combinations
// simpler and works well for small line lengths,
// but has worse time complexity
#[allow(dead_code)]
fn is_safe_p2(line: &[usize]) -> bool {
    is_safe(&line)
        || line
            .iter()
            .copied()
            .combinations(line.len() - 1)
            .any(|line| is_safe(&line))
}

/// is_safe for part 2 in O(n) time complexity
/// (at the cost of readability & simplicity)
#[allow(dead_code)]
fn is_safe_p2_on(mut line: Vec<usize>) -> bool {
    let cmp = cmp(&line);

    let mut idx = 0;
    let mut err = false;

    for &[l, r] in line.array_windows() {
        let valid = pair_valid(l, r, &cmp);

        if !valid && !err {
            err = true;
        } else if err {
            idx -= valid as usize;
            break;
        }

        idx += 1;
    }

    if err {
        line.remove(idx);
        line.array_windows().all(|&[l, r]| pair_valid(l, r, &cmp))
    } else {
        true
    }
}

/// determine the compare function to use for a line based on
/// which equality occurs more often
fn cmp(line: &[usize]) -> impl Fn(usize, usize) -> bool {
    if line.array_windows().filter(|&[l, r]| l > r).count() > line.len() / 2 {
        |l, r| l > r
    } else {
        |l, r| l < r
    }
}

fn pair_valid(l: usize, r: usize, cmp: impl Fn(usize, usize) -> bool) -> bool {
    cmp(l, r) && matches!(usize::abs_diff(l, r), 1..=3)
}
