
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

fn main() {
  let n = read::<i64>();
  let s = read::<String>();
  let mut max_ans = 0;

  for i in 1..n {
    let usize_i = i as usize;
    let a = &s[0..usize_i];
    let b = &s[usize_i..];
    let mut ans = 0;

    let mut a_vec = vec![];
    for c in a.chars() {
      if !a_vec.contains(&c) {
        a_vec.push(c);
      }
    }

    for i in a_vec {
      if b.contains(&i.to_string()) {
        ans += 1;
      }
    }

    max_ans = max(max_ans, ans)
  }

  println!("{}", max_ans);

}

