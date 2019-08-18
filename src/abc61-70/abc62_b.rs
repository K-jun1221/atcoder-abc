
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
  let h = read::<i64>();
  let w = read::<i64>();
  let mut map: Vec<String> = vec![];

  let mut st = String::from("");
  for i in 0..w + 2 {
    st.push_str("#")
  }

  map.push(st);
  for _ in 0..h {
    let mut row_s = String::from("#");
    let row = read::<String>();
    row_s.push_str(&row);
    row_s.push_str("#");
    map.push(row_s);
  }

  let mut st2 = String::from("");
  for i in 0..w + 2 {
    st2.push_str("#")
  }
  map.push(st2);

  for i in map {
    println!("{}", i);
  }
}
