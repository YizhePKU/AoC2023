use itertools::Itertools;

fn main() {
    let input = std::fs::read("data/day11").unwrap();
    let input = String::from_utf8(input).unwrap();

    let sky = input
        .split_terminator("\r\n")
        .map(|s| s.bytes().collect_vec())
        .collect_vec();
    let n = sky.len();
    let m = sky[0].len();

    let mut stars = vec![];
    for i in 0..n {
        for j in 0..m {
            if sky[i][j] == b'#' {
                stars.push((i, j));
            }
        }
    }

    let mut expand_row = vec![true; n];
    let mut expand_column = vec![true; m];
    for i in 0..n {
        for j in 0..m {
            if sky[i][j] == b'#' {
                expand_row[i] = false;
                expand_column[j] = false;
            }
        }
    }

    // Returns the Manhattan Distance between two points, taking space expansion
    // into consideration.
    let distance = |p1: (usize, usize), p2: (usize, usize)| {
        let r1 = usize::min(p1.0, p2.0);
        let r2 = usize::max(p1.0, p2.0);
        let c1 = usize::min(p1.1, p2.1);
        let c2 = usize::max(p1.1, p2.1);

        let mut result = (r2 - r1) + (c2 - c1);
        for i in r1..r2 {
            if expand_row[i] {
                result += 999999;
            }
        }
        for i in c1..c2 {
            if expand_column[i] {
                result += 999999;
            }
        }
        result
    };

    let mut pairwise_sum = 0;
    for i in 0..stars.len() {
        for j in i + 1..stars.len() {
            pairwise_sum += distance(stars[i], stars[j]);
        }
    }
    println!("pairwise_sum = {pairwise_sum}");
}
