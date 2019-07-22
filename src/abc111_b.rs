
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
  let n = read::<i32>();

  if 888 < n {
    println!("999");
  } else if 777 < n {
    println!("888");
  } else if 666 < n {
    println!("777");
  } else if 555 < n {
    println!("666");
  } else if 444 < n {
    println!("555");
  } else if 333 < n {
    println!("444");
  } else if 222 < n {
    println!("333");
  } else if 111 < n {
    println!("222");
  } else {
    println!("111");
  }
}

