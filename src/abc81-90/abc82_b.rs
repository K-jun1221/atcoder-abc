
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
use std::collections::BTreeMap;


fn main() {
  let a = read::<String>();
  let b = read::<String>();

  let mut a_vec: Vec<char> = a.chars().collect();
  let mut b_vec: Vec<char> = b.chars().collect();
  let mut ans = "Nutral";

  a_vec.sort();
  b_vec.sort_by(|a, b| b.cmp(a));

  // println!("a:{:?}, b:{:?}", a_vec, b_vec);

  let min_v = min(a_vec.len(), b_vec.len());

  for i in 0..min_v {
    if a_vec[i] > b_vec[i] {
      ans = "No";
      break;
    } else if b_vec[i] > a_vec[i] {
      ans = "Yes";
      break;
    } else {
      continue;
    }
  }

  if ans == "Nutral" {
    if a_vec.len() < b_vec.len() {
      ans = "Yes";
    } else {
      ans = "No";
    }
  }
  println!("{}", ans)
}

