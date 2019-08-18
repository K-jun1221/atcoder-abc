
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
  let a = read::<String>();
  let b = read::<String>();

  let a_len = a.len();
  let b_len = b.len();

  let a_char: Vec<char> = a.chars().collect();
  let b_char: Vec<char> = b.chars().collect();
  let mut ans = "";

  if a_len < b_len {
    ans = "LESS"
  } else if b_len < a_len {
    ans = "GREATER"
  } else {
    for i in 0..a_len {
      if a_char[i] == b_char[i] {
        if i == a_len - 1 {
          ans = "EQUAL";
          break;
        }
        continue;
      } else {
        if a_char[i].to_digit(10).unwrap() < b_char[i].to_digit(10).unwrap() {
          ans = "LESS"
        } else {
          ans = "GREATER"
        }
        break;
      }
    }
  }

  println!("{}", ans);

}