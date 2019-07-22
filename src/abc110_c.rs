
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

use std::collections::HashMap;

fn main() {
  let a = read::<String>();
  let b = read::<String>();

  let a_v = a.chars().collect::<Vec<char>>();
  let b_v = b.chars().collect::<Vec<char>>();

  if check(&a_v, &b_v) && check(&b_v, &a_v) {
    println!("Yes");
  } else {
    println!("No");
  }
}

fn check(a: &Vec<char>, b: &Vec<char>) -> bool {
  let mut map = HashMap::new();
  let mut result = true;

  for (i, v) in a.iter().enumerate() {
    let value = b[i];
    if map.contains_key(&v) {
      if *map.get(&v).unwrap() != value {
        result = false
      }
    } else {
      map.insert(v, value);
    }
  }
  result
}