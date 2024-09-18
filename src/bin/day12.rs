use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Record {
    cond: Vec<char>,
    desc: Vec<usize>,
}

type Cache<'a, 'b> = HashMap<(&'a [char], &'b [usize], usize), usize>;

fn search<'a, 'b>(
    cond: &'a [char],
    desc: &'b [usize],
    n: usize,
    cache: &mut Cache<'a, 'b>,
) -> usize {
    // cache lookup
    if let Some(value) = cache.get(&(cond, desc, n)) {
        return *value;
    }

    if cond.is_empty() {
        if (desc.is_empty() && n == 0) || (desc.len() == 1 && desc[0] == n) {
            1
        } else {
            0
        }
    } else {
        let mut result = 0;

        if cond[0] == '#' || cond[0] == '?' {
            result += search(&cond[1..], desc, n + 1, cache);
        }

        if cond[0] == '.' || cond[0] == '?' {
            if n == 0 {
                result += search(&cond[1..], desc, 0, cache);
            } else if !desc.is_empty() && n == desc[0] {
                result += search(&cond[1..], &desc[1..], 0, cache);
            }
        }

        // cache update
        cache.insert((cond, desc, n), result);

        result
    }
}

fn main() {
    let input = std::fs::read("data/day12").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut records = vec![];
    for line in input.split_terminator("\r\n") {
        let (part1, part2) = line.split_once(" ").unwrap();
        let cond: Vec<char> = part1.chars().collect_vec();
        let desc: Vec<usize> = part2.split(',').map(|s| s.parse().unwrap()).collect_vec();
        records.push(Record { cond, desc });
    }

    let mut sum = 0;
    let mut cache = HashMap::new();
    for Record { cond, desc } in &records {
        let result = search(cond, desc, 0, &mut cache);
        sum += result;
    }
    println!("sum = {sum}");

    let mut sum2 = 0;
    for Record { cond, desc } in &records {
        let mut cond2 = vec![];
        cond2.extend_from_slice(cond);
        cond2.push('?');
        cond2.extend_from_slice(cond);
        cond2.push('?');
        cond2.extend_from_slice(cond);
        cond2.push('?');
        cond2.extend_from_slice(cond);
        cond2.push('?');
        cond2.extend_from_slice(cond);

        let mut desc2 = vec![];
        desc2.extend_from_slice(desc);
        desc2.extend_from_slice(desc);
        desc2.extend_from_slice(desc);
        desc2.extend_from_slice(desc);
        desc2.extend_from_slice(desc);

        // localized cache
        let mut cache = HashMap::new();
        let result = search(&cond2, &desc2, 0, &mut cache);
        sum2 += result;

        println!("partial sum2 = {sum2}");
    }
    println!("sum2 = {sum2}");
}
