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
  let n = read::<String>();
  let mut n_char: Vec<char> = n.chars().collect();
  n_char.reverse();
  let mut rev = String::from("");
  for c in n_char {
    rev.push_str(&c.to_string());
  }
  // let s: String = n_char.iter().collect();

  if rev == n {
    println!("Yes");
  } else {
    println!("No");
  }

}

