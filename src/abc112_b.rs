
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
  let t = read::<i32>();
  let mut ans = 1001;

  for _ in 0..n {
    let c = read::<i32>();
    let t_i = read::<i32>();
    if (t >= t_i) {
      ans = min(ans, c);
    }
  }
  if ans == 1001 {
    println!("TLE");
  } else {
    println!("{}", ans);
  }

}

