
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
  let n = read::<String>();
  let n_char: Vec<char> = n.chars().collect();
  let a = n_char[0].to_digit(10).unwrap() as i64;
  let b = n_char[1].to_digit(10).unwrap() as i64;
  let c = n_char[2].to_digit(10).unwrap() as i64;
  let d = n_char[3].to_digit(10).unwrap() as i64;

  if a + b + c + d == 7 {
    println!("{}+{}+{}+{}=7", a, b, c, d);
  } else if a + b + c - d == 7 {
    println!("{}+{}+{}-{}=7", a, b, c, d);
  } else if a + b - c + d == 7 {
    println!("{}+{}-{}+{}=7", a, b, c, d);
  } else if a - b + c + d == 7 {
    println!("{}-{}+{}+{}=7", a, b, c, d);
  } else if a - b - c + d == 7 {
    println!("{}-{}-{}+{}=7", a, b, c, d);
  } else if a + b - c - d == 7 {
    println!("{}+{}-{}-{}=7", a, b, c, d);
  } else if a - b + c - d == 7 {
    println!("{}-{}+{}-{}=7", a, b, c, d);
  } else if a - b - c - d == 7 {
    println!("{}-{}-{}-{}=7", a, b, c, d);
  }
}