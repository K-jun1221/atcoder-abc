
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
  let mut ans = String::new();

  for i in n.chars() {
    if i == '9' {
      ans.push('1')
    } else if i == '1' {
      ans.push('9')
    } else {
      ans.push(i)
    }
  }

  println!("{}", ans);

}

