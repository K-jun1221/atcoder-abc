// mod custom_lib;
use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
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


fn read_n<T: FromStr>(n: u32) -> Vec<T> {
  read_n_logic::<T>(n, vec![])
}

fn read_n_logic<T: FromStr>(n: u32, mut a: Vec<T>) -> Vec<T> {
  match n {
    0 => a,
    _ => {
      a.push(read::<T>());
      read_n_logic(n - 1, a)
    }
  }
}

fn main() {
  let n = read::<usize>();
  let m = read::<u32>();
  let a = read_n::<usize>(m);

  let mut dp = vec![0; n + 3];
  let mut st = vec![true; n + 3];
  let md = 1_000_000_007;
  dp[0] = 1;

  for i in a {
    st[i] = false;
  }

  if st[1] {
    dp[1] = 1;
  }


  for i in 2..n + 2 {
    if st[i] {
      dp[i] = (dp[i - 1] + dp[i - 2]) % md;
    }
  }

  println!("{}", dp[n] % md);
}