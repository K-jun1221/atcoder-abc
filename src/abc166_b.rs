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
    let k: usize = read();
    let mut ans: usize = 0;
    let mut av: Vec<i64> = vec![0; n + 1];
    av[0] = -1;

    for _ in 0..k {
        let d: usize = read();
        let a: Vec<usize> = read_n(d);
        for i in a {
            av[i] += 1;
        }
    }

    ans = av.iter().filter(|&x| *x == 0).count();
    println!("{}", ans);

}
