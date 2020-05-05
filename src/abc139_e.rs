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

fn main() {
    let n: usize = read();
    let mut tree: Vec<Vec<Vec<(usize, usize)>>> = vec![vec![vec![]; n + 1]; n + 1];
    for i in 0..n {
        let ii = i + 1;
        let mut cn: (usize, usize) = (0, 0);
        for _ in 0..n - 1 {
            let j: usize = read();
            let a = max(ii, j);
            let b = min(ii, j);
            if cn.0 == 0 && cn.1 == 0 {
                cn = (a, b);
                continue;
            }
            tree[cn.0][cn.1].push((a, b));
            cn = (a, b);
        }
    }

    let mut memo: Vec<Vec<usize>> = vec![vec![0; n + 1]; n + 1];
    let mut maxv = 0;
    for i in 1..n + 1 {
        for j in 1..n {
            let mut cv: usize = 1;
            let a = max(i, j);
            let b = min(i, j);
            if memo[i][j] != 0 {
                cv += memo[a][b];
                maxv = max(maxv, cv);
                continue;
            }
            let m = search((a, b), &tree, &mut cv, &mut memo);
            if m == 0 {
                println!("{}", -1);
                return;
            }
            maxv = max(maxv, m);
        }
    }

    println!("{}", maxv);
}

fn search(
    cn: (usize, usize),
    tree: &Vec<Vec<Vec<(usize, usize)>>>,
    cv: &mut usize,
    memo: &mut Vec<Vec<usize>>,
) -> usize {
    if *cv >= tree.len() {
        return 0;
    }
    if tree[cn.0][cn.1].len() == 0 {
        return *cv;
    }
    // if memo[cn.0][cn.1] != 0 {}

    let mut maxv = cv.clone();
    *cv += 1;

    for i in &tree[cn.0][cn.1] {
        let mut maxvc = cv.clone();
        let result = search((i.0, i.1), tree, &mut maxvc, memo);
        if result == 0 {
            return 0;
        }
        maxv = max(maxv, result);
    }
    memo[cn.0][cn.1] = maxv;
    return maxv;
}
