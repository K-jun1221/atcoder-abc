
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
  // let mut a = vec![];
  let mut cnt = 0;
  for _ in 0..n {
    let mut a_i = read::<i64>();
    while a_i % 2 == 0 {
      a_i = a_i / 2;
      cnt += 1;
    }
  }

  println!("{}", cnt);

}