use std::{cmp::Ordering, collections::HashMap, hash::Hash};

use itertools::Itertools;

fn counter<T: Eq + Hash>(iter: impl Iterator<Item = T>) -> HashMap<T, usize> {
    let mut result = HashMap::new();
    for item in iter {
        *result.entry(item).or_default() += 1;
    }
    result
}

// Find the category number of a hand. "Five of a kind" is represented as 7,
// while "High card" is represented by 1.
fn get_category(hand: &str) -> u32 {
    let mut dist = counter(hand.bytes()).into_values().collect_vec();
    dist.sort();

    if dist == [5] {
        // Five of a kind
        7
    } else if dist == [1, 4] {
        // Four of a kind
        6
    } else if dist == [2, 3] {
        // Full house
        5
    } else if dist == [1, 1, 3] {
        // Three of a kind
        4
    } else if dist == [1, 2, 2] {
        // Two pair
        3
    } else if dist == [1, 1, 1, 2] {
        // One pair
        2
    } else if dist == [1, 1, 1, 1, 1] {
        // High card
        1
    } else {
        unreachable!()
    }
}

// Find the category number of a hand (updated rules).
fn get_category2(hand: &str) -> u32 {
    let mut ctr = counter(hand.bytes());
    let joker = ctr.remove(&b'J').unwrap_or_default();

    let mut dist = ctr.into_values().collect_vec();
    dist.sort();

    if dist.is_empty() {
        // special case of "JJJJJ"
        return 7;
    }

    *dist.last_mut().unwrap() += joker;

    if dist == [5] {
        // Five of a kind
        7
    } else if dist == [1, 4] {
        // Four of a kind
        6
    } else if dist == [2, 3] {
        // Full house
        5
    } else if dist == [1, 1, 3] {
        // Three of a kind
        4
    } else if dist == [1, 2, 2] {
        // Two pair
        3
    } else if dist == [1, 1, 1, 2] {
        // One pair
        2
    } else if dist == [1, 1, 1, 1, 1] {
        // High card
        1
    } else {
        unreachable!()
    }
}

// Compare two hands without considering categories.
fn secondary_cmp(lhs: &str, rhs: &str) -> Ordering {
    assert_eq!(lhs.len(), 5);
    assert_eq!(rhs.len(), 5);

    let strength = "AKQJT98765432";

    for i in 0..5 {
        let left = strength.find(lhs.as_bytes()[i] as char).unwrap();
        let right = strength.find(rhs.as_bytes()[i] as char).unwrap();
        if left < right {
            return Ordering::Greater;
        } else if left > right {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

// Compare two hands without considering categories (updated rules).
fn secondary_cmp2(lhs: &str, rhs: &str) -> Ordering {
    assert_eq!(lhs.len(), 5);
    assert_eq!(rhs.len(), 5);

    let strength = "AKQT98765432J";

    for i in 0..5 {
        let left = strength.find(lhs.as_bytes()[i] as char).unwrap();
        let right = strength.find(rhs.as_bytes()[i] as char).unwrap();
        if left < right {
            return Ordering::Greater;
        } else if left > right {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn camel_cmp(lhs: &str, rhs: &str) -> Ordering {
    let lhs_category = get_category(lhs);
    let rhs_category = get_category(rhs);

    if lhs_category > rhs_category {
        Ordering::Greater
    } else if lhs_category < rhs_category {
        Ordering::Less
    } else {
        secondary_cmp(lhs, rhs)
    }
}

fn camel_cmp2(lhs: &str, rhs: &str) -> Ordering {
    let lhs_category = get_category2(lhs);
    let rhs_category = get_category2(rhs);

    if lhs_category > rhs_category {
        Ordering::Greater
    } else if lhs_category < rhs_category {
        Ordering::Less
    } else {
        secondary_cmp2(lhs, rhs)
    }
}

fn main() {
    let input = std::fs::read("data/day7").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut hands: Vec<(&str, u32)> = vec![];
    for line in input.split_terminator("\r\n") {
        let (hand, bid) = line.split_once(" ").unwrap();
        hands.push((hand, bid.parse().unwrap()));
    }

    hands.sort_by(|lhs, rhs| camel_cmp(lhs.0, rhs.0));
    let mut winning = 0;
    for i in 0..hands.len() {
        winning += (i as u32 + 1) * hands[i].1;
    }
    println!("winning = {winning}");

    hands.sort_by(|lhs, rhs| camel_cmp2(lhs.0, rhs.0));
    let mut winning2 = 0;
    for i in 0..hands.len() {
        winning2 += (i as u32 + 1) * hands[i].1;
    }
    println!("winning2 = {winning2}");
}
