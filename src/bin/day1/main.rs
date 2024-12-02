use std::{collections::HashMap, fs};

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}

fn part1() -> u32 {
    let input = fs::read_to_string("input.txt").unwrap();
    let (mut v1, mut v2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .into_iter()
        .map(|line| {
            let mut numbers = line.split_ascii_whitespace();
            let num1: i32 = numbers.next().unwrap().parse().expect("number");
            let num2: i32 = numbers.next().unwrap().parse().expect("number");
            (num1, num2)
        })
        .unzip();
    v1.sort_unstable();
    v2.sort_unstable();
    v1.into_iter().zip(v2).map(|(n1, n2)| n1.abs_diff(n2)).sum()
}

fn part2() -> u32 {
    let input = fs::read_to_string("input.txt").unwrap();
    let (nums1, nums2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .into_iter()
        .map(|line| {
            let mut numbers = line.split_ascii_whitespace();
            let num1: u32 = numbers.next().unwrap().parse().expect("number");
            let num2: u32 = numbers.next().unwrap().parse().expect("number");
            (num1, num2)
        })
        .unzip();
    let mut occurances: HashMap<u32, u32> = HashMap::new();
    for n in nums2 {
        occurances
            .entry(n)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let similarity_score = nums1
        .into_iter()
        .map(|n| n * *occurances.entry(n).or_default())
        .sum();
    similarity_score
}
