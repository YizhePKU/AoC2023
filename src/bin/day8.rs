use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Cycle {
    size: usize,
    exits: Vec<usize>,
}

fn gcd(low: usize, high: usize) -> usize {
    if low > high {
        gcd(high, low)
    } else if low == 0 {
        high
    } else {
        gcd(high % low, low)
    }
}

fn lcm(low: usize, high: usize) -> usize {
    low / gcd(low, high) * high
}

fn merge_cycle(lhs: Cycle, rhs: Cycle) -> Cycle {
    let mut result = Cycle {
        size: lcm(lhs.size, rhs.size),
        exits: vec![],
    };

    for i in 0..result.size / lhs.size {
        for exit in &lhs.exits {
            let x = exit + i * lhs.size;
            if rhs.exits.iter().any(|y| &x >= y && (x - y) % rhs.size == 0) {
                result.exits.push(x);
            }
        }
    }

    result
}

fn main() {
    let input = std::fs::read("data/day8").unwrap();
    let input = String::from_utf8(input).unwrap();

    let re = Regex::new(r"(.+) = \((.+), (.+)\)").unwrap();

    let (instruction, lines) = input.split_once("\r\n\r\n").unwrap();

    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines.split_terminator("\r\n") {
        let caps = re.captures(line).unwrap();
        let src = caps.get(1).unwrap().as_str();
        let left = caps.get(2).unwrap().as_str();
        let right = caps.get(3).unwrap().as_str();
        network.insert(src, (left, right));
    }

    // let mut cur = "AAA";
    // let mut step = 0;
    // while cur != "ZZZ" {
    //     if instruction.as_bytes()[step % instruction.len()] == b'L' {
    //         cur = network[cur].0;
    //     } else if instruction.as_bytes()[step % instruction.len()] == b'R' {
    //         cur = network[cur].1;
    //     } else {
    //         unreachable!();
    //     }

    //     step += 1;
    // }
    // println!("step = {step}");

    let mut nodes = network
        .keys()
        .filter(|s| s.ends_with('A'))
        .cloned()
        .collect_vec();

    // Run the simulation for 1000 * instruction.len() steps to make sure all
    // nodes are in a cycle.
    let preheat = 1000 * instruction.len();

    for step in 0..preheat {
        if instruction.as_bytes()[step % instruction.len()] == b'L' {
            nodes.iter_mut().for_each(|s| *s = network[*s].0);
        } else if instruction.as_bytes()[step % instruction.len()] == b'R' {
            nodes.iter_mut().for_each(|s| *s = network[*s].1);
        } else {
            unreachable!();
        }
    }

    // Analyze the cycle size of every node and mark all exits.
    let mut cycles = vec![];
    for node in nodes {
        let mut cur = node;
        let mut exits = vec![];
        for step in preheat.. {
            if step != preheat && step % instruction.len() == 0 && cur == node {
                cycles.push(Cycle {
                    size: (step - preheat).into(),
                    exits,
                });
                break;
            }

            if cur.ends_with('Z') {
                exits.push((step - preheat).into());
            }

            if instruction.as_bytes()[step % instruction.len()] == b'L' {
                cur = network[cur].0;
            } else if instruction.as_bytes()[step % instruction.len()] == b'R' {
                cur = network[cur].1;
            } else {
                unreachable!();
            }
        }
    }

    let final_cycle = cycles.into_iter().reduce(merge_cycle).unwrap();
    println!(
        "final_step = {}",
        preheat + final_cycle.exits.first().unwrap()
    );
}
