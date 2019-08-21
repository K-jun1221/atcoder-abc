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
  // let sosuu = vec![0; 100];
  let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
  let mut primes_count = vec![0; primes.len()];

  for i in 0..primes.len() {
    // let mut cnt = 0;
    let prime_mum = primes[i];
    let mut a = n;
    while 0 < a {
      a = a / prime_mum;
      primes_count[i] += a;
    }
  }

  let mut ans = 0;

  let mut more_74 = 0;
  let mut more_24 = 0;
  let mut more_14 = 0;
  let mut more_4 = 0;
  let mut more_2 = 0;

  for i in primes_count {
    if 74 <= i {
      more_74 += 1;
    }
    if 24 <= i {
      more_24 += 1;
    }
    if 14 <= i {
      more_14 += 1;
    }
    if 4 <= i {
      more_4 += 1;
    }
    if 2 <= i {
      more_2 += 1;
    }
  }
  println!(
    "{}",
    more_74
      + more_24 * (more_2 - 1)
      + more_14 * (more_4 - 1)
      + ((more_4 * (more_4 - 1)) / 2) * (more_2 - 2)
  );
}
