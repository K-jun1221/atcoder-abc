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
  let mut p: Vec<i32> = vec![];

  for _ in 0..n {
    let p_i = read::<i32>();
    p.push(p_i);
  }

  let sum: i32 = p.iter().sum();
  let p_max = p.iter().max().unwrap();


  println!("{}", sum - (p_max / 2));

}

