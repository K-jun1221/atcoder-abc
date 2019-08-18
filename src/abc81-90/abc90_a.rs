
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
  let mut map: Vec<Vec<char>> = vec![];
  for i in 0..3 {
    let a = read::<String>();
    map.push(a.chars().collect());
  }

  println!("{}{}{}", map[0][0], map[1][1], map[2][2]);
}
