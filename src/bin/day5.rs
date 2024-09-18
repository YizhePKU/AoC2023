use itertools::Itertools;

// (dst_start, src_start, mapping_length)
type Mapping = Vec<(u64, u64, u64)>;

// (range_start, range_length)
type Range = Vec<(u64, u64)>;

fn apply_mapping(src: u64, mapping: &Mapping) -> u64 {
    for (dst_start, src_start, length) in mapping {
        if src >= *src_start && src < *src_start + *length {
            return src - *src_start + *dst_start;
        }
    }

    src
}

fn apply_mapping_range(range: &Range, mapping: &Mapping) -> Range {
    // sort `mapping` by `src_start`
    let mut mapping = mapping.clone();
    mapping.sort_by_key(|m| m.1);

    let mut result: Range = vec![];
    for &(range_start, range_length) in range {
        let range_end = range_start + range_length;

        // [i, range_end) is the part of the range that hasn't been processed yet.
        let mut i = range_start;
        for &(dst_start, src_start, mapping_length) in &mapping {
            let src_end = src_start + mapping_length;

            // There're four situations to consider when processing range [i,
            // range_end).

            // Situation 1: the mapping covers the range completely. Map [i,
            // range_end) to [dst_start + i - src_start, dst_start - src_start +
            // range_end)
            if src_start <= i && src_end >= range_end {
                result.push((dst_start + i - src_start, range_end - i));
                i = range_end;
            }
            // Situation 2: the mapping covers the starting part, but breaks in
            // the middle. Map [i, src_end) to [dst_start + i - src_start,
            // dst_end).
            else if src_start <= i && src_end > i && src_end < range_end {
                result.push((dst_start + i - src_start, src_end - i));
                i = src_end;
            }
            // Situation 3: the mapping covers the ending part, but starts
            // halfway. Map [i, src_start) to itself, then map [src_start,
            // range_end) to [dst_start, dst_start + range_end - src_start).
            else if src_start > i && src_start < range_end && src_end >= range_end {
                result.push((i, src_start - i));
                result.push((dst_start, range_end - src_start));
                i = range_end;
            }
            // Situation 4: the mapping is inside the range. Map [i, src_start)
            // to itself, then map [src_start, src_end) to [dst_start,
            // dst_end).
            else if src_start > i && src_end < range_end {
                result.push((i, src_start - i));
                result.push((dst_start, mapping_length));
                i = src_end;
            }
        }
        // map the rest of the un-mapped portion to itself
        if i < range_end {
            result.push((i, range_end - i));
        }
    }

    result
}

fn main() {
    let input = std::fs::read("data/day5").unwrap();
    let input = String::from_utf8(input).unwrap();

    let sections = input.split_terminator("\r\n\r\n").collect_vec();

    // The first section defines the seeds.
    let seeds: Vec<u64> = sections[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    // The rest of the sections define mappings.
    let mut mappings = vec![];
    for section in &sections[1..] {
        let mapping: Mapping = section
            .split_terminator("\r\n")
            .skip(1)
            .map(|s| {
                s.split_ascii_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();
        mappings.push(mapping);
    }

    let lowest_location: u64 = seeds
        .iter()
        .map(|&seed| mappings.iter().fold(seed, apply_mapping))
        .min()
        .unwrap();
    println!("lowest_location = {lowest_location}");

    // let range = vec![(0, 10), (20, 10)];
    // let mapping = vec![(100, 5, 2), (200, 18, 4), (300, 28, 4)];
    // let result = apply_mapping_range(&range, &mapping);
    // dbg!(result);
    let seed_range: Range = seeds
        .chunks_exact(2)
        .map(|slice| slice.into_iter().copied().collect_tuple().unwrap())
        .collect_vec();
    // dbg!(&seed_range);

    let location_range = mappings.iter().fold(seed_range, |range, mapping| {
        apply_mapping_range(&range, mapping)
    });
    // dbg!(&location_range);

    let lowest_location: u64 = location_range
        .iter()
        .map(|&(start, _)| start)
        .min()
        .unwrap();
    println!("lowest_location = {lowest_location}");
}
