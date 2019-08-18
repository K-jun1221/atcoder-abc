
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
  let mut a_vec = vec![];
  let mut total = 0;
  let mut subsc = 0;
  let mut ans = 100000000000;

  for _ in 0..n {
    let a = read::<i64>();
    a_vec.push(a);
    total += a;
  }

  for i in 0..n - 1 {
    subsc += a_vec[i as usize];
    total -= a_vec[i as usize];
    ans = min(ans, (total - subsc).abs());
  }

  println!("{}", ans);
}
