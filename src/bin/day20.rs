use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

impl Pulse {
    fn negate(&mut self) {
        match self {
            Pulse::High => *self = Pulse::Low,
            Pulse::Low => *self = Pulse::High,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    Broadcaster {
        dest: Vec<String>,
    },
    FlipFlop {
        dest: Vec<String>,
        state: Pulse,
    },
    Conjunction {
        dest: Vec<String>,
        state: HashMap<String, Pulse>,
    },
}

impl Module {
    fn dest(&self) -> &Vec<String> {
        match self {
            Module::Broadcaster { dest } => &dest,
            Module::FlipFlop { dest, state: _ } => &dest,
            Module::Conjunction { dest, state: _ } => &dest,
        }
    }

    /// Receive a pulse and updates its internal state. Returns the pulse it
    /// would send to all its destination modules, if any.
    fn recv(&mut self, src: &str, pulse: Pulse) -> Option<Pulse> {
        match self {
            Module::Broadcaster { dest: _ } => Some(pulse),
            Module::FlipFlop { dest: _, state } => {
                if pulse == Pulse::Low {
                    state.negate();
                    Some(*state)
                } else {
                    None
                }
            }
            Module::Conjunction { dest: _, state } => {
                *state.get_mut(src).unwrap() = pulse;

                if state.values().all(|&pulse| pulse == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        }
    }
}

fn part1(mut modules: HashMap<String, Module>) {
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    // Press the button 1000 times.
    for _ in 0..1000 {
        // A queue of pulses in the order they were sent. A pulse is stored as
        // (source, destination, pulse_type).
        let mut queue = VecDeque::new();
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

        while let Some((source, destination, pulse)) = queue.pop_front() {
            // println!("{source} -> {destination} ({pulse:?})");

            if pulse == Pulse::Low {
                low_pulses += 1;
            } else {
                high_pulses += 1;
            }

            if let Some(module) = modules.get_mut(&destination) {
                if let Some(next_pulse) = module.recv(&source, pulse) {
                    for next_destination in module.dest() {
                        queue.push_back((
                            destination.to_string(),
                            next_destination.to_string(),
                            next_pulse,
                        ));
                    }
                }
            }
        }
        // println!();
    }

    println!("low_pulses = {low_pulses}");
    println!("high_pulses = {high_pulses}");
    println!("product = {}", low_pulses * high_pulses);
}

fn part2_qt(modules: &mut HashMap<String, Module>) {
    for button in 1..10000 {
        let mut done = false;

        let mut queue = VecDeque::new();
        queue.push_back(("button".to_string(), "sm".to_string(), Pulse::Low));

        while let Some((source, destination, pulse)) = queue.pop_front() {
            if destination == "pl" && pulse == Pulse::Low {
                println!("part2_qt: button = {button}");
                done = true;
            }

            if let Some(module) = modules.get_mut(&destination) {
                if let Some(next_pulse) = module.recv(&source, pulse) {
                    for next_destination in module.dest() {
                        queue.push_back((
                            destination.to_string(),
                            next_destination.to_string(),
                            next_pulse,
                        ));
                    }
                }
            }
        }

        if done {
            break;
        }
    }
}

fn main() {
    let input = std::fs::read("data/day20").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut modules: HashMap<String, Module> = HashMap::new();
    for line in input.split_terminator("\r\n") {
        let (name, destinations) = line.split_once(" -> ").unwrap();
        let dest = destinations
            .split_terminator(", ")
            .map(|s| s.to_string())
            .collect_vec();

        if name.starts_with('%') {
            let name = name[1..].to_string();
            let state = Pulse::Low;
            let module = Module::FlipFlop { dest, state };
            modules.insert(name, module);
        } else if name.starts_with('&') {
            let name = name[1..].to_string();
            let state = HashMap::new(); // state for conjunction modules needs to be seperately initialized
            let module = Module::Conjunction { dest, state };
            modules.insert(name, module);
        } else {
            assert_eq!(name, "broadcaster");
            let name = name.to_string();
            let module = Module::Broadcaster { dest };
            modules.insert(name, module);
        }
    }

    // Initialize states for conjunction modules.
    let mut reverse_propagation_graph: HashMap<String, Vec<String>> = HashMap::new();
    for (name, module) in &modules {
        for dest in module.dest() {
            reverse_propagation_graph
                .entry(dest.to_string())
                .or_default()
                .push(name.to_string());
        }
    }
    for (name, sources) in reverse_propagation_graph {
        if let Some(Module::Conjunction { dest: _, state }) = modules.get_mut(&name) {
            for source in sources {
                state.insert(source.to_string(), Pulse::Low);
            }
        }
    }

    // part1(modules.clone());
    // let mut modules_qt = modules.clone();
    // part2_qt(&mut modules_qt);
    // part2_qt(&mut modules_qt);
    // part2_qt(&mut modules_qt);

    let qt_mask: u64 = 0b1110_1101_0101;
    let qt_delta = 0b0001_0010_1011;

    let vt_mask = 0b1111_1010_0011;
    let vt_delta = 0b0000_0101_1101;

    let dq_mask = 0b1111_0010_1001;
    let dq_delta = 0b0000_1101_0111;

    let nl_mask = 0b1110_1110_1111;
    let nl_delta = 0b0001_0001_0001;

    dbg!(qt_mask * vt_mask * dq_mask * nl_mask);
}
