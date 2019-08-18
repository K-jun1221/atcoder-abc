
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

fn main() {
  let n = read::<i32>();
  // let p = vec![];
  let mut count = 0;

  for i in 0..n {
    let p_i = read::<i32>();
    if p_i != i + 1 {
      count += 1;
    }
  }

  if count == 2 || count == 0 {
    println!("YES");
  } else {
    println!("NO");
  }

}

