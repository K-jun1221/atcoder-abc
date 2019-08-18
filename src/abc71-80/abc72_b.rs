
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
  let x = read::<String>();
  let mut ans = String::from("");

  for (i, k) in x.chars().enumerate() {
    let nth = i + 1;
    if nth % 2 == 1 {
      ans.push_str(&k.to_string())
    }
  }

  println!("{}", ans);
}

