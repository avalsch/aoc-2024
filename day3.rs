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
    let mut sum = 0;

    for c in re.captures_iter(&input()) {
        match &c[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if enabled => sum += c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap(),
            _ => (),
        }
    }

    sum
}
