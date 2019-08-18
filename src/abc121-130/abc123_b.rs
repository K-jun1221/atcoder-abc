use std::io::*;
use std::str::FromStr;

use std::cmp::{max, min};

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

fn main() {
  let mut meal = vec![];
  for _ in 0..5 {
    let a = read::<i32>();
    meal.push(a);
  }

  let mut max_res = 0;
  let mut ans = 0;
  for i in meal {
    max_res = max(max_res, (10 - i % 10) % 10);
    ans += i + (10 - i % 10) % 10;
  }

  println!("{}", ans - max_res);

}

