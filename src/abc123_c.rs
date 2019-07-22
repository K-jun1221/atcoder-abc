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
  let mut tran = vec![];
  let n = read::<i64>();
  for _ in 0..5 {
    let a = read::<i64>();
    tran.push(a);
  }

  let min_v = tran.iter().min().unwrap();

  let mut dev = n / min_v;
  dev += if n % min_v > 0 { 1 } else { 0 };

  println!("{:?}", dev + 4);
}

