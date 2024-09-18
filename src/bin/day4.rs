use std::collections::HashSet;

fn main() {
    let input = std::fs::read("data/day4").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut cards: Vec<(HashSet<u32>, HashSet<u32>)> = vec![];
    for line in input.split_terminator("\r\n") {
        let (_, line) = line.split_once(": ").unwrap();
        let (part1, part2) = line.split_once(" | ").unwrap();

        let winning: HashSet<u32> = part1
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let own: HashSet<u32> = part2
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        cards.push((winning, own));
    }

    let mut score = 0;
    for (winning, own) in &cards {
        let n = winning.intersection(own).count() as u32;
        if n > 0 {
            score += u32::pow(2, n - 1);
        }
    }
    println!("score = {score}");

    let mut cnt = vec![1; cards.len()];
    for i in 0..cards.len() {
        // Process the i-th card, which we have cnt[i] copies of.
        let (winning, own) = &cards[i];
        let n = winning.intersection(own).count();
        for j in 1..=n {
            cnt[i + j] += cnt[i];
        }
    }
    let total: u32 = cnt.iter().sum();
    println!("total = {total}");
}
