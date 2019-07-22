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
  let n = read::<u64>();
  let n_str = n.to_string();
  let mut sum = 0;

  sum += dfs(String::from("3"), n_str.len(), n);
  sum += dfs(String::from("5"), n_str.len(), n);
  sum += dfs(String::from("7"), n_str.len(), n);

  println!("{}", sum);
}

fn dfs(num: String, l_keta: usize, limit: u64) -> u64 {
  if num.len() > l_keta {
    return 0;
  }
  let num_i = num.parse::<u64>().unwrap();

  let mut sum = 0;

  if num.contains('3') && num.contains('5') && num.contains('7') && num_i <= limit {
    sum += 1;
  }
  
  let num_3 = num.clone();
  let num_5 = num.clone();
  let num_7 = num.clone();
  sum += dfs(num_3 + "3", l_keta, limit);
  sum += dfs(num_5 + "5", l_keta, limit);
  sum += dfs(num_7 + "7", l_keta, limit);
  sum
}