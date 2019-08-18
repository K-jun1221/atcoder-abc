
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
  let mut dist: HashMap<i64, i64> = HashMap::new();
  let mut memo = vec![0; n as usize + 1];

  for i in 2..n + 1 {
    let mut x = i;
    for j in 2..i + 1 {
      while 1 < x && x % j == 0 {
        x /= j;
        memo[j as usize] += 1;
      }
    }
  }

  let mut ans: u64 = 1;

  for i in memo {
    if i != 0 {
      ans *= i + 1;
      ans %= 1000000007;
    }
  }

  println!("{:?}", ans % 1000000007);
}

