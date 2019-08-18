
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
  let s = read::<String>();
  let mut dist: HashMap<char, i64> = HashMap::new();

  for c in s.chars() {
    let current = dist.entry(c).or_insert(0);
    *current += 1;
  }

  if dist.len() == 3
    && dist.get(&'a').unwrap() == &1
    && dist.get(&'b').unwrap() == &1
    && dist.get(&'c').unwrap() == &1
  {
    println!("Yes");
  } else {
    println!("No");
  }
}

