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
  let k = read::<i32>();
  let mut h = vec![];
  let mut answers = vec![];

  for _ in 0..n {
    let h_i = read::<i32>();
    h.push(h_i)
  }

  h.sort();

  let mut s_index = 0;
  for i in (k - 1)..n {
    answers.push(h[i as usize] - h[s_index]);
    s_index += 1;
  }


  // println!("{:?}", h);
  // println!("{:?}", answers);
  println!("{:?}", answers.iter().min().unwrap());

}

