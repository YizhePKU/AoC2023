use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Beam {
    pos: (usize, usize),
    dir: Direction,
}

fn get_energy(cave: &Vec<Vec<char>>, init: Beam) -> usize {
    let n = cave.len();
    let m = cave[0].len();

    let update = |(x, y): (usize, usize), dir: Direction| -> Option<(usize, usize)> {
        match dir {
            Direction::Up if x > 0 => Some((x - 1, y)),
            Direction::Down if x < n - 1 => Some((x + 1, y)),
            Direction::Left if y > 0 => Some((x, y - 1)),
            Direction::Right if y < m - 1 => Some((x, y + 1)),
            _ => None,
        }
    };

    let mut todo = vec![init];
    let mut vis = vec![vec![HashSet::new(); m]; n];
    while let Some(beam) = todo.pop() {
        let Beam { pos, dir } = beam;
        if !vis[pos.0][pos.1].contains(&dir) {
            vis[pos.0][pos.1].insert(dir);

            let dirs: &[Direction] = if cave[pos.0][pos.1] == '.' {
                &[dir]
            } else if cave[pos.0][pos.1] == '/' {
                match dir {
                    Direction::Up => &[Direction::Right],
                    Direction::Down => &[Direction::Left],
                    Direction::Left => &[Direction::Down],
                    Direction::Right => &[Direction::Up],
                }
            } else if cave[pos.0][pos.1] == '\\' {
                match dir {
                    Direction::Up => &[Direction::Left],
                    Direction::Down => &[Direction::Right],
                    Direction::Left => &[Direction::Up],
                    Direction::Right => &[Direction::Down],
                }
            } else if cave[pos.0][pos.1] == '|' {
                match dir {
                    Direction::Up => &[Direction::Up],
                    Direction::Down => &[Direction::Down],
                    Direction::Left => &[Direction::Up, Direction::Down],
                    Direction::Right => &[Direction::Up, Direction::Down],
                }
            } else if cave[pos.0][pos.1] == '-' {
                match dir {
                    Direction::Up => &[Direction::Left, Direction::Right],
                    Direction::Down => &[Direction::Left, Direction::Right],
                    Direction::Left => &[Direction::Left],
                    Direction::Right => &[Direction::Right],
                }
            } else {
                unreachable!();
            };

            for &next_dir in dirs {
                if let Some(next_pos) = update(pos, next_dir) {
                    todo.push(Beam {
                        pos: next_pos,
                        dir: next_dir,
                    });
                }
            }
        }
    }

    let mut energized = 0;
    for i in 0..n {
        for j in 0..m {
            if !vis[i][j].is_empty() {
                energized += 1;
            }
        }
    }
    energized
}

fn main() {
    let input = std::fs::read("data/day16").unwrap();
    let input = String::from_utf8(input).unwrap();

    let cave = input
        .split_terminator("\r\n")
        .map(|s| s.chars().collect_vec())
        .collect_vec();
    let n = cave.len();
    let m = cave[0].len();

    let init = Beam {
        pos: (0, 0),
        dir: Direction::Right,
    };
    let energized = get_energy(&cave, init);
    println!("energized = {energized}");

    let mut max_energized = 0;
    // down
    for j in 0..m {
        let init = Beam {
            pos: (0, j),
            dir: Direction::Down,
        };
        let energized = get_energy(&cave, init);
        max_energized = max_energized.max(energized);
    }
    // up
    for j in 0..m {
        let init = Beam {
            pos: (n - 1, j),
            dir: Direction::Up,
        };
        let energized = get_energy(&cave, init);
        max_energized = max_energized.max(energized);
    }
    // right
    for i in 0..n {
        let init = Beam {
            pos: (i, 0),
            dir: Direction::Right,
        };
        let energized = get_energy(&cave, init);
        max_energized = max_energized.max(energized);
    }
    // left
    for i in 0..n {
        let init = Beam {
            pos: (i, m - 1),
            dir: Direction::Left,
        };
        let energized = get_energy(&cave, init);
        max_energized = max_energized.max(energized);
    }
    println!("max_energized = {max_energized}");
}
