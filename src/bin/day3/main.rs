use std::fs;

use regex::Regex;

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn part1() -> i32 {
    let input = fs::read_to_string("input.txt").unwrap();
    let re = Regex::new(r"mul\((?<d1>[0-9]{1,3}),(?<d2>[0-9]{1,3})\)").unwrap();
    re.captures_iter(&input)
        .map(|caps| {
            let d1: i32 = caps.name("d1").unwrap().as_str().parse().unwrap();
            let d2: i32 = caps.name("d2").unwrap().as_str().parse().unwrap();
            d1 * d2
        })
        .sum()
}

fn part2() -> i32 {
    let input = fs::read_to_string("input.txt").unwrap();
    let re =
        Regex::new(r"mul\((?<d1>[0-9]{1,3}),(?<d2>[0-9]{1,3})\)|(?<cond>do(n't)?\(\))").unwrap();
    re.captures_iter(&input)
        .scan(true, |enabled, captures| {
            if let Some(condition) = captures.name("cond") {
                match condition.as_str() {
                    "do()" => *enabled = true,
                    "don't()" => *enabled = false,
                    _ => unreachable!(),
                }
                Some(0)
            } else if *enabled {
                let d1: i32 = captures.name("d1").unwrap().as_str().parse().unwrap();
                let d2: i32 = captures.name("d2").unwrap().as_str().parse().unwrap();
                Some(d1 * d2)
            } else {
                Some(0)
            }
        })
        .sum()
}
