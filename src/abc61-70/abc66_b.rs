
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

  let mut idx = s.len() - 1;
  while 0 < idx {
    let s_str = &s[0..idx];
    if s_str.len() % 2 == 0 {
      if &s[0..(idx / 2)] == &s[(idx / 2)..idx] {
        println!("{}", s_str.len());
        break;
      }
    }
    idx -= 1;
  }
}
