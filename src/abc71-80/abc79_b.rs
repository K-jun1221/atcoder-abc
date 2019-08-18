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
  let mut vec_n: Vec<i64> = vec![0; n as usize + 1];

  vec_n[0] = 2;
  vec_n[1] = 1;
  for i in 2..n + 1 {
    let i_size = i as usize;
    vec_n[i_size] = vec_n[i_size - 1] + vec_n[i_size - 2];
  }

  println!("{}", vec_n[n as usize]);
}

