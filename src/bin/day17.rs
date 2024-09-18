use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    pos: (usize, usize),
    dir: Direction,
    steps: u8,
}

fn main() {
    let input = std::fs::read("data/day17").unwrap();
    let input = String::from_utf8(input).unwrap();

    let chill = input
        .split_terminator("\r\n")
        .map(|s| s.bytes().map(|b| (b - b'0') as u32).collect_vec())
        .collect_vec();

    let n = chill.len();
    let m = chill[0].len();

    let step = |(x, y): (usize, usize), dir: Direction| -> Option<(usize, usize)> {
        match dir {
            Direction::Up if x > 0 => Some((x - 1, y)),
            Direction::Down if x < n - 1 => Some((x + 1, y)),
            Direction::Left if y > 0 => Some((x, y - 1)),
            Direction::Right if y < m - 1 => Some((x, y + 1)),
            _ => None,
        }
    };

    let turn = |dir: Direction| -> [Direction; 2] {
        match dir {
            Direction::Up => [Direction::Left, Direction::Right],
            Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left => [Direction::Up, Direction::Down],
            Direction::Right => [Direction::Up, Direction::Down],
        }
    };

    // Run Dijkstra's algorithm.
    let mut visited: HashMap<Node, u32> = HashMap::new();
    let mut unvisited: PriorityQueue<Node, Reverse<u32>> = PriorityQueue::new();
    unvisited.push(
        Node {
            pos: (0, 0),
            dir: Direction::Right,
            steps: 0,
        },
        Reverse(0),
    );
    unvisited.push(
        Node {
            pos: (0, 0),
            dir: Direction::Down,
            steps: 0,
        },
        Reverse(0),
    );

    let mut answer = None;
    while let Some((node, Reverse(cost))) = unvisited.pop() {
        visited.insert(node, cost);

        let Node { pos, dir, steps } = node;
        if pos == (n - 1, m - 1) {
            answer = Some(cost);
            break;
        }

        // keep going if we still can
        if steps < 3 {
            if let Some(new_pos) = step(pos, dir) {
                let new_node = Node {
                    pos: new_pos,
                    dir,
                    steps: steps + 1,
                };
                if !visited.contains_key(&new_node) {
                    let new_cost = cost + chill[new_pos.0][new_pos.1];
                    unvisited.push_increase(new_node, Reverse(new_cost));
                }
            }
        }

        // or turn left/right, resetting steps to 1
        for new_dir in turn(dir) {
            if let Some(new_pos) = step(pos, new_dir) {
                let new_node = Node {
                    pos: new_pos,
                    dir: new_dir,
                    steps: 1,
                };
                if !visited.contains_key(&new_node) {
                    let new_cost = cost + chill[new_pos.0][new_pos.1];
                    unvisited.push_increase(new_node, Reverse(new_cost));
                }
            }
        }
    }
    println!("answer = {}", answer.unwrap());

    // Do it again, using updated ruleset.
    let mut visited: HashMap<Node, u32> = HashMap::new();
    let mut unvisited: PriorityQueue<Node, Reverse<u32>> = PriorityQueue::new();
    unvisited.push(
        Node {
            pos: (0, 0),
            dir: Direction::Right,
            steps: 0,
        },
        Reverse(0),
    );
    unvisited.push(
        Node {
            pos: (0, 0),
            dir: Direction::Down,
            steps: 0,
        },
        Reverse(0),
    );

    let mut answer2 = None;
    while let Some((node, Reverse(cost))) = unvisited.pop() {
        visited.insert(node, cost);

        let Node { pos, dir, steps } = node;
        if pos == (n - 1, m - 1) && steps >= 4 {
            answer2 = Some(cost);
            break;
        }

        // keep going if we can
        if steps < 10 {
            if let Some(new_pos) = step(pos, dir) {
                let new_node = Node {
                    pos: new_pos,
                    dir,
                    steps: steps + 1,
                };
                if !visited.contains_key(&new_node) {
                    let new_cost = cost + chill[new_pos.0][new_pos.1];
                    unvisited.push_increase(new_node, Reverse(new_cost));
                }
            }
        }

        // or turn left/right (if we can), resetting steps to 1
        if steps >= 4 {
            for new_dir in turn(dir) {
                if let Some(new_pos) = step(pos, new_dir) {
                    let new_node = Node {
                        pos: new_pos,
                        dir: new_dir,
                        steps: 1,
                    };
                    if !visited.contains_key(&new_node) {
                        let new_cost = cost + chill[new_pos.0][new_pos.1];
                        unvisited.push_increase(new_node, Reverse(new_cost));
                    }
                }
            }
        }
    }
    println!("answer2 = {}", answer2.unwrap());
}
