
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
use std::collections::HashMap;

fn gcd(a: i64, b: i64) -> i64 {
  if a % b == 0 {
    b
  } else {
    gcd(b, a % b)
  }
}

fn lcm(a: i64, b: i64) -> i64 {
  a * b / gcd(a, b)
}

fn main() {
  let n = read::<i64>();
  let mut csf: Vec<Vec<i64>> = vec![];
  for _ in 0..n - 1 {
    let c = read::<i64>();
    let s = read::<i64>();
    let f = read::<i64>();
    csf.push(vec![c, s, f]);
  }

  for i in 0..n - 1 {
    let mut total = 0;
    for j in i..n - 1 {
      let j_size = j as usize;
      if total < csf[j_size][1] {
        total = csf[j_size][1];
        total += csf[j_size][0];
      } else {
        total += csf[j_size][2]
          - if (total % csf[j_size][2]) != 0 {
            (total % csf[j_size][2])
          } else {
            csf[j_size][2]
          };
        total += csf[j_size][0];
      }
    }
    println!("{}", total);
  }
  println!("{}", 0);
}
