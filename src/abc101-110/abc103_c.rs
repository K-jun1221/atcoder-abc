
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
    let ai = read::<i64>();
    a.push(ai);
  }

  let ans: i64 = a.iter().map(|x| x - 1).sum();
  println!("{}", ans);
}

