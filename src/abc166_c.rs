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

pub fn read_n<T: FromStr>(n: usize) -> Vec<T> {
    read_n_logic::<T>(n, vec![])
}

pub fn read_n_logic<T: FromStr>(n: usize, mut a: Vec<T>) -> Vec<T> {
    match n {
        0 => a,
        _ => {
            a.push(read::<T>());
            read_n_logic(n - 1, a)
        }
    }
}
use std::cmp::{max, min};

fn bfs(map: &Vec<Vec<usize>>, cn: usize, done: &mut Vec<i64>, hv: &Vec<usize>) {
    if done[cn] == -1 {
        return;
    }
    for i in &map[cn] {
        if hv[cn - 1] <= hv[*i - 1] {
            done[cn] = -1;
            break;
        } else {
            done[*i] = -1;
        }
    }
    if done[cn] == 0 {
        done[cn] = 1;
    }
}

fn main() {
    let n: usize = read();
    let m: usize = read();
    let hv: Vec<usize> = read_n(n);
    let mut map: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for _ in 0..m {
        let a: usize = read();
        let b: usize = read();
        map[a].push(b);
        map[b].push(a);
    }

    let mut status: Vec<i64> = vec![0; n + 1];
    status[0] = -1;

    for i in 1..n + 1 {
        bfs(&map, i, &mut status, &hv);
    }
    println!("{:?}", status.iter().filter(|&x| *x == 1).count());
}
