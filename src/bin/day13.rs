use itertools::Itertools;

fn transpose<T>(mat: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!mat.is_empty());

    let n = mat[0].len();
    let mut iters: Vec<_> = mat.into_iter().map(|n| n.into_iter()).collect();
    (0..n)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn main() {
    let input = std::fs::read("data/day13").unwrap();
    let input = String::from_utf8(input).unwrap();

    let patterns = input
        .split_terminator("\r\n\r\n")
        .map(|pat| {
            pat.split_terminator("\r\n")
                .map(|line| line.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec();

    let mut summary = 0;
    'outer: for pat in &patterns {
        // check horizontal lines; the i-th line should reflect the (i+1)-th
        // line, the j-th line the (2*i - j + 1)-th line, etc.
        'inner: for i in 0..pat.len() - 1 {
            for j in 0..=i {
                let k = 2 * i - j + 1;
                if k < pat.len() && pat[j] != pat[k] {
                    continue 'inner;
                }
            }
            summary += 100 * (i + 1);
            continue 'outer;
        }

        // check vertial lines by transposing the pattern
        let pat = transpose(pat.clone());
        'inner: for i in 0..pat.len() - 1 {
            for j in 0..=i {
                let k = 2 * i - j + 1;
                if k < pat.len() && pat[j] != pat[k] {
                    continue 'inner;
                }
            }
            summary += i + 1;
            continue 'outer;
        }
    }
    println!("summary = {summary}");

    let mut summary2 = 0;
    'outer: for pat in &patterns {
        // Try fixing every possible smudge.
        for r in 0..pat.len() {
            for c in 0..pat[0].len() {
                let mut pat = pat.clone();
                if pat[r][c] == '.' {
                    pat[r][c] = '#';
                } else {
                    pat[r][c] = '.';
                }

                // check horizontal lines; the i-th line should reflect the (i+1)-th
                // line, the j-th line the (2*i - j + 1)-th line, etc.
                'inner: for i in 0..pat.len() - 1 {
                    // The smudge must be part of the reflection.
                    let mut smudged = false;

                    for j in 0..=i {
                        let k = 2 * i - j + 1;
                        if k < pat.len() && pat[j] != pat[k] {
                            continue 'inner;
                        }
                        if k == r {
                            smudged = true;
                        }
                    }
                    if smudged {
                        summary2 += 100 * (i + 1);
                        continue 'outer;
                    }
                }

                // check vertial lines by transposing the pattern
                let pat = transpose(pat);
                'inner: for i in 0..pat.len() - 1 {
                    // The smudge must be part of the reflection.
                    let mut smudged = false;

                    for j in 0..=i {
                        let k = 2 * i - j + 1;
                        if k < pat.len() && pat[j] != pat[k] {
                            continue 'inner;
                        }
                        if k == c {
                            smudged = true;
                        }
                    }
                    if smudged {
                        summary2 += i + 1;
                        continue 'outer;
                    }
                }
            }
        }
    }
    println!("summary2 = {summary2}");
}
