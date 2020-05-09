use std::io::*;
use std::str::FromStr;

pub fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

use std::collections::VecDeque;

fn main() {
    let n: usize = read();
    let m: usize = read();
    // <to, weight, id>
    let mut map: Vec<Vec<(usize, usize, usize)>> = vec![vec![]; n + 1];
    let mut mv: Vec<(usize, usize, usize)> = vec![];

    for i in 0..m {
        let a: usize = read();
        let b: usize = read();
        let c: usize = read();
        map[a].push((b, c, i));
        map[b].push((a, c, i));
        mv.push((a, b, c));
    }

    let mut mapd: Vec<Vec<usize>> = vec![vec![0; n + 1]; n + 1];
    for i in 1..n + 1 {
        // cn, total_weight, edge_id
        let mut vd: VecDeque<(usize, usize, usize)> = VecDeque::new();
        vd.push_front((i, 0, m));
        let mut memo = vec![1_000_000_000; n + 1];
        let mut is_done = vec![false; m];
        loop {
            match vd.pop_front() {
                None => break,
                Some(s) => {
                    // println!("{:?}", s);
                    if s.2 < m {
                        is_done[s.2] = true;
                    }
                    if s.1 < memo[s.0] {
                        memo[s.0] = s.1;
                    }
                    for i in &map[s.0] {
                        if !is_done[i.2] {
                            vd.push_back((i.0, memo[s.0] + i.1, i.2))
                        }
                    }
                }
            }
        }
        for j in 1..n + 1 {
            mapd[i][j] = memo[j];
        }
    }

    let mut cnt = 0;
    for mi in mv {
        let mut flag = false;
        'outer: for i in 1..n + 1 {
            for j in 1..n + 1 {
                if mapd[i][mi.0] + mi.2 + mapd[mi.1][j] == mapd[i][j] {
                    flag = true;
                    break 'outer;
                }
            }
        }
        if !flag {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
