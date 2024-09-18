// Some helpful diagrams:
// https://dreampuf.github.io/GraphvizOnline/#digraph%20G%20%7B%0A%0A0%20%5Blabel%3D%2214%22%5D%0A1%20%5Blabel%3D%2227%22%5D%0A2%20%5Blabel%3D%2219%22%5D%0A3%20%5Blabel%3D%221%22%5D%0A4%20%5Blabel%3D%221%22%5D%0A5%20%5Blabel%3D%2221%22%5D%0A6%20%5Blabel%3D%2219%22%5D%0A7%20%5Blabel%3D%2215%22%5D%0A8%20%5Blabel%3D%221%22%5D%0A9%20%5Blabel%3D%221%22%5D%0A10%20%5Blabel%3D%229%22%5D%0A11%20%5Blabel%3D%221%22%5D%0A12%20%5Blabel%3D%227%22%5D%0A13%20%5Blabel%3D%2235%22%5D%0A14%20%5Blabel%3D%227%22%5D%0A15%20%5Blabel%3D%221%22%5D%0A16%20%5Blabel%3D%227%22%5D%0A17%20%5Blabel%3D%221%22%5D%0A18%20%5Blabel%3D%224%22%5D%0A2%20-%3E%203%0A3%20-%3E%201%0A0%20-%3E%204%0A3%20-%3E%205%0A4%20-%3E%202%0A4%20-%3E%206%0A1%20-%3E%208%0A7%20-%3E%208%0A6%20-%3E%209%0A5%20-%3E%2011%0A8%20-%3E%2012%0A9%20-%3E%2010%0A10%20-%3E%2011%0A11%20-%3E%207%0A9%20-%3E%2013%0A11%20-%3E%2014%0A14%20-%3E%2015%0A12%20-%3E%2017%0A13%20-%3E%2015%0A15%20-%3E%2016%0A16%20-%3E%2017%0A17%20-%3E%2018%0A%0A%7D
// https://dreampuf.github.io/GraphvizOnline/#digraph%20G%20%7B%0A%0A0%20%5Blabel%3D%22start%22%5D%0A1%20%5Blabel%3D%22%22%5D%0A2%20%5Blabel%3D%22%22%5D%0A3%20%5Blabel%3D%22%22%5D%0A4%20%5Blabel%3D%22%22%5D%0A5%20%5Blabel%3D%22%22%5D%0A6%20%5Blabel%3D%22%22%5D%0A7%20%5Blabel%3D%22%22%5D%0A8%20%5Blabel%3D%22end%22%5D%0A2%20-%3E%201%20%5Blabel%3D%2222%22%5D%0A5%20-%3E%203%20%5Blabel%3D%2218%22%5D%0A5%20-%3E%206%20%5Blabel%3D%2210%22%5D%0A1%20-%3E%203%20%5Blabel%3D%2230%22%5D%0A6%20-%3E%207%20%5Blabel%3D%2210%22%5D%0A1%20-%3E%205%20%5Blabel%3D%2224%22%5D%0A4%20-%3E%205%20%5Blabel%3D%2212%22%5D%0A2%20-%3E%204%20%5Blabel%3D%2222%22%5D%0A3%20-%3E%207%20%5Blabel%3D%2210%22%5D%0A4%20-%3E%206%20%5Blabel%3D%2238%22%5D%0A7%20-%3E%208%20%5Blabel%3D%226%22%5D%0A0%20-%3E%202%20%5Blabel%3D%2214%22%5D%0A%0A%7D
// https://dreampuf.github.io/GraphvizOnline/#digraph%20G%20%7B%0A%0A0%20%5Blabel%3D%22190%22%5D%0A1%20%5Blabel%3D%22121%22%5D%0A2%20%5Blabel%3D%22259%22%5D%0A3%20%5Blabel%3D%22295%22%5D%0A4%20%5Blabel%3D%22217%22%5D%0A5%20%5Blabel%3D%22433%22%5D%0A6%20%5Blabel%3D%221%22%5D%0A7%20%5Blabel%3D%2295%22%5D%0A8%20%5Blabel%3D%221%22%5D%0A9%20%5Blabel%3D%22225%22%5D%0A10%20%5Blabel%3D%221%22%5D%0A11%20%5Blabel%3D%221%22%5D%0A12%20%5Blabel%3D%221%22%5D%0A13%20%5Blabel%3D%2253%22%5D%0A14%20%5Blabel%3D%22135%22%5D%0A15%20%5Blabel%3D%22121%22%5D%0A16%20%5Blabel%3D%22151%22%5D%0A17%20%5Blabel%3D%22125%22%5D%0A18%20%5Blabel%3D%22157%22%5D%0A19%20%5Blabel%3D%22117%22%5D%0A20%20%5Blabel%3D%221%22%5D%0A21%20%5Blabel%3D%221%22%5D%0A22%20%5Blabel%3D%22165%22%5D%0A23%20%5Blabel%3D%22107%22%5D%0A24%20%5Blabel%3D%221%22%5D%0A25%20%5Blabel%3D%2271%22%5D%0A26%20%5Blabel%3D%22213%22%5D%0A27%20%5Blabel%3D%221%22%5D%0A28%20%5Blabel%3D%22125%22%5D%0A29%20%5Blabel%3D%221%22%5D%0A30%20%5Blabel%3D%221%22%5D%0A31%20%5Blabel%3D%2223%22%5D%0A32%20%5Blabel%3D%2249%22%5D%0A33%20%5Blabel%3D%22207%22%5D%0A34%20%5Blabel%3D%2293%22%5D%0A35%20%5Blabel%3D%2297%22%5D%0A36%20%5Blabel%3D%22173%22%5D%0A37%20%5Blabel%3D%22111%22%5D%0A38%20%5Blabel%3D%22369%22%5D%0A39%20%5Blabel%3D%221%22%5D%0A40%20%5Blabel%3D%221%22%5D%0A41%20%5Blabel%3D%22195%22%5D%0A42%20%5Blabel%3D%221%22%5D%0A43%20%5Blabel%3D%221%22%5D%0A44%20%5Blabel%3D%2285%22%5D%0A45%20%5Blabel%3D%221%22%5D%0A46%20%5Blabel%3D%221%22%5D%0A47%20%5Blabel%3D%22215%22%5D%0A48%20%5Blabel%3D%22181%22%5D%0A49%20%5Blabel%3D%2271%22%5D%0A50%20%5Blabel%3D%22245%22%5D%0A51%20%5Blabel%3D%2271%22%5D%0A52%20%5Blabel%3D%22169%22%5D%0A53%20%5Blabel%3D%221%22%5D%0A54%20%5Blabel%3D%2299%22%5D%0A55%20%5Blabel%3D%2283%22%5D%0A56%20%5Blabel%3D%22133%22%5D%0A57%20%5Blabel%3D%221%22%5D%0A58%20%5Blabel%3D%22115%22%5D%0A59%20%5Blabel%3D%221%22%5D%0A60%20%5Blabel%3D%22117%22%5D%0A61%20%5Blabel%3D%22183%22%5D%0A62%20%5Blabel%3D%221%22%5D%0A63%20%5Blabel%3D%221%22%5D%0A64%20%5Blabel%3D%2261%22%5D%0A65%20%5Blabel%3D%221%22%5D%0A66%20%5Blabel%3D%22259%22%5D%0A67%20%5Blabel%3D%2263%22%5D%0A68%20%5Blabel%3D%22217%22%5D%0A69%20%5Blabel%3D%2271%22%5D%0A70%20%5Blabel%3D%22105%22%5D%0A71%20%5Blabel%3D%221%22%5D%0A72%20%5Blabel%3D%2259%22%5D%0A73%20%5Blabel%3D%22257%22%5D%0A74%20%5Blabel%3D%221%22%5D%0A75%20%5Blabel%3D%221%22%5D%0A76%20%5Blabel%3D%221%22%5D%0A77%20%5Blabel%3D%22437%22%5D%0A78%20%5Blabel%3D%2277%22%5D%0A79%20%5Blabel%3D%221%22%5D%0A80%20%5Blabel%3D%22119%22%5D%0A81%20%5Blabel%3D%22129%22%5D%0A82%20%5Blabel%3D%221%22%5D%0A83%20%5Blabel%3D%22111%22%5D%0A84%20%5Blabel%3D%22281%22%5D%0A85%20%5Blabel%3D%22209%22%5D%0A86%20%5Blabel%3D%221%22%5D%0A87%20%5Blabel%3D%22113%22%5D%0A88%20%5Blabel%3D%221%22%5D%0A89%20%5Blabel%3D%22163%22%5D%0A90%20%5Blabel%3D%221%22%5D%0A91%20%5Blabel%3D%221%22%5D%0A92%20%5Blabel%3D%2270%22%5D%0A93%20%5Blabel%3D%221%22%5D%0A2%20-%3E%206%0A6%20-%3E%203%0A6%20-%3E%207%0A0%20-%3E%208%0A8%20-%3E%201%0A8%20-%3E%209%0A4%20-%3E%2010%0A10%20-%3E%205%0A10%20-%3E%2013%0A1%20-%3E%2011%0A11%20-%3E%202%0A3%20-%3E%2012%0A12%20-%3E%204%0A11%20-%3E%2014%0A12%20-%3E%2015%0A7%20-%3E%2020%0A13%20-%3E%2021%0A16%20-%3E%2020%0A20%20-%3E%2017%0A18%20-%3E%2021%0A21%20-%3E%2019%0A20%20-%3E%2022%0A21%20-%3E%2023%0A5%20-%3E%2024%0A19%20-%3E%2024%0A24%20-%3E%2026%0A9%20-%3E%2027%0A27%20-%3E%2025%0A27%20-%3E%2028%0A14%20-%3E%2029%0A15%20-%3E%2030%0A25%20-%3E%2029%0A29%20-%3E%2016%0A17%20-%3E%2030%0A30%20-%3E%2018%0A29%20-%3E%2031%0A30%20-%3E%2032%0A31%20-%3E%2039%0A34%20-%3E%2039%0A39%20-%3E%2035%0A28%20-%3E%2040%0A39%20-%3E%2041%0A40%20-%3E%2034%0A40%20-%3E%2038%0A32%20-%3E%2042%0A36%20-%3E%2042%0A42%20-%3E%2037%0A22%20-%3E%2043%0A42%20-%3E%2044%0A23%20-%3E%2045%0A26%20-%3E%2046%0A35%20-%3E%2043%0A43%20-%3E%2036%0A37%20-%3E%2045%0A45%20-%3E%2033%0A33%20-%3E%2046%0A43%20-%3E%2048%0A45%20-%3E%2049%0A46%20-%3E%2047%0A44%20-%3E%2053%0A52%20-%3E%2053%0A53%20-%3E%2050%0A53%20-%3E%2056%0A49%20-%3E%2057%0A50%20-%3E%2057%0A57%20-%3E%2054%0A57%20-%3E%2058%0A41%20-%3E%2059%0A51%20-%3E%2059%0A59%20-%3E%2055%0A59%20-%3E%2060%0A48%20-%3E%2062%0A55%20-%3E%2062%0A62%20-%3E%2052%0A38%20-%3E%2063%0A62%20-%3E%2064%0A47%20-%3E%2065%0A63%20-%3E%2051%0A54%20-%3E%2065%0A63%20-%3E%2061%0A65%20-%3E%2067%0A67%20-%3E%2071%0A69%20-%3E%2071%0A71%20-%3E%2073%0A60%20-%3E%2074%0A56%20-%3E%2075%0A58%20-%3E%2076%0A68%20-%3E%2074%0A74%20-%3E%2072%0A70%20-%3E%2075%0A75%20-%3E%2066%0A66%20-%3E%2076%0A76%20-%3E%2069%0A74%20-%3E%2078%0A64%20-%3E%2079%0A75%20-%3E%2080%0A76%20-%3E%2081%0A72%20-%3E%2079%0A79%20-%3E%2070%0A61%20-%3E%2082%0A79%20-%3E%2083%0A82%20-%3E%2068%0A82%20-%3E%2077%0A78%20-%3E%2086%0A81%20-%3E%2088%0A77%20-%3E%2086%0A86%20-%3E%2087%0A84%20-%3E%2088%0A88%20-%3E%2089%0A80%20-%3E%2090%0A85%20-%3E%2090%0A90%20-%3E%2084%0A83%20-%3E%2091%0A87%20-%3E%2091%0A91%20-%3E%2085%0A73%20-%3E%2093%0A89%20-%3E%2093%0A93%20-%3E%2092%0A%0A%7D
// https://dreampuf.github.io/GraphvizOnline/#digraph%20G%20%7B%0A%0A0%20%5Blabel%3D%22start%22%5D%0A1%20%5Blabel%3D%22%22%5D%0A2%20%5Blabel%3D%22%22%5D%0A3%20%5Blabel%3D%22%22%5D%0A4%20%5Blabel%3D%22%22%5D%0A5%20%5Blabel%3D%22%22%5D%0A6%20%5Blabel%3D%22%22%5D%0A7%20%5Blabel%3D%22%22%5D%0A8%20%5Blabel%3D%22%22%5D%0A9%20%5Blabel%3D%22%22%5D%0A10%20%5Blabel%3D%22%22%5D%0A11%20%5Blabel%3D%22%22%5D%0A12%20%5Blabel%3D%22%22%5D%0A13%20%5Blabel%3D%22%22%5D%0A14%20%5Blabel%3D%22%22%5D%0A15%20%5Blabel%3D%22%22%5D%0A16%20%5Blabel%3D%22%22%5D%0A17%20%5Blabel%3D%22%22%5D%0A18%20%5Blabel%3D%22%22%5D%0A19%20%5Blabel%3D%22%22%5D%0A20%20%5Blabel%3D%22%22%5D%0A21%20%5Blabel%3D%22%22%5D%0A22%20%5Blabel%3D%22%22%5D%0A23%20%5Blabel%3D%22%22%5D%0A24%20%5Blabel%3D%22%22%5D%0A25%20%5Blabel%3D%22%22%5D%0A26%20%5Blabel%3D%22%22%5D%0A27%20%5Blabel%3D%22%22%5D%0A28%20%5Blabel%3D%22%22%5D%0A29%20%5Blabel%3D%22%22%5D%0A30%20%5Blabel%3D%22%22%5D%0A31%20%5Blabel%3D%22%22%5D%0A32%20%5Blabel%3D%22%22%5D%0A33%20%5Blabel%3D%22%22%5D%0A34%20%5Blabel%3D%22%22%5D%0A35%20%5Blabel%3D%22end%22%5D%0A3%20-%3E%207%20%5Blabel%3D%2256%22%5D%0A31%20-%3E%2034%20%5Blabel%3D%22166%22%5D%0A13%20-%3E%2022%20%5Blabel%3D%22372%22%5D%0A16%20-%3E%2019%20%5Blabel%3D%2274%22%5D%0A26%20-%3E%2027%20%5Blabel%3D%22262%22%5D%0A8%20-%3E%2017%20%5Blabel%3D%22216%22%5D%0A29%20-%3E%2025%20%5Blabel%3D%22220%22%5D%0A0%20-%3E%202%20%5Blabel%3D%22190%22%5D%0A23%20-%3E%2024%20%5Blabel%3D%2266%22%5D%0A1%20-%3E%206%20%5Blabel%3D%2298%22%5D%0A12%20-%3E%2015%20%5Blabel%3D%22100%22%5D%0A19%20-%3E%2027%20%5Blabel%3D%22118%22%5D%0A28%20-%3E%2026%20%5Blabel%3D%22108%22%5D%0A13%20-%3E%2012%20%5Blabel%3D%2296%22%5D%0A29%20-%3E%2030%20%5Blabel%3D%22440%22%5D%0A28%20-%3E%2033%20%5Blabel%3D%22114%22%5D%0A2%20-%3E%204%20%5Blabel%3D%22124%22%5D%0A15%20-%3E%2014%20%5Blabel%3D%22176%22%5D%0A16%20-%3E%2017%20%5Blabel%3D%22210%22%5D%0A2%20-%3E%209%20%5Blabel%3D%22228%22%5D%0A1%20-%3E%205%20%5Blabel%3D%22298%22%5D%0A18%20-%3E%2026%20%5Blabel%3D%22136%22%5D%0A12%20-%3E%2020%20%5Blabel%3D%22198%22%5D%0A21%20-%3E%2018%20%5Blabel%3D%22172%22%5D%0A5%20-%3E%2011%20%5Blabel%3D%22124%22%5D%0A33%20-%3E%2032%20%5Blabel%3D%22212%22%5D%0A5%20-%3E%203%20%5Blabel%3D%22220%22%5D%0A18%20-%3E%2019%20%5Blabel%3D%22248%22%5D%0A6%20-%3E%2015%20%5Blabel%3D%22168%22%5D%0A11%20-%3E%207%20%5Blabel%3D%22160%22%5D%0A14%20-%3E%2016%20%5Blabel%3D%22114%22%5D%0A27%20-%3E%2031%20%5Blabel%3D%22132%22%5D%0A10%20-%3E%206%20%5Blabel%3D%22154%22%5D%0A21%20-%3E%2028%20%5Blabel%3D%2264%22%5D%0A20%20-%3E%2021%20%5Blabel%3D%2286%22%5D%0A14%20-%3E%2018%20%5Blabel%3D%2288%22%5D%0A34%20-%3E%2035%20%5Blabel%3D%2272%22%5D%0A24%20-%3E%2034%20%5Blabel%3D%22260%22%5D%0A32%20-%3E%2031%20%5Blabel%3D%22284%22%5D%0A9%20-%3E%2010%20%5Blabel%3D%2274%22%5D%0A26%20-%3E%2032%20%5Blabel%3D%22122%22%5D%0A30%20-%3E%2033%20%5Blabel%3D%22116%22%5D%0A10%20-%3E%2012%20%5Blabel%3D%2226%22%5D%0A27%20-%3E%2024%20%5Blabel%3D%2274%22%5D%0A7%20-%3E%2016%20%5Blabel%3D%22110%22%5D%0A7%20-%3E%208%20%5Blabel%3D%22120%22%5D%0A25%20-%3E%2028%20%5Blabel%3D%2262%22%5D%0A15%20-%3E%2021%20%5Blabel%3D%22184%22%5D%0A25%20-%3E%2030%20%5Blabel%3D%2280%22%5D%0A3%20-%3E%208%20%5Blabel%3D%22436%22%5D%0A17%20-%3E%2023%20%5Blabel%3D%22218%22%5D%0A22%20-%3E%2020%20%5Blabel%3D%2274%22%5D%0A9%20-%3E%2013%20%5Blabel%3D%22128%22%5D%0A6%20-%3E%2011%20%5Blabel%3D%22128%22%5D%0A20%20-%3E%2025%20%5Blabel%3D%22120%22%5D%0A22%20-%3E%2029%20%5Blabel%3D%22186%22%5D%0A4%20-%3E%201%20%5Blabel%3D%22262%22%5D%0A19%20-%3E%2023%20%5Blabel%3D%22102%22%5D%0A11%20-%3E%2014%20%5Blabel%3D%2252%22%5D%0A4%20-%3E%2010%20%5Blabel%3D%22138%22%5D%0A%0A%7D

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Point = (usize, usize);
type Path = HashSet<Point>;
type Edge = (usize, usize); // (source_path_index, target_path_index)

fn parse_input() -> Vec<Vec<char>> {
    let input = std::fs::read("data/day23").unwrap();
    let input = String::from_utf8(input).unwrap();

    input
        .split_terminator("\r\n")
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn find_all_paths(board: &Vec<Vec<char>>) -> Vec<Path> {
    let n = board.len();
    let m = board[0].len();

    let mut paths = vec![];
    let mut vis = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            let mut path = HashSet::new();
            let mut todos = vec![];
            todos.push((i, j));
            while let Some((x, y)) = todos.pop() {
                if !vis.contains(&(x, y)) && board[x][y] == '.' {
                    vis.insert((x, y));
                    path.insert((x, y));

                    if x > 0 {
                        todos.push((x - 1, y));
                    }
                    if x < n - 1 {
                        todos.push((x + 1, y));
                    }
                    if y > 0 {
                        todos.push((x, y - 1));
                    }
                    if y < m - 1 {
                        todos.push((x, y + 1));
                    }
                }
            }
            if !path.is_empty() {
                paths.push(path);
            }
        }
    }

    paths
}

fn build_edges_between_paths(board: &Vec<Vec<char>>, paths: &Vec<Path>) -> HashSet<Edge> {
    let n = board.len();
    let m = board[0].len();

    // build a reverse mapping from point to path
    let mut mapping = HashMap::new();
    for i in 0..paths.len() {
        for point in &paths[i] {
            mapping.insert(*point, i);
        }
    }

    let mut edges = HashSet::new();
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if ['^', '>', 'v', '<'].contains(&board[i][j]) {
                let (src, tgt) = if board[i][j] == '^' {
                    ((i + 1, j), (i - 1, j))
                } else if board[i][j] == 'v' {
                    ((i - 1, j), (i + 1, j))
                } else if board[i][j] == '<' {
                    ((i, j + 1), (i, j - 1))
                } else if board[i][j] == '>' {
                    ((i, j - 1), (i, j + 1))
                } else {
                    unreachable!();
                };

                edges.insert((mapping[&src], mapping[&tgt]));
            }
        }
    }

    edges
}

/// Find the largest sum in the directed graph, starting from `start`.
fn largest_sum(
    start: usize,
    end: usize,
    paths: &Vec<Path>,
    edges: &HashSet<Edge>,
    cache: &mut HashMap<usize, usize>,
) -> usize {
    // Check cached result.
    if let Some(result) = cache.get(&start) {
        return *result;
    }

    if start == end {
        return paths[start].len();
    }

    let mut result = 0;
    for i in 0..paths.len() {
        if edges.contains(&(start, i)) {
            // Going through icey tiles counts as an additional step, therefore `+1`.
            result = result.max(paths[start].len() + largest_sum(i, end, paths, edges, cache) + 1);
        }
    }

    // Update cached result.
    cache.insert(start, result);

    result
}

// /// Find the largest sum in the undirected graph, starting from `start`.
// /// `current_sum` is the sum of nodes/edges we've visited, not including the
// /// current vertex. Returns None if the end cannot be reached.
// fn search(
//     start: usize,
//     end: usize,
//     current_sum: usize,
//     vis: &mut HashSet<usize>,
//     paths: &Vec<Path>,
//     edges: &HashSet<Edge>,
// ) -> Option<usize> {
//     if start == end {
//         return Some(current_sum + paths[end].len());
//     }
//     vis.insert(start);
//     let mut answer = None;
//     for i in 0..paths.len() {
//         if !vis.contains(&i) && (edges.contains(&(start, i)) || edges.contains(&(i, start))) {
//             if let Some(candidate) = search(
//                 i,
//                 end,
//                 current_sum + paths[start].len() + 1,
//                 vis,
//                 paths,
//                 edges,
//             ) {
//                 let prev = answer.unwrap_or(0);
//                 answer = Some(usize::max(prev, candidate));
//             }
//         }
//     }
//     vis.remove(&start);
//     answer
// }

/// Reduce the problem to the Elementary Longest Path Problem. The resulting
/// graph is stored as an adjencency list, where dummy vertexes are added for
/// the starting/ending vertexes at the first/last index.
fn convert_to_edge_weighted_graph(
    board: &Vec<Vec<char>>,
    paths: &Vec<Path>,
    edges: &HashSet<Edge>,
) -> HashMap<Edge, i64> {
    // Map single-tile-paths to new vertex index. The first and last indexes are reserved.
    let mut idx2idx = HashMap::new();
    for i in 0..paths.len() {
        if paths[i].len() == 1 {
            idx2idx.insert(i, idx2idx.len() + 1);
        }
    }

    let mut result = HashMap::new();

    // Add edges for the dummy start/end vertex.
    let n = board.len();
    let m = board[0].len();
    for i in 0..paths.len() {
        if paths[i].contains(&(0, 1)) {
            for j in 0..paths.len() {
                if edges.contains(&(i, j)) {
                    // `+0` because the starting tile doesn't count, which
                    // counters the extra edge
                    result.insert((0, idx2idx[&j]), paths[i].len() as i64);
                }
            }
        }
        if paths[i].contains(&(n - 1, m - 2)) {
            for j in 0..paths.len() {
                if edges.contains(&(j, i)) {
                    // `+2` because of an extra edge and a vertex of value 1
                    result.insert(
                        (idx2idx[&j], idx2idx.len() + 1),
                        (paths[i].len() + 2) as i64,
                    );
                }
            }
        }
    }

    // Add the rest of the edges.
    for (&old_index1, &new_index1) in &idx2idx {
        for (&old_index2, &new_index2) in &idx2idx {
            for path_index in 0..paths.len() {
                if edges.contains(&(old_index1, path_index))
                    && edges.contains(&(path_index, old_index2))
                {
                    // `+3` because there're two edges and a vertex of value 1
                    result.insert(
                        (new_index1, new_index2),
                        (paths[path_index].len() + 3) as i64,
                    );
                }
            }
        }
    }

    result
}

fn main() {
    let board = parse_input();
    let n = board.len();
    let m = board[0].len();

    let paths = find_all_paths(&board);
    let edges = build_edges_between_paths(&board, &paths);

    // Visualize the graph.
    // for i in 0..paths.len() {
    //     println!("{} [label=\"{}\"]", i, paths[i].len());
    // }
    // for (src, tgt) in &edges {
    //     println!("{src} -> {tgt}");
    // }

    let mut end = 0;
    for i in 0..paths.len() {
        if paths[i].contains(&(n - 1, m - 2)) {
            // This is the goal vertex.
            end = i;
        }
    }

    let mut cache = HashMap::new();
    let part1 = largest_sum(0, end, &paths, &edges, &mut cache) - 1; // the starting point doesn't count as a step
    dbg!(part1);

    let graph = convert_to_edge_weighted_graph(&board, &paths, &edges);

    // Visualize the new graph.
    // let n = graph.keys().map(|&(x, y)| usize::max(x, y)).max().unwrap(); // index of the ending vertex
    // println!("0 [label=\"start\"]");
    // for i in 1..=n - 1 {
    //     println!("{i} [label=\"\"]");
    // }
    // println!("{n} [label=\"end\"]");
    // for (&(i, j), &weight) in &graph {
    //     println!("{i} -> {j} [label=\"{weight}\"]");
    // }

    // Dump the graph in Metis/Chaco format.
    let n = graph.keys().map(|&(x, y)| usize::max(x, y)).max().unwrap() + 1;
    let m = graph.len();
    println!("{n} {m} 11");
    for i in 0..n {
        print!("0");
        for j in 0..n {
            if let Some(weight) = graph.get(&(i, j)) {
                print!(" {} {weight}", j + 1);
            }
            if let Some(weight) = graph.get(&(j, i)) {
                print!(" {} {weight}", j + 1);
            }
        }
        println!();
    }

    // Now use an external program to solve the Elementary Longest Path Problem.
    // I used https://github.com/KarlsruheLongestPaths/KaLP.
}
