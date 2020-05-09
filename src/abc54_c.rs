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

fn dfs(cn: usize, map: &Vec<Vec<usize>>, cnt: &mut usize, is_done: &mut Vec<bool>) {
    if is_done[cn] {
        return;
    }

    is_done[cn] = true;
    if is_done.iter().all(|&x| x == true) {
        *cnt += 1;
    }
    for i in &map[cn] {
        if is_done[*i] == true {
            continue;
        }
        let mut copy_done = is_done.clone();
        dfs(*i, map, cnt, &mut copy_done);
    }
}

fn main() {
    let n: usize = read();
    let m: usize = read();
    let mut map: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for _ in 0..m {
        let a: usize = read();
        let b: usize = read();
        map[a].push(b);
        map[b].push(a);
    }
    let mut ans = 0;
    let mut is_done = vec![false; n + 1];
    is_done[0] = true;

    dfs(1, &map, &mut ans, &mut is_done);
    println!("{}", ans);
}
