
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
  let n = read::<i64>();
  let mut a_vec = vec![];
  for _ in 0..n {
    a_vec.push(read::<i64>())
  }

  let mut current = 0;
  let mut cost = 0;
  while current != 1 && cost <= n {
    let next = a_vec[current];
    cost += 1;
    current = next as usize - 1;
  }

  println!("{}", if cost == n + 1 { -1 } else { cost });
}
