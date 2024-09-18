use itertools::Itertools;
use rand::{seq::IteratorRandom, thread_rng};
use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, HashMap<String, usize>>;

fn parse_input() -> Graph {
    let input = std::fs::read("data/day25").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut result: Graph = HashMap::new();
    for line in input.split_terminator("\r\n") {
        let (lhs, part2) = line.split_once(": ").unwrap();
        for rhs in part2.split_whitespace() {
            *result
                .entry(lhs.to_string())
                .or_default()
                .entry(rhs.to_string())
                .or_default() += 1;
            *result
                .entry(rhs.to_string())
                .or_default()
                .entry(lhs.to_string())
                .or_default() += 1;
        }
    }

    result
}

/// Randomly contract the graph until there're only two vertexes left.
fn contract(graph: &mut Graph) {
    let mut rng = thread_rng();

    while graph.len() > 2 {
        // Pick an edge at random.
        let lhs = graph.keys().choose(&mut rng).unwrap().to_string();
        let rhs = graph[&lhs].keys().choose(&mut rng).unwrap().to_string();

        // Remove the two vertexes from the graph, keeping track of which other
        // vertexes it is connected (and how many).
        let mut connectivity = HashMap::new();
        for (vertex, count) in graph.remove(&lhs).unwrap() {
            if vertex != lhs && vertex != rhs {
                graph.get_mut(&vertex).unwrap().remove(&lhs);
                *connectivity.entry(vertex).or_default() += count;
            }
        }
        for (vertex, count) in graph.remove(&rhs).unwrap() {
            if vertex != lhs && vertex != rhs {
                graph.get_mut(&vertex).unwrap().remove(&rhs);
                *connectivity.entry(vertex).or_default() += count;
            }
        }

        // Add a new vertex representing the merge of the two vertexes.
        let name = format!("{lhs}-{rhs}");
        for (vertex, count) in connectivity {
            graph
                .entry(name.to_string())
                .or_default()
                .insert(vertex.to_string(), count);
            graph
                .entry(vertex.to_string())
                .or_default()
                .insert(name.to_string(), count);
        }
    }
}

fn main() {
    let graph = parse_input();

    // Keep contracting until we achieve a 3-cut.
    let mut attempt = 0;
    let (component1, component2) = loop {
        attempt += 1;
        println!("Attempt {attempt}");

        let mut graph = graph.clone();
        contract(&mut graph);
        let k = *graph.values().next().unwrap().values().next().unwrap();
        if k == 3 {
            break graph.into_keys().collect_tuple().unwrap();
        }
    };
    dbg!(&component1, &component2);

    let component_size1 = (component1.len() + 1) / 4;
    let component_size2 = (component2.len() + 1) / 4;
    dbg!(
        component_size1,
        component_size2,
        component_size1 * component_size2
    );
}
