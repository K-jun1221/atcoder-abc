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

pub fn read_n<T: FromStr>(n: usize) -> Vec<T> {
  read_n_logic::<T>(n, vec![])
}

pub fn read_n_logic<T: FromStr>(n: usize, mut a: Vec<T>) -> Vec<T> {
  match n {
    0 => a,
    _ => {
      a.push(read::<T>());
      read_n_logic(n - 1, a)
    }
  }
}

use std::cmp::{max, min};
use std::collections::BinaryHeap;
use std::collections::HashMap;

fn main() {
  let mut n: i64 = read();
  let mut x: i64 = read();
  let mut ans: i64 = 0;
  // ;ans
  let mut p = vec![0; 51];
  let mut t = vec![0; 51];
  p[1] = 3;
  t[1] = 5;
  for i in 2..51 {
    p[i] = p[i - 1] * 2 + 1;
    t[i] = t[i - 1] * 2 + 3;
  }

  while 0 < n {
    let atumi = t[n as usize - 1];
    let pathi = p[n as usize - 1];

    // println!(
    //   "atumi: {} pathi: {} ans: {} n: {} x: {}",
    //   atumi, pathi, ans, n, x
    // );

    if n == 1 {
      if x == 1 {
        ans += 0;
      } else if 1 < x && x <= 4 {
        ans += x - 1;
      } else {
        ans += 3
      }
      break;
    }

    if x == 1 {
      ans += 0;
      break;
    } else if x == atumi + 2 {
      ans = pathi + 1;
      break;
    } else if x == atumi * 2 + 3 {
      ans = pathi * 2 + 1;
      break;
    } else if 1 < x && x <= atumi + 1 {
      n -= 1;
      x -= 1;
    } else {
      n -= 1;
      x -= 2 + atumi;
      ans += pathi + 1;
    }
  }

  println!("{}", ans);
}
