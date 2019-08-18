
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
  let n = read::<i64>();
  let mut h = vec![];
  for _ in 0..n {
    let h_i = read::<i64>();
    h.push(h_i);
  }

  let mut cnt = 0;
  let mut cant = false;

  for i in 0..n - 1 {
    let idx = i as usize;
    let mut current = h[idx];
    let next = h[idx + 1];
    if current - 1 == next {
      h[idx] = current - 1;
      current = h[idx];
    }

    if current > next {
      cant = true;
      break;
    }

    if idx > 0 {
      let prev = h[idx - 1];

      if prev > current {
        cant = true;
        break;
      }
    }
  }

  if cant {
    println!("No");
  } else {
    println!("Yes");
  }
}

