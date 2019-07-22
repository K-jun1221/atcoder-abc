
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

fn main() {
  let s = read::<String>();
  let k = read::<i64>();
  let mut ans = '1';

  let s_char: Vec<char> = s.chars().collect();

  for i in 0..k {
    if s_char[i as usize] != '1' {
      ans = s_char[i as usize];
      break;
    }
  }

  println!("{}", ans);
}