
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
  let t = read::<f64>();
  let a = read::<f64>();

  let mut h = vec![];

  for _ in 0..n {
    let h_i = read::<f64>();

    let temp = (a - (t - (h_i * 0.006))).abs();
    h.push(temp);
  }

  let mut min_v: f64 = 100000.0;
  let mut ans = 0;

  for (i, v) in h.iter().enumerate() {
    if v < &min_v {
      min_v = *v;
      ans = i + 1;
    }
  }

  println!("{}", ans);

  // println!("{}", x + (y / 2))
}

