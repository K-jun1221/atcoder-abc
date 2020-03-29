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
    let n: i64 = read();
    let mut cnt_map: Vec<Vec<usize>> = vec![vec![0; 10]; 10];
    let mut ans = 0;

    if n == 1 {
        println!("{}", 1);
        return;
    }

    for i in 0..n + 1 {
        let ie = i % 10;
        let mut is = i;
        while is >= 10 {
            is = is / 10
        }
        cnt_map[is as usize][ie as usize] += 1;
    }
    // for i in &cnt_map {
    //     println!("{:?}", i);
    // }

    for i in 1..10 {
        for j in 1..10 {
            ans += &cnt_map[i][j] * &cnt_map[j][i];
        }
        // ans -= cnt_map[i][i] * cnt_map[i][i]
    }
    println!("{}", ans);
}
