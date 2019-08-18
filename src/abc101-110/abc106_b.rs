
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

fn count_divisor(n: &u32) -> usize {
  (1..n + 1).filter(|i| n % i == 0).count()
}

fn main() {
  let n = read::<u32>();
  let mut count = 0;

  let ans = (1..n + 1)
    .filter(|x| x % 2 == 1)
    .filter(|x| count_divisor(x) == 8)
    .count();

  println!("{}", ans);
}