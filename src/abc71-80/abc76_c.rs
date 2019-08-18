
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
use std::collections::HashMap;

fn main() {
  let s = read::<String>();
  let t = read::<String>();
  let s_len = s.len();
  let t_len = t.len();

  let s_char: Vec<char> = s.chars().collect();
  let t_char: Vec<char> = t.chars().collect();
  let mut found = 0;

  if s_len < t_len {
    println!("UNRESTORABLE");
  } else {
    // let t_start = 0;
    for i in 0..s_len - t_len + 1 {
      let mut out = false;
      for j in 0..t_len {
        if t_char[j] != s_char[i + j] && s_char[i + j] != '?' {
          out = true;
          break;
        }
      }
      if !out {
        found = i;
      }
    }

    if found != 0 {
      let mut ans = String::from("");

      for x in 0..found {
        if s_char[x] == '?' {
          ans.push_str("a")
        } else {
          ans.push_str(&s_char[x].to_string())
        }
      }

      ans.push_str(&t);

      for x in found + t_len..s_len {
        if s_char[x] == '?' {
          ans.push_str("a")
        } else {
          ans.push_str(&s_char[x].to_string())
        }
      }

      println!("{}", ans);
    } else {
      println!("UNRESTORABLE");
    }

  }
}

