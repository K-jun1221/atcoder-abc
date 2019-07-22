
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
  let d = read::<i32>();
  if (d == 22) {
    println!("Christmas Eve Eve Eve");
  }

  if (d == 23) {
    println!("Christmas Eve Eve");
  }


  if (d == 24) {
    println!("Christmas Eve");
  }

  if (d == 25) {
    println!("Christmas");
  }

}

