use std::collections::HashMap;

use itertools::Itertools;

fn calibrate(s: &str) -> u32 {
    let digits = s
        .bytes()
        .filter(|&b| b >= b'0' && b <= b'9')
        .map(|b| b - b'0')
        .collect_vec();

    if digits.is_empty() {
        panic!("{s} doesn't contain any digits");
    }

    (digits[0] * 10 + digits[digits.len() - 1]) as u32
}

fn calibrate2(s: &str) -> u32 {
    let targets: HashMap<&str, u32> = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let a = targets
        .iter()
        .filter(|(&k, _)| s.contains(k))
        .min_by_key(|(&k, _)| s.find(k).unwrap())
        .expect("No digit-like thingy in {s}")
        .1;
    let b = targets
        .iter()
        .filter(|(&k, _)| s.contains(k))
        .max_by_key(|(&k, _)| s.rfind(k).unwrap())
        .unwrap()
        .1;

    a * 10 + b
}

fn main() {
    let input = std::fs::read("data/day1").unwrap();
    let input = String::from_utf8(input).unwrap();

    let sum: u32 = input.split_ascii_whitespace().map(calibrate2).sum();
    println!("Sum is {}", sum);
}
