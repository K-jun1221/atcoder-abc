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
  let front = &s[..3];
  let back = &s[1..];
  let mut ok = false;
  for i in 0..10 {
    let i_char = i.to_string().chars().nth(0).unwrap();
    if front.chars().all(|x| x == i_char) || back.chars().all(|x| x == i_char) {
      ok = true;
    }
  }

  if ok {
    println!("Yes");
  } else {
    println!("No");
  }
}

