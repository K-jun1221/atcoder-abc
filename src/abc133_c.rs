
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
  let mut l = read::<i64>();
  let mut r = read::<i64>();

  let mut a = vec![];

  // if l == 0 {
  //   l = 1;
  // }
  // if r == 0 {
  //   r = 1;
  // }

  let ls = l % 2019;
  let rs = r % 2019;

  for i in ls..rs + 1 {
    for j in i + 1..rs + 1 {
      println!("i:{}, j:{}", i, j);
      a.push((i * j) % 2019);
    }
  }

  let mut ans = a[0];

  for ai in a {
    ans = min(ans, ai)
  }
  
  println!("{:?}", ans);
}

