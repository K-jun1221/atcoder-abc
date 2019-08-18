
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
  let k = read::<i32>();
  let mut x = vec![];
  for _ in 0..n {
    let x_i = read::<i32>();
    x.push(x_i);
  }

  let mut dist = vec![];

  for i in 0..n - k + 1 {
    let idx = i as usize;
    dist.push(x[idx].abs() + (x[idx + (k - 1) as usize] - x[idx]).abs());
  }

  for i in k - 1..n {
    let idx = i as usize;
    dist.push(x[idx].abs() + (x[idx - (k - 1) as usize] - x[idx]).abs());
  }

  // println!("{:?}", dist);
  println!("{:?}", dist.iter().min().unwrap());
}
