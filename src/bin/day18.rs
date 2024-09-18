#![feature(is_sorted)]

use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Find the minimal bounding box that encloses all points. Returns
// (x_min, x_max, y_min, y_max).
fn bounding_box<'a, T>(points: T) -> (i32, i32, i32, i32)
where
    T: IntoIterator<Item = &'a (i32, i32)>,
{
    let mut x_min = i32::MAX;
    let mut x_max = 0;
    let mut y_min = i32::MAX;
    let mut y_max = 0;
    for &(x, y) in points {
        x_min = x_min.min(x);
        x_max = x_max.max(x);
        y_min = y_min.min(y);
        y_max = y_max.max(y);
    }
    (x_min, x_max, y_min, y_max)
}

fn rgb_to_instruction(rgb: &str) -> (Direction, usize) {
    let distance = usize::from_str_radix(&rgb[1..=5], 16).unwrap();
    let table = [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ];
    let dir = table[(rgb.as_bytes()[6] - b'0') as usize];
    (dir, distance)
}

/// Perform discretization on a set of values. `values` should already be sorted
/// in ascending order. Returns the width of each section and a reverse mapping
/// from value to section index.
fn discretize(values: &[i32]) -> (Vec<usize>, HashMap<i32, usize>) {
    assert!(values.is_sorted());
    assert!(!values.is_empty());

    let mut width = vec![1];
    for i in 0..values.len() - 1 {
        let diff = values[i + 1] - values[i];
        if diff > 1 {
            width.push((diff - 1) as usize);
        }
        width.push(1);
    }

    let mut mapping = HashMap::new();
    let mut prefix_sum = values[0];
    for i in 0..width.len() {
        mapping.insert(prefix_sum, i);
        prefix_sum += width[i] as i32;
    }

    (width, mapping)
}

fn main() {
    let input = std::fs::read("data/day18").unwrap();
    let input = String::from_utf8(input).unwrap();

    // (dir, distance, color)
    let mut instructions = vec![];
    for line in input.split_terminator("\r\n") {
        let tokens = line.split_ascii_whitespace().collect_vec();
        let dir = if tokens[0] == "U" {
            Direction::Up
        } else if tokens[0] == "D" {
            Direction::Down
        } else if tokens[0] == "L" {
            Direction::Left
        } else if tokens[0] == "R" {
            Direction::Right
        } else {
            unreachable!()
        };
        let distance: usize = tokens[1].parse().unwrap();
        let color = &tokens[2][1..tokens[2].len() - 1];
        instructions.push((dir, distance, color));
    }

    let step = |(x, y): (i32, i32), dir: Direction| -> (i32, i32) {
        match dir {
            Direction::Up => (x - 1, y),
            Direction::Down => (x + 1, y),
            Direction::Left => (x, y - 1),
            Direction::Right => (x, y + 1),
        }
    };

    let mut route = HashSet::new();
    let mut pos = (0, 0);
    route.insert(pos);
    for &(dir, distance, _color) in &instructions {
        for _ in 0..distance {
            pos = step(pos, dir);
            route.insert(pos);
        }
    }

    let (x_min, x_max, y_min, y_max) = bounding_box(&route);

    // Run floodfill.
    let mut inner_area = 0;
    let mut vis = HashSet::new();
    for i in x_min..=x_max {
        for j in y_min..=y_max {
            let mut todo = vec![(i, j)];
            let mut area = 0;
            let mut edge = false;
            while let Some((x, y)) = todo.pop() {
                if !vis.contains(&(x, y)) && !route.contains(&(x, y)) {
                    vis.insert((x, y));

                    area += 1;

                    if x == x_min || x == x_max || y == y_min || y == y_max {
                        edge = true;
                    }

                    if x > x_min {
                        todo.push((x - 1, y));
                    }
                    if x < x_max {
                        todo.push((x + 1, y));
                    }
                    if y > y_min {
                        todo.push((x, y - 1));
                    }
                    if y < y_max {
                        todo.push((x, y + 1));
                    }
                }
            }
            if !edge {
                inner_area += area;
            }
        }
    }
    println!("inner_area = {inner_area}");
    println!("total_area = {}", inner_area + route.len());

    // Under updated ruleset, perform discretization.
    let mut xs = BTreeSet::new();
    let mut ys = BTreeSet::new();
    let mut pos = (0, 0);
    xs.insert(0);
    ys.insert(0);
    for &(_, _, color) in &instructions {
        let (dir, distance) = rgb_to_instruction(color);
        for _ in 0..distance {
            pos = step(pos, dir);
        }
        xs.insert(pos.0);
        ys.insert(pos.1);
    }

    let xs = Vec::from_iter(xs);
    let ys = Vec::from_iter(ys);

    let (height, xmap) = discretize(&xs);
    let (width, ymap) = discretize(&ys);

    // look at the xmap
    // let mut xmap_vec: Vec<(i32, usize)> = vec![];
    // for (&value, &idx) in &xmap {
    //     xmap_vec.push((value, idx));
    // }
    // xmap_vec.sort();
    // for (value, idx) in xmap_vec {
    //     println!("{value} -> {idx}");
    // }
    // dbg!(height.len());

    let range = |lhs: usize, rhs: usize| {
        if lhs < rhs {
            lhs..=rhs
        } else {
            rhs..=lhs
        }
    };

    // Traverse the board again, but map route into discretized index.
    let mut route = HashSet::new();
    let mut pos: (i32, i32) = (0, 0);
    let mut mapped_pos: (usize, usize) = (xmap[&pos.0], ymap[&pos.1]);
    route.insert(mapped_pos);
    for &(_, _, color) in &instructions {
        let (dir, distance) = rgb_to_instruction(color);
        for _ in 0..distance {
            pos = step(pos, dir);
        }

        let new_mapped_pos = (xmap[&pos.0], ymap[&pos.1]);

        assert!(mapped_pos.0 == new_mapped_pos.0 || mapped_pos.1 == new_mapped_pos.1);

        for i in range(mapped_pos.0, new_mapped_pos.0) {
            for j in range(mapped_pos.1, new_mapped_pos.1) {
                route.insert((i, j));
            }
        }

        mapped_pos = new_mapped_pos;
    }

    // print the board after mapping
    // for i in 0..height.len() {
    //     for j in 0..width.len() {
    //         if route.contains(&(i, j)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    // Run floodfill with discretization.
    let mut inner_area = 0;
    let mut vis = HashSet::new();
    for i in 0..height.len() {
        for j in 0..width.len() {
            let mut todo = vec![(i, j)];
            let mut area = 0;
            let mut edge = false;
            while let Some((x, y)) = todo.pop() {
                if !vis.contains(&(x, y)) && !route.contains(&(x, y)) {
                    vis.insert((x, y));

                    area += height[x] * width[y];

                    if x == 0 || x == height.len() - 1 || y == 0 || y == width.len() - 1 {
                        edge = true;
                    }

                    if x > 0 {
                        todo.push((x - 1, y));
                    }
                    if x < height.len() - 1 {
                        todo.push((x + 1, y));
                    }
                    if y > 0 {
                        todo.push((x, y - 1));
                    }
                    if y < width.len() - 1 {
                        todo.push((x, y + 1));
                    }
                }
            }
            if !edge {
                inner_area += area;
            }
        }
    }
    println!("inner_area = {inner_area}");

    let mut route_area = 0;
    for &(x, y) in &route {
        route_area += height[x] * width[y];
    }
    println!("route_area = {route_area}");

    println!("total_area = {}", inner_area + route_area);
}
