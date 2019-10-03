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

fn sieve_of_eratosthenes(n: i64) -> Vec<i64> {
  let mut spf = vec![None; n as usize + 1];
  let mut is_prime = vec![true; n as usize + 1];
  let mut primes = Vec::new();
  is_prime[0] = false;
  is_prime[1] = false;

  for i in 2..n + 1 {
    if is_prime[i as usize] {
      primes.push(i);
      spf[i as usize] = Some(i);
    }
    for prime in &primes {
      if i * prime >= n + 1 || prime > &spf[i as usize].unwrap() {
        break;
      }
      is_prime[(i * prime) as usize] = false;
      spf[(i * prime) as usize] = Some(*prime);
    }
  }
  primes
}


fn factorization_with_sieve_of_eratosthenes(mut n: i64) -> HashMap<i64, i64> {
  let mut primes = sieve_of_eratosthenes(f64::sqrt(n as f64) as i64);
  let mut hm_primes = HashMap::new();
  for prime in primes {
    while n % prime == 0 {
      n /= prime;
      if hm_primes.contains_key(&prime) {
        let mut x = hm_primes.get_mut(&prime).unwrap();
        *x += 1;
      } else {
        hm_primes.insert(prime, 1);
      }
    }
  }
  if n > 1 {
    if hm_primes.contains_key(&n) {
      let mut x = hm_primes.get_mut(&n).unwrap();
      *x += 1;
    } else {
      hm_primes.insert(n, 1);
    }
  }
  hm_primes
}

fn nCr(n: i64, r: i64) -> i64 {
  let mut ans = 1;

  for i in 1..r + 1 {
    let j = n - i + 1;
    let MOD: i64 = 1000000007;
    // println!("i: {}, j: {}", i, j);
    ans *= (j % MOD);
    ans %= MOD;
    ans /= (i);
  }
  return ans;
}

use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
  // nCr(1000000008, 9);
  let mut n: i64 = read();
  let mut m: i64 = read();
  let MOD: i64 = 1000000007;

  let yakusu = factorization_with_sieve_of_eratosthenes(m);

  // println!("{:?}", yakusu);
  let mut ans = 1;


  for (k, v) in yakusu.iter() {
    ans *= nCr(v + n - 1, *v);
    ans %= MOD;
  }

  println!("{}", ans)
}