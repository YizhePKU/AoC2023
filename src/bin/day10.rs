use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read("data/day10").unwrap();
    let input = String::from_utf8(input).unwrap();

    // Connectivity constants.
    let up = (-1, 0);
    let down = (1, 0);
    let left = (0, -1);
    let right = (0, 1);
    let connectivity = HashMap::from([
        (b'|', HashSet::from([up, down])),
        (b'-', HashSet::from([left, right])),
        (b'L', HashSet::from([up, right])),
        (b'J', HashSet::from([up, left])),
        (b'7', HashSet::from([down, left])),
        (b'F', HashSet::from([down, right])),
        (b'.', HashSet::from([])),
    ]);

    let matrix = input
        .split_terminator("\r\n")
        .map(|s| s.bytes().collect_vec())
        .collect_vec();
    let n = matrix.len();
    let m = matrix[0].len();

    // Returns true if a point is in-bound.
    let valid = |i, j| i >= 0 && i < n as i32 && j >= 0 && j < m as i32;

    // Find the starting point.
    let mut start = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == b'S' {
                start = (i as i32, j as i32);
            }
        }
    }
    assert!(valid(start.0, start.1));

    // Pick a random starting direction.
    let mut route = vec![];
    let mut starting_symbol = b'.';
    'outer: for starting_dir in [up, down, left, right] {
        let mut cur = start;
        let mut dir = starting_dir;
        loop {
            // Move a step along the pipe.
            cur.0 += dir.0;
            cur.1 += dir.1;
            // Check if we went out of bounds.
            if !valid(cur.0, cur.1) {
                route.clear();
                continue 'outer;
            }
            // Check if we looped back to the starting point.
            let reverse_dir = (-dir.0, -dir.1);
            if cur == start {
                // Rewrite the starting symbol ('S') to the appropriate pipe symbol
                starting_symbol = *connectivity
                    .iter()
                    .filter(|&(_, v)| *v == HashSet::from([starting_dir, reverse_dir]))
                    .next()
                    .unwrap()
                    .0;
                break 'outer;
            }
            // Check if the new pipe is actually connected.
            let connections = &connectivity[&matrix[cur.0 as usize][cur.1 as usize]];
            if !connections.contains(&reverse_dir) {
                route.clear();
                continue 'outer;
            }
            // Record the new pipe as part of the route.
            route.push(cur);
            // Update the direction.
            dir = *connections
                .iter()
                .filter(|&&dir| dir != reverse_dir)
                .next()
                .unwrap();
        }
    }
    println!("steps = {}", (route.len() + 1) / 2);

    // Create a high-resolution version of the matrix.
    let mut matrix2 = vec![vec![b'.'; 3 * m]; 3 * n];
    for i in 0..n {
        for j in 0..m {
            let mut symbol = matrix[i][j];
            if symbol == b'S' {
                symbol = starting_symbol;
            }

            if symbol == b'|' {
                matrix2[i * 3 + 0][j * 3 + 1] = b'X';
                matrix2[i * 3 + 1][j * 3 + 1] = b'X';
                matrix2[i * 3 + 2][j * 3 + 1] = b'X';
            } else if symbol == b'-' {
                matrix2[i * 3 + 1][j * 3 + 0] = b'X';
                matrix2[i * 3 + 1][j * 3 + 1] = b'X';
                matrix2[i * 3 + 1][j * 3 + 2] = b'X';
            } else if symbol == b'L' {
                matrix2[i * 3 + 0][j * 3 + 1] = b'X';
                matrix2[i * 3 + 1][j * 3 + 1] = b'X';
                matrix2[i * 3 + 1][j * 3 + 2] = b'X';
            } else if symbol == b'J' {
                matrix2[i * 3 + 0][j * 3 + 1] = b'X';
                matrix2[i * 3 + 1][j * 3 + 0] = b'X';
                matrix2[i * 3 + 1][j * 3 + 1] = b'X';
            } else if symbol == b'7' {
                matrix2[i * 3 + 1][j * 3 + 0] = b'X';
                matrix2[i * 3 + 1][j * 3 + 1] = b'X';
                matrix2[i * 3 + 2][j * 3 + 1] = b'X';
            } else if symbol == b'F' {
                matrix2[i * 3 + 1][j * 3 + 1] = b'X';
                matrix2[i * 3 + 1][j * 3 + 2] = b'X';
                matrix2[i * 3 + 2][j * 3 + 1] = b'X';
            }
        }
    }

    // For the purpose of area-tracking, treat the starting point as part of the
    // route.
    route.push(start);

    let s = matrix2
        .iter()
        .map(|bytes| String::from_utf8(bytes.clone()).unwrap())
        .join("\n");
    println!("{s}");

    // Run floodfill on the high-resolution matrix.
    let mut vis = vec![vec![false; 3 * m]; 3 * n];
    let mut inner_area = HashSet::new();
    for i in 0..3 * n {
        for j in 0..3 * m {
            let mut todo = vec![(i, j)];
            let mut area = HashSet::new();
            let mut edge = false;
            while let Some((x, y)) = todo.pop() {
                if !vis[x][y]
                    && (matrix2[x][y] == b'.' || !route.contains(&(x as i32 / 3, y as i32 / 3)))
                {
                    vis[x][y] = true;
                    if x == 0 || x == 3 * n - 1 || y == 0 || y == 3 * m - 1 {
                        edge = true;
                    }
                    if !route.contains(&(x as i32 / 3, y as i32 / 3)) {
                        area.insert((x / 3, y / 3));
                    }

                    if x > 0 {
                        todo.push((x - 1, y));
                    }
                    if x < 3 * n - 1 {
                        todo.push((x + 1, y));
                    }
                    if y > 0 {
                        todo.push((x, y - 1));
                    }
                    if y < 3 * m - 1 {
                        todo.push((x, y + 1));
                    }
                }
            }
            if !edge {
                for point in area {
                    inner_area.insert(point);
                }
            }
        }
    }
    println!("inner_area = {}", inner_area.len());
}
