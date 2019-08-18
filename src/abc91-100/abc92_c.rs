
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

use std::cmp::{max, min};

fn main() {
  let n = read::<i64>();
  let mut a = vec![];
  for _ in 0..n {
    a.push(read::<i64>());
  }

  a.push(0);
  a.insert(0, 0);
  let mut total = 0;

  for i in 1..n + 2 {
    let idx = i as usize;
    let pred = a[idx - 1];
    let now = a[idx];
    total += (pred - now).abs();
  }

  for i in 1..n + 1 {
    let idx = i as usize;
    let pred = a[idx - 1];
    let now = a[idx];
    let next = a[idx + 1];

    println!(
      "{}",
      total - (pred - now).abs() - (now - next).abs() + (pred - next).abs()
    );
  }
}

