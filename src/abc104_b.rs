
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
  let s = read::<String>();
  let s_str = s.chars().collect::<Vec<char>>();
  let s_len = s_str.len();

  let check1 = s_str[0] == 'A';
  let check2 = s_str[2..s_len - 3].iter().filter(|&x| x == &'C').count() == 1;
  let check3 = s_str[0..s_len]
    .iter()
    .filter(|&x| x.is_lowercase() || x == &'C' || x == &'A')
    .count()
    == s_len;

  // println!("{} {} {}", check1, check2, check3);

  if check1 && check2 && check3 {
    println!("AC");
  } else {
    println!("WA");
  }
}

