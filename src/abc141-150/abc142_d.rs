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

fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn trivial_division(n: u64) -> Vec<u64> {
    let mut result = n;
    for i in 2..((n as f64).sqrt() as u64) + 1 {
        if n % i == 0 {
            result = i;
            break;
        }
    }

    if result == n {
        return vec![n];
    } else {
        let mut v1 = vec![result];
        let mut v2 = trivial_division(n / result);

        v1.append(&mut v2);

        return v1;
    }
}

// fn main() {
//     println!("{:?}", trivial_division(238528));
// }

fn main() {
    let a: i64 = read();
    let b: i64 = read();

    let ab = gcd(a, b);

    let mut uniq: HashSet<u64> = HashSet::new();
    let sosuu = trivial_division(ab as u64);

    // println!("{:?}", sosuu);

    let uniq: HashSet<u64> = sosuu.into_iter().collect();

    // println!("{:?}", uniq);

    if uniq.contains(&1) {
        println!("{}", 1);
    } else {
        println!("{}", uniq.len() + 1);
    }

}
