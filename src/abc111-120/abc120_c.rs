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
  let mut ans = 0;
  // let mut stack = 0;
  let mut zero_c = 0;
  let mut one_c = 0;

  let stack_num = s.chars().nth(0).unwrap();

  for i in s.chars() {

    if i == stack_num {
      zero_c += 1;
    } else {
      one_c += 1;
    }
  }

  ans = if zero_c > one_c { one_c } else { zero_c };

  println!("{}", ans * 2);

}

