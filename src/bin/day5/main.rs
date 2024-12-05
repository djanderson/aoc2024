use anyhow::{Error, Result};
use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn part1() -> i32 {
    let input = fs::read_to_string("input.txt").unwrap();
    let (page_ordering_rules, pages_to_update) = input.split_once("\n\n").unwrap();
    let mut ordering_rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for rule in page_ordering_rules.lines() {
        let (s1, s2) = rule.split_once('|').unwrap();
        let n1 = s1.parse().expect("number");
        let n2 = s2.parse().expect("number");
        ordering_rules
            .entry(n1)
            .and_modify(|set| _ = set.insert(n2))
            .or_insert(HashSet::from([n2]));
    }
    let updates = pages_to_update
        .lines()
        .map(|line| Update::from_str(line).unwrap());
    let mut sum = 0;
    let empty_hashset = HashSet::new();
    for update in updates {
        let pages = update.pages;
        let mut correct = true;
        for (i, page) in pages.iter().enumerate() {
            let earlier_pages: HashSet<i32> = HashSet::from_iter(pages[..i].iter().cloned());
            let later_pages = ordering_rules.get(page).unwrap_or(&empty_hashset);
            if !earlier_pages.is_disjoint(later_pages) {
                correct = false;
                break;
            }
        }
        if correct {
            let middle_page = pages[pages.len() / 2];
            sum += middle_page;
        }
    }

    sum
}

fn part2() -> i32 {
    let input = fs::read_to_string("example.txt").unwrap();
    input.lines().count() as i32
}

struct Update {
    pages: Vec<i32>,
}

impl FromStr for Update {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let pages = s
            .split(',')
            .map(|n| n.parse::<i32>().expect("number"))
            .collect();

        Ok(Self { pages })
    }
}
