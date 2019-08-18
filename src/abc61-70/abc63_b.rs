
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
use std::collections::HashSet;


fn main() {
  let s = read::<String>();
  let mut uniq = HashSet::new();

  for c in s.chars() {
    uniq.insert(c);
  }

  if s.len() == uniq.len() {
    println!("yes")
  } else {
    println!("no")
  }
}
