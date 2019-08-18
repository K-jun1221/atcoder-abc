
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
  let mut card = vec![];
  let mut alice = 0;
  let mut bob = 0;
  for i in 0..n {
    card.push(read::<i64>());
  }

  card.sort_by_key(|x| -x);

  for (i, v) in card.iter().enumerate() {
    if i % 2 == 0 {
      alice += *v;
    } else {
      bob += *v;
    }
  }
  println!("{}", alice - bob);
}
