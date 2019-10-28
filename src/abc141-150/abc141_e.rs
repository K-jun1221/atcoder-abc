
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
use std::collections::BinaryHeap;

fn zalgorithm(s: &str) -> Vec<usize> {
    let s_char: Vec<char> = s.chars().collect();
    let mut a = vec![0; s_char.len()];

    a[0] = s_char.len();
    let mut i = 1;
    let mut j = 0;

    while i < s_char.len() {
        while i + j < s_char.len() && s_char[j] == s_char[i + j] {
            j += 1;
        }
        a[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while i + j < s_char.len() && k + a[k] < j {
            k += 1;
        }
        i += k;
        j -= k;
    }

    return a;
}


fn main() {
    let n: usize = read();
    let s: String = read();

    let mut ans = 0;


    for i in 0..n {
        let a = zalgorithm(&s[i..]);
        let mut maybeMax = 0;
        for (idx, ai) in a.iter().enumerate() {
            if idx >= *ai {
                maybeMax = max(maybeMax, *ai)
            }
        }
        ans = max(ans, maybeMax)
    }

    println!("{}", ans);

}
