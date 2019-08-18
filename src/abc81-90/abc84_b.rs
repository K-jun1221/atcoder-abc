
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
  let a = read::<i64>();
  let b = read::<i64>();

  let a_size = a as usize;
  let b_size = b as usize;

  let s = read::<String>();
  let h = &s[a_size..b_size];
  let mut ans = "No";

  if h == "-" {
    let front = &s[..a_size];
    let back = &s[b_size..];
    if front.chars().all(|x| x.is_digit(10)) && back.chars().all(|x| x.is_digit(10)) {
      ans = "Yes"
    } else {
      ans = "No"
    }
  } else {
    ans = "No"
  }

  println!("{}", ans)
}

