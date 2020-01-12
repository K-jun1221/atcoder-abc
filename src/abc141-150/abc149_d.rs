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
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let n: i64 = read();
    let k: i64 = read();
    let r: i64 = read();
    let s: i64 = read();
    let p: i64 = read();
    let t: String = read();
    let mut ans = 0;
    // let mut vec: Vec<Vec<char>> = vec![vec![]; k as usize];
    let mut dist: HashMap<i64, Vec<char>> = HashMap::new();

    for (i, v) in t.chars().enumerate() {
        let key = (i as i64 + 1) % k;
        if dist.contains_key(&key) {
            let current = dist.get_mut(&key).unwrap();
            current.push(v);
        } else {
            dist.insert(key, vec![v]);
        }
    }

    for (k, v) in dist.iter() {
        let mut prev = '#';
        for i in v {
            if (*i == prev) {
                prev = '#'
            } else {
                match *i {
                    'r' => ans += p,
                    's' => ans += r,
                    'p' => ans += s,
                    _ => (),
                };
                prev = *i;
            }
        }
    }

    println!("{}", ans)
}
