
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

// use std::cmp::{max, min};

fn main() {
  let n = read::<usize>();
  let k = read::<usize>();
  let mut ans = (n / k) * (n / k) * (n / k);
  if k % 2 == 0 {
    let cnt = (n + k / 2) / k;
    ans += cnt * cnt * cnt;
  }
  println!("{}", ans);
}

