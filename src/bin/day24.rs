use itertools::Itertools;

type Position = (f64, f64, f64);
type Velocity = (f64, f64, f64);

fn parse_input() -> Vec<(Position, Velocity)> {
    let input = std::fs::read("data/day24").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut result = vec![];
    for line in input.split_terminator("\r\n") {
        let (part1, part2) = line.split_once(" @ ").unwrap();
        let position = part1
            .split_terminator(", ")
            .map(|s| s.trim().parse().unwrap())
            .collect_tuple()
            .unwrap();
        let velocity = part2
            .split_terminator(", ")
            .map(|s| s.trim().parse().unwrap())
            .collect_tuple()
            .unwrap();
        result.push((position, velocity));
    }

    result
}

/// Returns the timing for the two hailstone to intersect. Both timing can be
/// negative. Returns None if the two hailstone never cross (parallel paths).
fn intersect_2d(stone1: (Position, Velocity), stone2: (Position, Velocity)) -> Option<(f64, f64)> {
    let ((x1, y1, _z1), (dx1, dy1, _dz1)) = stone1;
    let ((x2, y2, _z2), (dx2, dy2, _dz2)) = stone2;

    let det = dy1 * dx2 - dx1 * dy2;

    if det == 0. {
        // Assuming two lines colliding completely doesn't count as "intersect".
        return None;
    }

    let det1 = dx2 * (y2 - y1) - dy2 * (x2 - x1);
    let det2 = dx1 * (y2 - y1) - dy1 * (x2 - x1);

    let t1 = det1 / det;
    let t2 = det2 / det;

    Some((t1, t2))
}

fn main() {
    let stones = parse_input();

    let mut part1 = 0;
    for i in 0..stones.len() {
        for j in i + 1..stones.len() {
            if let Some((t1, t2)) = intersect_2d(stones[i], stones[j]) {
                if t1 >= 0. && t2 >= 0. {
                    let ((x1, y1, _z1), (dx1, dy1, _dz1)) = stones[i];
                    let x = x1 + (dx1 * t1);
                    let y = y1 + (dy1 * t1);

                    let min = 200000000000000.;
                    let max = 400000000000000.;
                    if x >= min && x <= max {
                        if y >= min && y <= max {
                            part1 += 1;
                        }
                    }
                }
            }
        }
    }
    dbg!(part1);

    // Only three stones are needed to determine the answer. Use any solver (I
    // used Maple) to solve a system of polynomial equations.
}
