#!/usr/bin/env cargo

---
package.edition = "2021"

[dependencies]
regex = "*"
---

use regex::Regex;

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn input() -> String {
    std::fs::read_to_string("day3.input").unwrap()
}

fn part1() -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(&input())
        .map(|c| c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap())
        .sum::<u32>()
}

fn part2() -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;

    re.captures_iter(&input())
        .filter_map(|c| match &c[0] {
            "do()" => { enabled = true; None }
            "don't()" => { enabled = false; None }
            _ if enabled => Some(c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap()),
            _ => None,
        })
        .sum()
}
