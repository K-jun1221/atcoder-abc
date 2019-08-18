
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

use std::cmp::{max, min};

fn main() {
  let h = read::<i64>();
  let w = read::<i64>();

  let mut ans = 10000000;

  for i in 1..w {
    let a = h * i;

    // 縦方向に2分割
    let b1 = h * (((w - i) + (w - i) % 2) / 2);
    let c1 = h * (((w - i) - (w - i) % 2) / 2);
    let max1_v = max(a, max(b1, c1));
    let min1_v = min(a, min(b1, c1));
    ans = min(max1_v - min1_v, ans);

    // 縦方向 -> 横方向
    let b2 = (w - i) * ((h + (h % 2)) / 2);
    let c2 = (w - i) * ((h - (h % 2)) / 2);
    let max2_v = max(a, max(b2, c2));
    let min2_v = min(a, min(b2, c2));
    ans = min(max2_v - min2_v, ans);
  }

  for i in 1..h {
    let a = w * i;

    // 横方向に2分割
    let b1 = w * (((h - i) + (h - i) % 2) / 2);
    let c1 = w * (((h - i) - (h - i) % 2) / 2);
    let max1_v = max(a, max(b1, c1));
    let min1_v = min(a, min(b1, c1));
    ans = min(max1_v - min1_v, ans);

    // 横方向 -> 縦方向
    let b2 = (h - i) * ((w + (w % 2)) / 2);
    let c2 = (h - i) * ((w - (w % 2)) / 2);
    let max2_v = max(a, max(b2, c2));
    let min2_v = min(a, min(b2, c2));
    ans = min(max2_v - min2_v, ans);
  }

  println!("{}", ans);
}
