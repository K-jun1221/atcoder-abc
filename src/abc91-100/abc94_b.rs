
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
  let n = read::<i64>();
  let m = read::<i64>();
  let x = read::<i64>();
  let mut m_vec = vec![];

  for _ in 0..m {
    m_vec.push(read::<i64>())
  }

  m_vec.sort();

  let mut to_zero = 0;
  let mut to_n = 0;
  for i in m_vec {
    if i < x {
      to_zero += 1;
    } else {
      to_n += 1;
    }
  }

  println!("{}", min(to_zero, to_n));

}

