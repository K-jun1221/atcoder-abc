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
  let b = read::<String>();
  let mut ans = String::new();

  for i in b.chars() {
    if i == 'A' {
      ans.push('T');
    }
    if i == 'C' {
      ans.push('G');
    }
    if i == 'G' {
      ans.push('C');
    }
    if i == 'T' {
      ans.push('A');
    }
  }

  println!("{}", ans);
}

