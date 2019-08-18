
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


fn main() {
  let n = read::<i64>();
  let mut uniq = HashSet::new();
  let mut rainbow = 0;
  for _ in 0..n {
    let a = read::<i64>();
    if 1 <= a && a <= 399 {
      uniq.insert("gray");
    } else if 200 <= a && a <= 799 {
      uniq.insert("brown");
    } else if 800 <= a && a <= 1199 {
      uniq.insert("green");
    } else if 1200 <= a && a <= 1599 {
      uniq.insert("water");
    } else if 1600 <= a && a <= 1999 {
      uniq.insert("blue");
    } else if 2000 <= a && a <= 2399 {
      uniq.insert("yellow");
    } else if 2400 <= a && a <= 2799 {
      uniq.insert("orange");
    } else if 2800 <= a && a <= 3199 {
      uniq.insert("red");
    }

    if 3200 <= a {
      rainbow += 1;
    }
  }
  println!("{} {}", max(1, uniq.len()), uniq.len() + rainbow);
}
