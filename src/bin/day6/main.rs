use std::fs;

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn part1() -> i32 {
    let input = fs::read_to_string("example.txt").unwrap();
    let total_cols = input.lines().next().unwrap().len();
    let total_rows = input.lines().count();

    // Build the maps and initialize guard.
    let mut obstructions: PositionMap<bool> = PositionMap::new(total_rows, total_cols);
    let mut visited_positions: PositionMap<bool> = PositionMap::new(total_rows, total_cols);
    let mut guard = {
        let mut guard_row = 0;
        let mut guard_col = 0;
        for (col, line) in input.lines().enumerate() {
            for (row, char) in line.char_indices() {
                if char == '#' {
                    obstructions.set(row, col, true);
                } else if char == '^' {
                    guard_row = row;
                    guard_col = col;
                    visited_positions.set(row, col, true);
                }
            }
        }
        Guard::new(guard_row, guard_col)
    };

    // Run the simulation.
    loop {
        let Some((next_row, next_col)) = guard.next() else {
            // Leaving area up or to left.
            break;
        };
        if next_row == total_rows || next_col == total_cols {
            // Leaving area down or to right.
            break;
        }
        if *obstructions.get(next_row, next_col) {
            guard.turn();
        } else {
            visited_positions.set(next_row, next_col, true);
            guard.move_forward();
        }
    }

    visited_positions.count() as i32
}

fn part2() -> i32 {
    let input = fs::read_to_string("input.txt").unwrap();
    let total_cols = input.lines().next().unwrap().len();
    let total_rows = input.lines().count();

    // Build the maps and initialize guard.
    let mut initial_obstructions: PositionMap<bool> = PositionMap::new(total_rows, total_cols);
    let mut initial_visited_positions: PositionMap<Option<Direction>> =
        PositionMap::new(total_rows, total_cols);
    let initial_guard = {
        let mut guard_row = 0;
        let mut guard_col = 0;
        for (col, line) in input.lines().enumerate() {
            for (row, char) in line.char_indices() {
                if char == '#' {
                    initial_obstructions.set(row, col, true);
                } else if char == '^' {
                    guard_row = row;
                    guard_col = col;
                    initial_visited_positions.set(row, col, Some(Direction::Up));
                }
            }
        }
        Guard::new(guard_row, guard_col)
    };

    let mut permutations = 0;

    for col in 0..total_cols {
        for row in 0..total_rows {
            let on_guards_current_spot = row == initial_guard.row && col == initial_guard.col;
            if on_guards_current_spot || *initial_obstructions.get(row, col) {
                continue;
            }

            let mut guard = initial_guard.clone();
            let mut visited_positions = initial_visited_positions.clone();
            let mut obstructions = initial_obstructions.clone();
            obstructions.set(row, col, true);

            // Run the simulation.
            loop {
                let Some((next_row, next_col)) = guard.next() else {
                    // Leaving area up or to left.
                    break;
                };
                if next_row == total_rows || next_col == total_cols {
                    // Leaving area down or to right.
                    break;
                }
                if let Some(previous_direction) = visited_positions.get(next_row, next_col) {
                    if *previous_direction == guard.direction {
                        // We've created a loop for the guard;
                        permutations += 1;
                        break;
                    }
                }
                if *obstructions.get(next_row, next_col) {
                    guard.turn();
                } else {
                    visited_positions.set(next_row, next_col, Some(guard.direction));
                    guard.move_forward();
                }
            }
        }
    }

    permutations
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
struct Guard {
    pub row: usize,
    pub col: usize,
    pub direction: Direction,
}

impl Guard {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
            direction: Direction::Up,
        }
    }

    fn next(&self) -> Option<(usize, usize)> {
        match self.direction {
            Direction::Up => {
                if self.col == 0 {
                    None
                } else {
                    Some((self.row, self.col - 1))
                }
            }
            Direction::Right => Some((self.row + 1, self.col)),
            Direction::Down => Some((self.row, self.col + 1)),
            Direction::Left => {
                if self.row == 0 {
                    None
                } else {
                    Some((self.row - 1, self.col))
                }
            }
        }
    }

    fn move_forward(&mut self) {
        let (row, col) = self.next().expect("move should be valid");
        self.row = row;
        self.col = col;
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
struct PositionMap<T: Clone + Default + PartialEq> {
    positions: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> PositionMap<T>
where
    T: Clone + Default + PartialEq,
{
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            positions: vec![T::default(); rows * cols],
            rows,
            cols,
        }
    }

    // Set a position in the map as visited/occupied.
    fn set(&mut self, row: usize, col: usize, val: T) {
        self.positions[row * self.cols + col] = val;
    }

    // Return a reference to the value at the requested position.
    fn get(&self, row: usize, col: usize) -> &T {
        &self.positions[row * self.cols + col]
    }

    // Count all visited/occupied positions in the map.
    fn count(&self) -> usize {
        let default = T::default();
        self.positions.iter().filter(|&pos| *pos != default).count()
    }
}
