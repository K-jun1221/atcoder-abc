
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

// use std::cmp::{max, min};

fn main() {
  let mut n = read::<i32>();
  let mut ans = String::new();

  while n != 1 {
    let amari = n % (-2);
    n = n / (-2);
    if amari == -1 {
      n += 1;
      ans.insert(0, '1');
    } else if amari == 1 {
      ans.insert(0, '1');
    } else {
      ans.insert(0, '0');
    }
  }

  ans.insert(0, '1');
  println!("{}", ans);
}

