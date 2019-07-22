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
  let a = read::<u64>();
  let b = read::<u64>();
  let k = read::<u64>();
  let mut count = 0;
  let mut ans = 0;

  let m = if a > b { a } else { b };
  for i in (1..m + 1).rev() {
    if a % i == 0 && b % i == 0 {
      count += 1;
      if count == k {
        ans = i;
        break;
      }
    }
  }
  println!("{}", ans)
}

