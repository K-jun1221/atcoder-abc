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
  let front: &str = &s[0..2];
  let back: &str = &s[2..];

  let f_num: i32 = front.parse().unwrap();
  let b_num: i32 = back.parse().unwrap();

  if within_month(f_num) && within_month(b_num) {
    println!("AMBIGUOUS");
  } else if within_month(f_num) {
    println!("MMYY");
  } else if within_month(b_num) {
    println!("YYMM");
  } else {
    println!("NA");
  }

}

fn within_month(num: i32) -> bool {
  return num <= 12 && num >= 1;
}
