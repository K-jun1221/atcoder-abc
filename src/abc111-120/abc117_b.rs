
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
  let mut a = vec![];
  for _ in 0..n {
    let a_i = read::<i32>();
    a.push(a_i);
  }

  let max_a = a.iter().max().unwrap();
  let sum: i32 = a.iter().sum();

  if *max_a < sum - *max_a {
    println!("Yes");
  } else {
    println!("No")
  }
}

