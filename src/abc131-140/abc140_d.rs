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

pub fn read_n<T: FromStr>(n: usize) -> Vec<T> {
  read_n_logic::<T>(n, vec![])
}

pub fn read_n_logic<T: FromStr>(n: usize, mut a: Vec<T>) -> Vec<T> {
  match n {
    0 => a,
    _ => {
      a.push(read::<T>());
      read_n_logic(n - 1, a)
    }
  }
}

use std::cmp::{max, min};

fn main() {
  let n: usize = read();
  let k: usize = read();
  let s: String = read();
  let s_vec: Vec<char> =  s.chars().collect();

  let mut s_sum = vec![];
	let mut ans = 0;

  let mut cnt = 0;
  let mut cc = 'L';
  for (idx, i) in s_vec.iter().enumerate() {
      if cnt == 0 {
          cnt += 1;
          cc = *i;
          continue
      }
      if cc == *i {
          cnt += 1;
      } else {
          s_sum.push(cnt);
          cnt = 1;
          cc = *i;
      }

      if idx == s_vec.len() -1 {
          s_sum.push(cnt);
      }
  }

	// println!("s_sum: {:?}", s_sum);
  let bl = ((k + 1) * 2 -1) as i64 ;
	// println!("bl: {}", bl);
	if s_sum.len() as i64 - bl <= 0 {
		// println!("sum return");
		println!("{}", s_sum.iter().fold(0, |sum, i| sum + i) -1);
		return
	}

	let mut prev = 0;
	let mut remain = 0;
	for ki in 0..bl {
		prev += s_sum[ki as usize];
	}

	for ki in bl..s_sum.len() as i64 {
		remain += s_sum[ki as usize] - 1;
	}

	ans = prev -1 + remain;
	// println!("ans: {}", ans);
	// println!("remain: {}, prev: {}", remain, prev);

	for i in 0..s_sum.len() as i64 - bl {
		let next = i + bl;
		let front = s_sum[next as usize];
		let back = s_sum[i as usize];
		prev -= back;
		prev += front;

		remain += back -1;
		remain -= front -1;

		ans = max(ans, prev -1 + remain);
		// println!("remain: {}, prev: {}", remain, prev);
		// println!("ans: {}", ans);
	}

	println!("{}", ans);
}
