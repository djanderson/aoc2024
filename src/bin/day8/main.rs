use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn part1() -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    let max_col = input.lines().next().unwrap().len();
    let max_row = input.lines().count();
    // Build antenna location map.
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    for (col, line) in input.lines().enumerate() {
        for (row, frequency) in line.chars().enumerate() {
            if frequency != '.' {
                let point = Point { col, row };
                antennas
                    .entry(frequency)
                    .and_modify(|points| points.push(point.clone()))
                    .or_insert(vec![point]);
            }
        }
    }
    for v in antennas.values_mut() {
        v.sort_unstable();
    }
    // Build antinode location map.
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (frequency, locations) in antennas.iter() {
        // Use 2 cursors to get all combinations of 2 points for this antenna.
        for i in 0..locations.len() - 1 {
            for j in i + 1..locations.len() {
                let p1 = &locations[i];
                let p2 = &locations[j];
                if let Some(antinode) = p1.antinode(p2).filter(|p| {
                    // Check that p1's antinode is within map boundaries
                    (p.row < max_row && p.col < max_col)
                        // and is not on top of an antenna of the same frequency
                        && antennas.get(&frequency).unwrap().binary_search(p).is_err()
                }) {
                    antinodes.insert(antinode);
                }
                if let Some(antinode) = p2.antinode(p1).filter(|p| {
                    // Check that p2's antinode is within map boundaries
                    (p.row < max_row && p.col < max_col)
                        // and is not on top of an antenna of the same frequency
                        && antennas.get(&frequency).unwrap().binary_search(p).is_err()
                }) {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.len()
}

fn part2() -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    let max_col = input.lines().next().unwrap().len();
    let max_row = input.lines().count();
    // Build antenna location map.
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    for (col, line) in input.lines().enumerate() {
        for (row, frequency) in line.chars().enumerate() {
            if frequency != '.' {
                let point = Point { col, row };
                antennas
                    .entry(frequency)
                    .and_modify(|points| points.push(point.clone()))
                    .or_insert(vec![point]);
            }
        }
    }
    for v in antennas.values_mut() {
        v.sort_unstable();
    }
    // Build antinode location map.
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (_frequency, locations) in antennas.iter() {
        // Use 2 cursors to get all combinations of 2 points for this antenna.
        for i in 0..locations.len() - 1 {
            for j in i + 1..locations.len() {
                let p1 = &locations[i];
                let p2 = &locations[j];
                let new_antinodes = p1.antinode_with_harmonics(p2, max_col, max_row);
                antinodes.extend(new_antinodes);
            }
        }
    }
    antinodes.len()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    pub col: usize,
    pub row: usize,
}

impl Point {
    fn antinode(&self, other: &Self) -> Option<Self> {
        let row_diff = self.row as isize - other.row as isize;
        let col_diff = self.col as isize - other.col as isize;
        let row = self.row as isize + row_diff;
        let col = self.col as isize + col_diff;
        if row < 0 || col < 0 {
            None
        } else {
            Some(Point {
                row: row as usize,
                col: col as usize,
            })
        }
    }

    fn antinode_with_harmonics(
        &self,
        other: &Self,
        max_col: usize,
        max_row: usize,
    ) -> HashSet<Point> {
        let mut result = HashSet::from([self.clone(), other.clone()]);
        let this_row = self.row as isize;
        let this_col = self.col as isize;
        let other_row = other.row as isize;
        let other_col = other.col as isize;
        let row_diff = this_row - other_row;
        let col_diff = this_col - other_col;
        // Project from self
        let mut row = this_row;
        let mut col = this_col;
        loop {
            row += row_diff;
            if row < 0 || row >= max_row as isize {
                break;
            }
            col += col_diff;
            if col < 0 || col >= max_col as isize {
                break;
            }
            result.insert(Point {
                col: col as usize,
                row: row as usize,
            });
        }
        // Project from other
        let mut row = other_row;
        let mut col = other_col;
        loop {
            row -= row_diff;
            if row < 0 || row >= max_row as isize {
                break;
            }
            col -= col_diff;
            if col < 0 || col >= max_col as isize {
                break;
            }
            result.insert(Point {
                col: col as usize,
                row: row as usize,
            });
        }
        result
    }
}
