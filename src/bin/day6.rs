use itertools::Itertools;

fn main() {
    let input = std::fs::read("data/day6").unwrap();
    let input = String::from_utf8(input).unwrap();

    let lines = input.split_terminator("\r\n").collect_vec();
    let time: Vec<u32> = lines[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let distance: Vec<u32> = lines[1]
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut product = 1;
    for i in 0..time.len() {
        let mut way = 0;
        for hold in 0..=time[i] {
            if hold * (time[i] - hold) > distance[i] {
                way += 1;
            }
        }
        product *= way;
    }
    println!("product = {product}");

    let big_time: u64 = lines[0]
        .split_ascii_whitespace()
        .skip(1)
        .join("")
        .parse()
        .unwrap();
    let big_distance: u64 = lines[1]
        .split_ascii_whitespace()
        .skip(1)
        .join("")
        .parse()
        .unwrap();
    let mut way = 0;
    for hold in 0..=big_time {
        if hold * (big_time - hold) > big_distance {
            way += 1;
        }
    }
    println!("way = {way}");
}
