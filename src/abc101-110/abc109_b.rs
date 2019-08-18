
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

struct word {
  vec: Vec<char>,
  string: String,
}

fn main() {
  let n = read::<i32>();
  let mut words = vec![];
  let mut used_words = vec![];

  for _ in 0..n {
    let w = read::<String>();
    let w_vec = w.chars().collect::<Vec<char>>();
    words.push(word {
      vec: w_vec,
      string: w,
    });
  }

  let mut prev_last = words[0].vec[0];
  let mut result = true;

  for i in words {
    if i.vec[0] != prev_last || used_words.contains(&i.string) {
      result = false;
      break;
    }
    prev_last = *i.vec.last().unwrap();
    used_words.push(i.string);

  }

  if result {
    println!("Yes");
  } else {
    println!("No");
  }
}

