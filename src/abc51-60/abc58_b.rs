
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
  let o = read::<String>();
  let e = read::<String>();

  let o_vec: Vec<char> = o.chars().collect();
  let e_vec: Vec<char> = e.chars().collect();

  let o_len = o_vec.len();
  let e_len = e_vec.len();

  let mut ans = String::from("");

  for i in 0..min(o_len, e_len) {
    ans.push(o_vec[i]);
    ans.push(e_vec[i]);
  }

  if e_len < o_len {
    ans.push(o_vec[o_len as usize - 1]);
  }

  println!("{}", ans);
}