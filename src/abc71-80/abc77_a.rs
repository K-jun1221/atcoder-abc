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
  let y = read::<String>();
  let x_char: Vec<char> = x.chars().collect();
  let y_char: Vec<char> = y.chars().collect();

  if x_char[0] == y_char[2] && x_char[1] == y_char[1] && x_char[2] == y_char[0] {
    println!("YES");
  } else {
    println!("NO");
  }
}

