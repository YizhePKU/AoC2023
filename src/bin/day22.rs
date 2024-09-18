use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Block(i32, i32, i32);

impl Block {
    fn fall(&self) -> Self {
        Self(self.0, self.1, self.2 - 1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick(HashSet<Block>);

impl Brick {
    fn fall(&self) -> Self {
        let mut brick = Brick(HashSet::new());
        for point in &self.0 {
            brick.0.insert(point.fall());
        }
        brick
    }

    fn is_grounded(&self) -> bool {
        self.0.iter().any(|point| point.2 == 1)
    }

    fn collides_with(&self, other: &Brick) -> bool {
        self.0.intersection(&other.0).next().is_some()
    }
}

fn range_inclusive(start: i32, end: i32) -> std::ops::RangeInclusive<i32> {
    if start <= end {
        start..=end
    } else {
        end..=start
    }
}

fn parse_input() -> Vec<Brick> {
    let input = std::fs::read("data/day22").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut bricks = vec![];
    for line in input.split_terminator("\r\n") {
        let (start, end) = line.split_once('~').unwrap();
        let (x1, y1, z1) = start
            .split_terminator(',')
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let (x2, y2, z2) = end
            .split_terminator(',')
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();

        let mut brick = Brick(HashSet::new());
        for x in range_inclusive(x1, x2) {
            for y in range_inclusive(y1, y2) {
                for z in range_inclusive(z1, z2) {
                    brick.0.insert(Block(x, y, z));
                }
            }
        }
        bricks.push(brick);
    }

    bricks
}

// Wait until all bricks fall downward as far as they can go. Returns the number
// of bricks that falled.
fn stablize(bricks: &mut Vec<Brick>) -> usize {
    let mut index_of_bricks_that_have_falled_at_least_once = HashSet::new();

    let mut blocks = HashSet::new();
    for brick in bricks.iter() {
        for block in &brick.0 {
            blocks.insert(*block);
        }
    }

    loop {
        let mut candidates = vec![];
        for i in 0..bricks.len() {
            if !bricks[i].is_grounded() {
                // Remove the blocks of this brick.
                for block in &bricks[i].0 {
                    blocks.remove(block);
                }
                // Check if a brick can fall without colliding with other bricks.
                if blocks.intersection(&bricks[i].fall().0).next().is_none() {
                    index_of_bricks_that_have_falled_at_least_once.insert(i);
                    candidates.push(i);
                }
                // Add back the blocks of this brick.
                for block in &bricks[i].0 {
                    blocks.insert(*block);
                }
            }
        }

        if candidates.is_empty() {
            break;
        } else {
            for i in candidates {
                // Remove the blocks of this brick.
                for block in &bricks[i].0 {
                    blocks.remove(block);
                }
                // Let it fall.
                bricks[i] = bricks[i].fall();
                // Add back the blocks of this brick.
                for block in &bricks[i].0 {
                    blocks.insert(*block);
                }
            }
        }
    }

    index_of_bricks_that_have_falled_at_least_once.len()
}

fn is_safe_to_disintegrate(bricks: &[Brick], idx: usize) -> bool {
    let mut bricks = bricks.to_vec();
    bricks.remove(idx);

    let original = bricks.clone();
    stablize(&mut bricks);

    bricks == original
}

fn main() {
    let mut bricks = parse_input();

    stablize(&mut bricks);

    let mut safe_to_disintegrate = 0;
    let mut chain_reaction = 0;
    for i in 0..bricks.len() {
        dbg!(i);
        if is_safe_to_disintegrate(&bricks, i) {
            safe_to_disintegrate += 1;
        } else {
            let mut temp_bricks = bricks.clone();
            temp_bricks.remove(i);
            chain_reaction += stablize(&mut temp_bricks);
        }
    }
    dbg!(safe_to_disintegrate, chain_reaction);
}
