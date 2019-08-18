
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
use std::collections::HashSet;

fn divisors(N: i64) -> HashSet<i64> {
  let mut ds = vec![];
  let mut d = 1;
  while d * d <= N {
    if N % d == 0 {
      ds.push(d);
      ds.push(N / d);
    }
    d += 1;
  }
  let uniq: HashSet<i64> = ds.into_iter().collect();
  uniq
}

fn main() {
  let n = read::<i64>();
  let yakusuu = divisors(n);
  let mut y_vec = vec![];

  let mut ans = 100000000000;

  for i in yakusuu.iter() {
    y_vec.push(i);
  }
  y_vec.sort();
  let harf = y_vec.len() / 2;
  for i in 0..harf {
    let a = y_vec[i as usize];
    let b = y_vec[i as usize + harf];

    let a_len = a.to_string().len();
    let b_len = b.to_string().len();

    ans = min(ans, max(b_len, a_len));
  }
  if n == 1 {
    println!("{}", 1);
  } else {
    println!("{}", ans);
  }

}

