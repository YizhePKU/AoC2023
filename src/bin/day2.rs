use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct CubeSet {
    r: u32,
    g: u32,
    b: u32,
}

fn main() {
    let input = std::fs::read("data/day2").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut games: HashMap<u32, Vec<CubeSet>> = HashMap::new();
    for line in input.split_terminator("\r\n") {
        let (part1, part2) = line.split_once(": ").unwrap();
        let id: u32 = part1[5..].parse().unwrap();

        let mut game = vec![];
        for set in part2.split_terminator("; ") {
            let mut cubes = CubeSet { r: 0, g: 0, b: 0 };
            for pair in set.split_terminator(", ") {
                let (number, color) = pair.split_once(' ').unwrap();
                let n: u32 = number.parse().unwrap();
                if color == "red" {
                    cubes.r = n;
                } else if color == "green" {
                    cubes.g = n;
                } else if color == "blue" {
                    cubes.b = n;
                } else {
                    panic!("Unknown color {color}");
                }
            }
            game.push(cubes);
        }
        games.insert(id, game);
    }

    let sum: u32 = games
        .iter()
        .filter(|(_, sets)| {
            sets.iter()
                .all(|set| set.r <= 12 && set.g <= 13 && set.b <= 14)
        })
        .map(|(id, _)| id)
        .sum();
    println!("sum = {sum}");

    let powersum: u32 = games
        .iter()
        .map(|(_, sets)| {
            let r = sets.iter().map(|s| s.r).max().unwrap();
            let g = sets.iter().map(|s| s.g).max().unwrap();
            let b = sets.iter().map(|s| s.b).max().unwrap();
            r * g * b
        })
        .sum();
    println!("powersum = {powersum}");
}
