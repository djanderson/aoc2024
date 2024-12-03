use std::fs;

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn part1() -> i32 {
    let input = fs::read_to_string("example.txt").unwrap();
    input.lines().count() as i32
}

fn part2() -> i32 {
    let input = fs::read_to_string("example.txt").unwrap();
    input.lines().count() as i32
}
