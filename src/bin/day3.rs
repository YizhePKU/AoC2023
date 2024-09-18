use itertools::Itertools;

/// Scan the schematic for numbers. Numbers are represented as (row, column, length).
fn scan(schema: &Vec<Vec<u8>>) -> Vec<(usize, usize, usize)> {
    let mut result = vec![];
    for row in 0..schema.len() {
        let mut column = 0;
        loop {
            // Find the next number on this row.
            while column < schema[0].len() && !schema[row][column].is_ascii_digit() {
                column += 1;
            }
            // If none is found, goto the next row.
            if column >= schema[0].len() {
                break;
            }
            // Find the length of this number.
            let mut length = 0;
            while column < schema[0].len() && schema[row][column].is_ascii_digit() {
                length += 1;
                column += 1;
            }
            // Register this number.
            result.push((row, column - length, length));
        }
    }

    result
}

/// Check if a number is adjacent to a symbol.
fn adjacent_to_symbol(schema: &Vec<Vec<u8>>, number: (usize, usize, usize)) -> bool {
    // A symbol is any byte other than a digit or a dot.
    let is_symbol = |c: &u8| !c.is_ascii_digit() && *c != b'.';

    let (row, column, length) = number;

    let left = column.saturating_sub(1);
    let right = if column + length < schema[0].len() {
        column + length
    } else {
        schema[0].len() - 1
    };

    // Check the row above, unless it's the first row.
    if row > 0 && schema[row - 1][left..=right].iter().any(is_symbol) {
        return true;
    }
    // Check the row below, unless it's the last row.
    if row < schema.len() - 1 && schema[row + 1][left..=right].iter().any(is_symbol) {
        return true;
    }
    // Check the symbol on the left and right.
    if is_symbol(&schema[row][left]) || is_symbol(&schema[row][right]) {
        return true;
    }

    return false;
}

/// Check if a number is adjacent to a square.
fn adjacent_to(number: (usize, usize, usize), square: (usize, usize)) -> bool {
    let (row, column, length) = number;
    let (x, y) = square;

    let left = column.saturating_sub(1);
    let right = column + length;
    let up = row.saturating_sub(1);
    let down = row + 1;

    x >= up && x <= down && y >= left && y <= right
}

fn parse_number(schema: &Vec<Vec<u8>>, number: (usize, usize, usize)) -> u32 {
    let (row, column, length) = number;
    let s = String::from_utf8(schema[row][column..column + length].to_vec()).unwrap();
    s.parse().unwrap()
}

fn main() {
    let input = std::fs::read("data/day3").unwrap();
    let input = String::from_utf8(input).unwrap();

    let schema = input
        .split_terminator("\r\n")
        .map(|s| s.bytes().collect_vec())
        .collect_vec();

    let mut sum = 0;
    for number in scan(&schema) {
        if adjacent_to_symbol(&schema, number) {
            sum += parse_number(&schema, number);
        }
    }
    println!("sum = {sum}");

    // Find all gears in the schema.
    let numbers = scan(&schema);
    let mut ratio_sum = 0;
    for i in 0..schema.len() {
        for j in 0..schema[0].len() {
            if schema[i][j] == b'*' {
                // Find all numbers that are adjacent to the gear.
                let adj = numbers
                    .iter()
                    .filter(|&&num| adjacent_to(num, (i, j)))
                    .copied()
                    .collect_vec();
                // It's only a gear if it has exactly two adjacent numbers.
                if adj.len() == 2 {
                    let gear_ratio = parse_number(&schema, adj[0]) * parse_number(&schema, adj[1]);
                    ratio_sum += gear_ratio;
                }
            }
        }
    }
    println!("ratio_sum = {ratio_sum}");
}
