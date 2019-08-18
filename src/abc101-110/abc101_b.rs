
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

  let n_str = n.to_string();
  let n_char: Vec<char> = n_str.chars().collect();

  let mut a: i64 = 0;
  for i in n_char {
    a += i.to_string().parse::<i64>().unwrap();
  }

  // println!("{} {} ", a, n);

  if n % a == 0 {
    println!("Yes");
  } else {
    println!("No");
  }

}

