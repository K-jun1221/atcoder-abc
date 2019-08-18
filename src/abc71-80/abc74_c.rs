
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
  let a = read::<i64>();
  let b = read::<i64>();
  let c = read::<i64>();
  let d = read::<i64>();
  let e = read::<i64>();
  let f = read::<i64>();

  let mut dp_s = vec![false; f as usize + 10];
  for i in c..f + 1 {
    let i_size = i as usize;
    dp_s[i_size] = if dp_s[i_size - c as usize] {
      true
    } else {
      false
    };
  }

  for i in d..f + 1 {
    let i_size = i as usize;
    dp_s[i_size] = if dp_s[i_size - d as usize] {
      true
    } else {
      false
    };
  }

  let mut dp_w = vec![false; f as usize + 10];
  for i in a * 100..f + 1 {
    let i_size = i as usize;
    dp_s[i_size] = if dp_s[i_size - c as usize] {
      true
    } else {
      false
    };
  }

  for i in d..f + 1 {
    let i_size = i as usize;
    dp_s[i_size] = if dp_s[i_size - d as usize] {
      true
    } else {
      false
    };
  }


  println!("{:?}", dp_s);
}

