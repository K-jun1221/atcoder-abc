
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
  let mut idx_7 = 0;
  let mut idx_4 = 0;
  let mut memo = vec![false; (n + 1) as usize];

  while idx_7 <= n {
    idx_4 = idx_7;
    while idx_4 <= n {
      memo[idx_4 as usize] = true;
      idx_4 += 4;
    }
    memo[idx_7 as usize] = true;
    idx_7 += 7
  }

  if memo[n as usize] {
    println!("Yes");
  } else {
    println!("No");
  }

}

