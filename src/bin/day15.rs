use itertools::Itertools;

fn hash(s: &str) -> u8 {
    let mut result: u8 = 0;
    for byte in s.bytes() {
        result = result.wrapping_add(byte);
        result = result.wrapping_mul(17);
    }
    result
}

fn main() {
    let input = std::fs::read("data/day15").unwrap();
    let input = String::from_utf8(input).unwrap();

    let steps = input.split_terminator(',').map(|s| s.trim()).collect_vec();

    let mut sum = 0;
    for step in &steps {
        sum += hash(step) as u64;
    }
    println!("sum = {sum}");

    let mut hashmap: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];
    for step in &steps {
        if let Some((label, focal_len)) = step.split_once('=') {
            let idx = hash(label) as usize;
            let focal_len: u32 = focal_len.parse().unwrap();
            if let Some(i) = hashmap[idx].iter().position(|&entry| entry.0 == label) {
                hashmap[idx][i].1 = focal_len;
            } else {
                hashmap[idx].push((label, focal_len));
            }
        } else if let Some((label, _)) = step.split_once('-') {
            let idx = hash(label) as usize;
            if let Some(i) = hashmap[idx].iter().position(|&entry| entry.0 == label) {
                hashmap[idx].remove(i);
            }
        } else {
            panic!("Bad step: {}", step);
        }
    }

    let mut power = 0;
    for idx in 0..256 {
        for slot in 0..hashmap[idx].len() {
            power += (idx as u32 + 1) * (slot as u32 + 1) * hashmap[idx][slot].1;
        }
    }
    println!("power = {power}");
}
