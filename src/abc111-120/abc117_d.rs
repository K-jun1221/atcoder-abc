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
use std::collections::VecDeque;

fn main() {
  let n: usize = read();
  let k: i64 = read();
  let a: Vec<i64> = read_n(n);
  let mut keta_memo = vec![0; 42];

  for i in &a {
    let result: String = format!("{:0>keta$b}", i, keta = 42);
    // println!("{}", result);
    for (i, c) in result.chars().enumerate() {
      if c == '1' {
        keta_memo[i] += 1;
      }
    }
  }

  let mut keta: i64 = 1;
  let mut cnt = 0;
  let mut ans = 0;
  let max_v = max(k, *a.iter().max().unwrap());
  while keta < max_v {
    keta *= 2;
    cnt += 1;
  }

  for i in 0..cnt {
    let rev_idx: usize = 42 - i - 1;
    let two: u64 = 2;
    if k > two.pow(i as u32) as i64 {
      ans +=
        two.pow(i as u32) as i64 * max(keta_memo[rev_idx], a.len() as i64 - keta_memo[rev_idx]);
    } else {
      ans += two.pow(i as u32) as i64 * keta_memo[rev_idx]
    }

  }

  println!("{:?}", ans);
}
