use std::{fs, str::FromStr};

use anyhow::{bail, Error, Result};

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn part1() -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    let calibrations = input.lines().map(|s| Calibration::from_str(s).unwrap());
    calibrations.map(|cal| cal.sum_valid_p1()).sum()
}

fn part2() -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    let calibrations = input.lines().map(|s| Calibration::from_str(s).unwrap());
    calibrations.map(|cal| cal.sum_valid_p2()).sum()
}

#[derive(Debug)]
struct Calibration {
    target_value: usize,
    operands: Vec<usize>,
}

impl Calibration {
    // Return true if value and operands could form a valid equation.
    fn sum_valid_p1(&self) -> usize {
        // The approach here is to use the bits of a range of numbers to
        // pick the operator between any two numbers.
        // For example, if I have the Calibration 3267: 81 40 27, I
        // now that I need to try the following:
        // 81 + 40 + 27
        // 81 + 40 * 27
        // 81 * 40 + 27
        // 81 * 40 * 27
        //
        // This looks like an increasing bit pattern, with + == 0 and * == 1.
        // (0, 0) = 0
        // (0, 1) = 1
        // (1, 0) = 2
        // (1, 1) = 3
        //
        // Instead of walking the bits, I just look at the low bit (with & 1) and
        // right shift the number to move the next highest bit into that position
        // on each iteration.
        //
        // The number of bits I need is:
        // 2 numbers = 1 bit
        // 3 numbers = 2 bits
        // 4 numbers = 3 bits
        // etc...
        let mut total = 0;
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

    fn sum_valid_p2(&self) -> usize {
        // The approach here is very similar to sum_valid_p1 but
        // we're using ternary (base 3) math instead of base 2,
        // which means we don't get to play the same trick of using
        // incrementing numbers to create our bit pattern. Instead
        // I create an enum with 3 valid states, and implement an iterator
        // over the "trits" (ternary bits) until all perumutations are
        // exhausted.
        let mut total = 0;
        let ntrits = self.operands.len() - 1;
        let mut operations = OpPerumutations::new(ntrits);
        'outer: loop {
            let mut operands = self.operands.iter().copied();
            let mut value = operands.next().unwrap();
            for operand in operands {
                let Some(operation) = operations.next() else {
                    break 'outer;
                };
                let next_value = match operation {
                    Operator::Add => value + operand,
                    Operator::Multiply => value * operand,
                    Operator::Concat => {
                        let mut s = value.to_string();
                        s.push_str(&operand.to_string());
                        s.parse().expect("concatted number")
                    }
                };
                //println!("{value} {operation:#?} {operand} = {next_value}");
                value = next_value;
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

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

struct OpPerumutations {
    ops: Vec<Operator>,
    cur: usize,
    max_cur: usize,
    carry: bool,
}

impl OpPerumutations {
    fn new(n: usize) -> Self {
        Self {
            ops: vec![Operator::Add; n],
            cur: 0,
            max_cur: 3usize.pow(n as u32) * n,
            carry: false,
        }
    }
}

impl Iterator for OpPerumutations {
    type Item = Operator;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.max_cur {
            return None;
        }
        let cur = self.cur % self.ops.len();
        let op = self.ops[cur];
        if cur == 0 || self.carry {
            self.carry = match op {
                Operator::Add => {
                    self.ops[cur] = Operator::Multiply;
                    false
                }
                Operator::Multiply => {
                    self.ops[cur] = Operator::Concat;
                    false
                }
                Operator::Concat => {
                    self.ops[cur] = Operator::Add;
                    true
                }
            };
        }
        self.cur += 1;
        Some(op)
    }
}
