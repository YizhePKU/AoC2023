fn diff(xs: &[i32]) -> Vec<i32> {
    let mut result = vec![];
    for i in 1..xs.len() {
        result.push(xs[i] - xs[i - 1]);
    }
    result
}

fn sigma(init: i32, xs: &[i32]) -> Vec<i32> {
    let mut result = vec![init];
    for x in xs {
        result.push(x + result.last().unwrap());
    }
    result
}

fn extrapolate(xs: &[i32]) -> Vec<i32> {
    if xs.iter().all(|&x| x == 0) {
        vec![0; xs.len() + 1]
    } else {
        sigma(xs[0], &extrapolate(&diff(xs)))
    }
}

fn main() {
    let input = std::fs::read("data/day9").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut sum = 0;
    for line in input.split_terminator("\r\n") {
        let xs: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let extrapolated = extrapolate(&xs);
        sum += extrapolated.last().unwrap();
    }
    println!("sum = {sum}");

    let mut sum = 0;
    for line in input.split_terminator("\r\n") {
        let mut xs: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        xs.reverse();
        let extrapolated = extrapolate(&xs);
        sum += extrapolated.last().unwrap();
    }
    println!("sum = {sum}");
}
