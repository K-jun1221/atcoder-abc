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
  let mut antena = vec![];
  for _ in 0..5 {
    let a = read::<i32>();
    antena.push(a);
  }

  let k = read::<i32>();
  let mut answer = true;

  for i in 0.. {
    for j in i + 1..5 {
      let b = antena[i];
      let c = antena[j];
      if c - b > k {
        answer = false;
        break;
      }
    }

  }

  if answer {
    println!("Yay!")
  } else {
    println!(":(")
  }
}

