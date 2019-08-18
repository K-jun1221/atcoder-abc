
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
  let a = read::<i64>();
  let b = read::<i64>();
  let c = read::<i64>();
  let x = read::<i64>();
  let y = read::<i64>();

  if a + b <= 2 as i64 * c {
    println!("{}", x * a + y * b);
  } else {
    let min1 = min(x, y);
    let min2 = min(2 * c, a);
    let min3 = min(2 * c, b);
    println!("{}", min1 * c * 2 + (x - min1) * min2 + (y - min1) * min3);
  }

}

