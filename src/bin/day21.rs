use std::collections::HashSet;

use itertools::Itertools;

fn parse_input() -> (Vec<Vec<char>>, (usize, usize)) {
    let input = std::fs::read("data/day21").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut board = input
        .split_terminator("\r\n")
        .map(|s| s.chars().collect_vec())
        .collect_vec();

    let mut starting_point = None;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == 'S' {
                board[i][j] = '.';
                starting_point = Some((i, j))
            }
        }
    }

    (board, starting_point.unwrap())
}

fn solve(board: &Vec<Vec<char>>, starting_point: (usize, usize), steps: usize) -> usize {
    let n = board.len();
    let m = board[0].len();

    let mut cur = vec![starting_point];
    let mut next = vec![];
    let mut vis = HashSet::new();
    for _step in 0..steps {
        while let Some((i, j)) = cur.pop() {
            if !vis.contains(&(i, j)) {
                vis.insert((i, j));

                if i > 0 && board[i - 1][j] == '.' {
                    next.push((i - 1, j));
                }
                if i < n - 1 && board[i + 1][j] == '.' {
                    next.push((i + 1, j));
                }
                if j > 0 && board[i][j - 1] == '.' {
                    next.push((i, j - 1));
                }
                if j < m - 1 && board[i][j + 1] == '.' {
                    next.push((i, j + 1));
                }
            }
        }

        std::mem::swap(&mut cur, &mut next);
    }

    // Add the current reachable positions to vis as well.
    for point in &cur {
        vis.insert(*point);
    }

    let mut answer = 0;
    for &(i, j) in &vis {
        if (i + j + steps) % 2 == (starting_point.0 + starting_point.1) % 2 {
            answer += 1;
        }
    }

    answer
}

fn main() {
    let (board, starting_point) = parse_input();

    let part1 = solve(&board, starting_point, 64);
    dbg!(part1);

    let full_board_even = solve(&board, starting_point, 300); // 7424
    let full_board_odd = solve(&board, starting_point, 301); // 7388

    let top_middle = solve(&board, (0, 65), 130); // 5568
    let bottom_middle = solve(&board, (130, 65), 130); // 5579
    let left_middle = solve(&board, (65, 0), 130); // 5559
    let right_middle = solve(&board, (65, 130), 130); // 5588

    let top_left_big = solve(&board, (0, 0), 130 + 65); // 6475
    let top_left_small = solve(&board, (0, 0), 64); // 933
    let top_right_big = solve(&board, (0, 130), 130 + 65); // 6481
    let top_right_small = solve(&board, (0, 130), 64); // 956
    let bottom_left_big = solve(&board, (130, 0), 130 + 65); // 6472
    let bottom_left_small = solve(&board, (130, 0), 64); // 941
    let bottom_right_big = solve(&board, (130, 130), 130 + 65); // 6495
    let bottom_right_small = solve(&board, (130, 130), 64); // 948

    let mut part2 = 0;
    part2 += full_board_odd * (202299 * 202299);
    part2 += full_board_even * (202300 * 202300);

    part2 += top_middle;
    part2 += bottom_middle;
    part2 += left_middle;
    part2 += right_middle;

    part2 += top_left_big * 202299;
    part2 += top_left_small * 202300;
    part2 += top_right_big * 202299;
    part2 += top_right_small * 202300;
    part2 += bottom_left_big * 202299;
    part2 += bottom_left_small * 202300;
    part2 += bottom_right_big * 202299;
    part2 += bottom_right_small * 202300;

    dbg!(part2);
}
