
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

  if n == 1 && m == 1 {
    println!("{}", 1);
  } else if n == 1 {
    println!("{}", m - 2);
  } else if m == 1 {
    println!("{}", n - 2); 
  } else {
    let coner = 4;
    let edge = 2 * (n - 2) + 2 * (m - 2);
    println!("{}", n * m - coner - edge);
  }

}
