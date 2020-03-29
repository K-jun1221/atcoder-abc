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

fn main() {
  let n: i64 = read();
  let x: i64 = read();
  let y: i64 = read();

  let xy = y - x;
  let a = x - 1;
  let b = n - y;

  let mut check = false;

  for i in 0..n {
    if i == 0 {
      println!("{}", n);
      continue;
    }
    let ii = i + 1;
    let mut ans = n - i;

    // +2
    if xy <= ii && ii - 1 <= a && ii - 1 <= b {
      ans += 2
    }

    // +1
    if xy <= ii && ((a < ii - 1) || (b < ii - 1)) {
      ans += 1
    }

    // -1
    if xy == ii {
      ans -= 1
    }

    // -2
    if a + b + xy - ii > 0 {
      ans -= a + b + xy - ii
    }

    println!("{}", ans);
  }
}
