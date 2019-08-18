
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
// use std::collections::BinaryHeap;

fn main() {
  let s = read::<String>();
  let s_char: Vec<char> = s.chars().collect();
  let mut ans = String::from("");
  ans.push_str(&s_char[0].to_string());
  let s_len = s.len() as usize;
  ans.push_str(&(s_len-2).to_string());
  ans.push_str(&s_char[s_len-1].to_string());

  println!("{}", ans);

}
