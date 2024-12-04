use std::fs;

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn part1() -> i32 {
    let input = fs::read_to_string("input.txt").unwrap();
    // Create a vector of vectors of characters.
    let p: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let nrows = p.len();
    let ncols = p.first().unwrap().len();

    let mut count = 0;

    // Let i be the row index and j be the column index.
    for i in 0..nrows {
        for j in 0..ncols {
            if p[i][j] != 'X' {
                continue;
            }
            // Found an X. Search the clock, starting at 12 o'clock,
            // then 1:30, 3, 4:30, 6, 7:30, 9, and lastly 10:30.
            // I will call these positions u (up), r (right), l (left),
            // d (down) and ur, dr, dl, ul for the 4 diagonals. The name
            // is the direction the clock arrow is pointing.
            //
            // First, eliminate search paths if they are too close to an edge.
            let u = i >= 3;
            let r = j < ncols - 3;
            let d = i < nrows - 3;
            let l = j >= 3;
            let ur = u && r;
            let ul = u && l;
            let dr = d && r;
            let dl = d && l;

            // Look up: dec i
            if u && p[i - 1][j] == 'M' && p[i - 2][j] == 'A' && p[i - 3][j] == 'S' {
                count += 1;
            }
            // Look up and right: dec i, inc j
            if ur && p[i - 1][j + 1] == 'M' && p[i - 2][j + 2] == 'A' && p[i - 3][j + 3] == 'S' {
                count += 1;
            }
            // Look right: inc j
            if r && p[i][j + 1] == 'M' && p[i][j + 2] == 'A' && p[i][j + 3] == 'S' {
                count += 1;
            }
            // Look down and right: inc i, inc j
            if dr && p[i + 1][j + 1] == 'M' && p[i + 2][j + 2] == 'A' && p[i + 3][j + 3] == 'S' {
                count += 1;
            }
            // Look down: inc i
            if d && p[i + 1][j] == 'M' && p[i + 2][j] == 'A' && p[i + 3][j] == 'S' {
                count += 1;
            }
            // Look down and left: inc i, dec j
            if dl && p[i + 1][j - 1] == 'M' && p[i + 2][j - 2] == 'A' && p[i + 3][j - 3] == 'S' {
                count += 1;
            }
            // Look left: dec j
            if l && p[i][j - 1] == 'M' && p[i][j - 2] == 'A' && p[i][j - 3] == 'S' {
                count += 1;
            }
            // Look up and left: dec i, dec j
            if ul && p[i - 1][j - 1] == 'M' && p[i - 2][j - 2] == 'A' && p[i - 3][j - 3] == 'S' {
                count += 1;
            }
        }
    }

    count
}

fn part2() -> i32 {
    let input = fs::read_to_string("input.txt").unwrap();

    // Create a vector of vectors of characters.
    let p: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let nrows = p.len();
    let ncols = p.first().unwrap().len();

    let mut count = 0;

    // Let i be the row index and j be the column index.
    for i in 1..nrows - 1 {
        for j in 1..ncols - 1 {
            if p[i][j] != 'A' {
                continue;
            }

            // Found an A.
            let ul = p[i - 1][j - 1];
            let ur = p[i - 1][j + 1];
            let dl = p[i + 1][j - 1];
            let dr = p[i + 1][j + 1];

            let mas_one = (ul == 'M' && dr == 'S') || (ul == 'S' && dr == 'M');
            let mas_two = (ur == 'M' && dl == 'S') || (ur == 'S' && dl == 'M');

            if mas_one && mas_two {
                count += 1;
            }
        }
    }

    count
}
