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

fn main() {
    let n: usize = read();
    let m: i64 = read();
    let mut a: Vec<i64> = read_n(n);
    let search_max = 2 * a.iter().max().unwrap() + 1;

    a.sort();

    let criterion = |aa: i64| -> bool {
        let mut found = 0;
        for i in 0..n {
            let search = a.lower_bound(&(aa - a[i]));
            found += (a.len() - search) as i64;
        }
        found >= m
    };

    let (ok, ng) = binary_search(0, search_max, criterion);

    
    let aa = ok;
    let mut ans = 0;
    let mut found = 0;

    for i in 0..n {
        let search = a.lower_bound(&(aa - a[i]));
        ans += (a.len() - search) as i64 * a[i] * 2;
        found += (a.len() - search) as i64;
    }
    ans -= ok * (found - m);

    println!("{}", ans);
}

fn binary_search<F>(lb: i64, ub: i64, criterion: F) -> (i64, i64)
where
    F: Fn(i64) -> bool,
{
    let mut ok = lb;
    let mut ng = ub;
    while ng - ok > 1 {
        let mid = (ng + ok) / 2;
        if criterion(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    (ok, ng)
}
