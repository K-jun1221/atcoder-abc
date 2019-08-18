
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
  let s = read::<i32>();
  let mut s_list = vec![s];
  let mut ans = 0;

  for i in 2..10000000 {
    let s_c = s_list[i - 2].clone();
    let new_s = if s_c % 2 == 0 { s_c / 2 } else { 3 * s_c + 1 };
    if s_list.contains(&new_s) {
      ans = i;
      break;
    }

    s_list.push(new_s);
  }

  println!("{:?}", ans)

}

