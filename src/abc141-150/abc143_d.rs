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

use std::cmp::Ordering;

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

}

use std::cmp::{max, min};
use std::collections::BinaryHeap;

fn main() {
    let n: usize = read();
    let mut a: Vec<i64> = read_n(n);

    a.sort();
    let mut ans = 0;
    for (i, v1) in a.iter().enumerate() {
        for (j, v2) in a.iter().enumerate() {
            if i != j {

                // 一つ目の条件
                let fc = a.lower_bound(&(v1 + v2));

                // 2つ目の条件
                let sc = a.upper_bound(&(v1 - v2).abs());

                let mut cc = fc - sc;
                if i + 1 <= fc && sc < i + 1 {
                    cc -= 1;
                }
                if j + 1 <= fc && sc < j + 1 {
                    cc -= 1;
                }

                ans += cc

            }

        }
    }

    println!("{}", ans / 6)
}
