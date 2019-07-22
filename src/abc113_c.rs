
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
  let n = read::<usize>();
  let m = read::<i32>();
  let mut pm: Vec<(i32, i32)> = vec![];
  let mut vec_t: Vec<Vec<i32>> = vec![vec![]; n];

  for _ in 0..m {
    let p = read::<i32>();
    let m = read::<i32>();
    pm.push((p, m));
    vec_t[(p - 1) as usize].push(m)
  }

  for i in 0..vec_t.len() {
    vec_t[i].sort();
  }

  for (p, m) in pm {
    let m_num = vec_t[(p - 1) as usize].binary_search(&m).unwrap();
    let p_num: String = format!("{:0>keta$}", p, keta = 6 as usize);
    let m_num: String = format!("{:0>keta$}", m_num + 1, keta = 6 as usize);

    println!("{}{}", p_num, m_num);
  }
}

