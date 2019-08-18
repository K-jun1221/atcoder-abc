
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
  let a = read::<String>();
  let b = read::<String>();
  let c = read::<String>();
  let A: Vec<char> = a.chars().nth(0).unwrap().to_uppercase().collect();
  let B: Vec<char> = b.chars().nth(0).unwrap().to_uppercase().collect();
  let C: Vec<char> = c.chars().nth(0).unwrap().to_uppercase().collect();

  let mut ans = String::from("");
  ans.push(A[0]);
  ans.push(B[0]);
  ans.push(C[0]);
  println!("{}", ans);
}