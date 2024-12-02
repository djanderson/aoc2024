#![feature(iter_map_windows)]

use std::{fs, str::FromStr};

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

#[derive(Debug)]
struct Report {
    levels: Vec<u32>,
}

impl FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let res: Result<Vec<_>, _> = s.split_ascii_whitespace().map(|n| n.parse()).collect();
        Ok(Self { levels: res? })
    }
}

impl Report {
    // Report is safe is all levels are monotonic, unique, and
    // at most 3 apart.
    fn is_safe(&self) -> bool {
        let is_monotonic = self.levels.is_sorted() || self.levels.iter().rev().is_sorted();
        let is_unique = self
            .levels
            .iter()
            .map_windows(|&[x, y]| x == y)
            .all(|same| !same);
        let max_distance = self
            .levels
            .iter()
            .map_windows(|&[x, y]| x.abs_diff(*y))
            .max()
            .expect("max distance");
        is_monotonic && is_unique && max_distance <= 3
    }

    // Report is safe with dampner if the report is safe with
    // at most one level removed.
    fn is_safe_with_dampner(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        // Inefficient, but good enough for the input data.
        self.levels.iter().enumerate().any(|(i, _)| {
            let dampened = self.dampen_level(i);
            dampened.is_safe()
        })
    }

    // Returns a copy of this Report with level `n` removed.
    //
    // Panics if `n` is not a level in `self`.
    fn dampen_level(&self, n: usize) -> Self {
        let mut dampened = self.levels.clone();
        dampened.remove(n);
        Self { levels: dampened }
    }
}

fn part1() -> u32 {
    let input = fs::read_to_string("input.txt").unwrap();
    input
        .lines()
        .map(|line| line.parse::<Report>().unwrap())
        .filter(|r| r.is_safe())
        .count() as u32
}

fn part2() -> u32 {
    let input = fs::read_to_string("input.txt").unwrap();
    input
        .lines()
        .map(|line| line.parse::<Report>().unwrap())
        .filter(|r| r.is_safe_with_dampner())
        .count() as u32
}
