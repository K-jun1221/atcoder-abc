use std::io::*;
use std::str::FromStr;

use std::collections::BTreeMap;


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
  let n = read::<u64>();
  let mut m = read::<u64>();
  let mut ans = 0;
  let mut bmap: BTreeMap<u64, u64> = BTreeMap::new();

  for _ in 0..n {
    let a = read::<u64>();
    let b = read::<u64>();

    bmap.insert(a, b);
  }

  for (k, v) in bmap {
    if m > v {
      m -= v;
      ans += v * k;
    } else {
      ans += m * k;
      break;
    }
  }

  println!("{}", ans);
}

