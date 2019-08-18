
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
// use std::collections::BinaryHeap;

fn main() {
  let n = read::<i64>();
  let k = read::<i64>();
  let mut l_vec = vec![];
  let mut ans = 0;

  for _ in 0..n {
    l_vec.push(read::<i64>())
  }

  l_vec.sort_by_key(|x| -x);

  for i in 0..k {
    ans += l_vec[i as usize];
  }

  println!("{}", ans);
}
