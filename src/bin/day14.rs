use std::collections::HashMap;

use itertools::Itertools;

fn tilt_north(platform: &mut Vec<Vec<char>>) {
    let n = platform.len();
    let m = platform[0].len();

    for i in 0..n {
        for j in 0..m {
            if platform[i][j] == 'O' {
                let mut k = i;
                while k > 0 && platform[k - 1][j] == '.' {
                    platform[k][j] = '.';
                    platform[k - 1][j] = 'O';
                    k -= 1;
                }
            }
        }
    }
}

fn tilt_south(platform: &mut Vec<Vec<char>>) {
    let n = platform.len();
    let m = platform[0].len();

    for i in (0..n).rev() {
        for j in 0..m {
            if platform[i][j] == 'O' {
                let mut k = i;
                while k + 1 < n && platform[k + 1][j] == '.' {
                    platform[k][j] = '.';
                    platform[k + 1][j] = 'O';
                    k += 1;
                }
            }
        }
    }
}

fn tilt_west(platform: &mut Vec<Vec<char>>) {
    let n = platform.len();
    let m = platform[0].len();

    for j in 0..m {
        for i in 0..n {
            if platform[i][j] == 'O' {
                let mut k = j;
                while k > 0 && platform[i][k - 1] == '.' {
                    platform[i][k] = '.';
                    platform[i][k - 1] = 'O';
                    k -= 1;
                }
            }
        }
    }
}

fn tilt_east(platform: &mut Vec<Vec<char>>) {
    let n = platform.len();
    let m = platform[0].len();

    for j in (0..m).rev() {
        for i in 0..n {
            if platform[i][j] == 'O' {
                let mut k = j;
                while k + 1 < m && platform[i][k + 1] == '.' {
                    platform[i][k] = '.';
                    platform[i][k + 1] = 'O';
                    k += 1;
                }
            }
        }
    }
}

fn main() {
    let input = std::fs::read("data/day14").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut platform = input
        .split_terminator("\r\n")
        .map(|s| s.chars().collect_vec())
        .collect_vec();

    // for row in &platform {
    //     println!("{}", String::from_iter(row.iter()));
    // }
    // println!();

    let mut cache = HashMap::new();
    for cycle in 0..190 { // Loop size = 9, loop starts at 110
        tilt_north(&mut platform);
        tilt_west(&mut platform);
        tilt_south(&mut platform);
        tilt_east(&mut platform);

        // check repetition
        if let Some(prev_cycle) = cache.get(&platform) {
            println!("Loop detected: cycle {cycle} == cycle {prev_cycle}");
        }
        cache.insert(platform.clone(), cycle);
    }

    // for row in &platform {
    //     println!("{}", String::from_iter(row.iter()));
    // }
    // println!();

    let mut load = 0;
    for i in 0..platform.len() {
        for j in 0..platform[0].len() {
            if platform[i][j] == 'O' {
                load += platform.len() - i;
            }
        }
    }
    println!("load = {load}");
}
