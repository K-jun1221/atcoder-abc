
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
  let mut n = read::<i64>();
  let mut ans = 0;
  let mut ans_max = 0;

  for i in 1..n + 1 {
    let mut k = i.clone();
    let mut cnt = 0;
    while k % 2 == 0 && k != 1 && k != 0 {
      k /= 2;
      cnt += 1;
    }
    if ans_max < cnt {
      ans = i;
      ans_max = cnt;
    }
  }

  if n == 1 {
    println!("{}", 1);
  } else {
    println!("{}", ans);
  }

}
