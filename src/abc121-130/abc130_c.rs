use std::cmp;

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

fn main() {
  let w = read::<u32>();
  let h = read::<u32>();
  let x = read::<u32>();
  let y = read::<u32>();

  let way = if x * 2 == w && y * 2 == h { '1' } else { '0' };
  println!("{} {}", (w as f64 * h as f64) / 2.0, way);
}