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
  let n = read::<i32>();
  let mut ans: f64 = 0.0;

  for _ in 0..n {
    let value = read::<f64>();
    let unit = read::<String>();

    if unit == "BTC" {
      ans += value * 380000.0;
    } else {
      ans += value;
    }
  }

  println!("{}", ans);
}

