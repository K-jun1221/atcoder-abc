
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
  let s = read::<String>();
  let t = read::<String>();

  let mut s_str = s.chars().collect::<Vec<char>>();
  let t_str = t.chars().collect::<Vec<char>>();
  let s_len = s_str.len();
  let mut ok = false;

  for i in 0..s_len {
    let si = s_str.pop().unwrap();
    s_str.insert(0, si);

    // println!("{:?}", s_str);
    if t_str == s_str {
      ok = true;
      break;
    }
  }

  if ok {
    println!("Yes");
  } else {
    println!("No");
  }
  // println!(
  //   "{}",
  //   tasks.iter().max().unwrap() - tasks.iter().min().unwrap()
  // );
}

