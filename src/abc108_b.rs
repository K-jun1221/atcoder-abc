
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
  let x1 = read::<i32>();
  let y1 = read::<i32>();
  let x2 = read::<i32>();
  let y2 = read::<i32>();

  let dx = x2 - x1;
  let dy = y2 - y1;

  let y3 = y1 + dy + dx;
  let x3 = x1 + dx - dy;

  let y4 = y1 + dx;
  let x4 = x1 - dy;

  println!("{} {} {} {}", x3, y3, x4, y4);

}

