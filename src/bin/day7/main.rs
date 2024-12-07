use std::{fs, str::FromStr};

use anyhow::{bail, Error, Result};

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn part1() -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    let calibrations = input.lines().map(|s| Calibration::from_str(s).unwrap());
    calibrations.map(|cal| cal.sum_valid()).sum()
}

fn part2() -> u32 {
    let input = fs::read_to_string("input.txt").unwrap();
    input.lines().count() as u32
}

#[derive(Debug)]
struct Calibration {
    target_value: usize,
    operands: Vec<usize>,
}

impl Calibration {
    // Return true if value and operands could form a valid equation.
    fn sum_valid(&self) -> usize {
        let mut total = 0;
        // 2 numbers = 1 bit
        // 3 numbers = 2 bits
        // 4 numbers = 3 bits
        // etc...
        let nbits = self.operands.len() as u32 - 1;
        for i in 0..(2u32.pow(nbits)) {
            let mut n = i;
            let mut operands = self.operands.iter().copied();
            let mut value = operands.next().unwrap();
            for operand in operands {
                match n & 1 {
                    0 => value += operand,
                    1 => value *= operand,
                    _ => unreachable!(),
                }
                if value > self.target_value {
                    break;
                }
                n = n >> 1;
            }
            if value == self.target_value {
                total += value;
                break; // only add the number once if it's valid
            }
        }
        total
    }
}

impl FromStr for Calibration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let Some((rhs, lhs)) = s.split_once(':') else {
            bail!("expected colon");
        };
        let Ok(value) = rhs.parse() else {
            bail!("expected number: got {rhs}");
        };
        let operands = lhs
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>().expect("number"))
            .collect();
        Ok(Self {
            target_value: value,
            operands,
        })
    }
}
