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
  let mut ans = 0;

  let mut count = 0;
  for i in b.chars() {
    if i == 'A' || i == 'C' || i == 'G' || i == 'T' {
      count += 1;
    } else {
      count = 0;
    }

    if count > ans {
      ans = count;
    }
  }

  println!("{}", ans);
}

