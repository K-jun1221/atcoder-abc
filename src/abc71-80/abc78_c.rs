
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
  let n = read::<i64>();
  let m = read::<u32>();
  let two: i32 = 2;

  println!(
    "{}",
    ((1900 * m) + (100 * (n - m as i64) as u32)) as u32 * two.pow(m) as u32
  );
}