
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
use std::collections::HashMap;

fn main() {
  let n = read::<usize>();
  let m = read::<usize>();
  let mut map: Vec<Vec<i64>> = vec![vec![0; n]; n];

  for _ in 0..m {
    let a = read::<usize>();
    let b = read::<usize>();
    map[a - 1][b - 1] += 1 as i64;
    map[b - 1][a - 1] += 1 as i64;
  }

  for i in map {
    println!("{}", i.iter().sum::<i64>());
    // println!();
  }
}

