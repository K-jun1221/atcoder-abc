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
  // 2019/04/30
  let s = read::<String>();

  let year = &s[0..4];
  let month = &s[5..7];

  let year_num = year.parse::<i32>().unwrap();
  let month_num = month.parse::<i32>().unwrap();

  if year_num < 2019 {
    println!("Heisei");
  } else if year_num == 2019 && month_num <= 4 {
    println!("Heisei");
  } else {
    println!("TBD");
  }
}

