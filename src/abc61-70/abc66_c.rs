
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
use std::collections::VecDeque;


fn main() {
  let n = read::<i64>();
  let mut ans = VecDeque::new();

  for i in 0..n {
    let a = read::<i64>();
    if i % 2 != n % 2 {
      ans.push_front(a);
    } else {
      ans.push_back(a);
    }
  }

  for i in ans {
    print!("{} ", i)
  }
}
