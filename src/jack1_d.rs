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

use std::collections::HashMap;
use std::collections::HashSet;

use std::cmp::{max, min};
fn main() {
    let n: i64 = read();
    let mut ans = 0;
    let mut dist_top: HashMap<i64, i64> = HashMap::new();

    for i in 1..n + 1 {
        let mut num = i;
        let mut dist: HashMap<i64, i64> = HashMap::new();
        let prime_num = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        for j in prime_num {
            if j <= i {
                loop {
                    if num % j != 0 {
                        break;
                    }
                    if dist.contains_key(&j) {
                        let current = dist.get_mut(&j).unwrap();
                        *current += 1;
                    } else {
                        dist.insert(j, 1);
                    }
                    num = num / j;
                }
            }
        }

        for (k, v) in dist.iter() {
            if dist_top.contains_key(&k) {
                let current = dist_top.get_mut(&k).unwrap();
                *current += *v;
            } else {
                dist_top.insert(*k, *v);
            }
        }
    }

    // println!("{:?}", dist_top);
    let mut a = vec![];
    for (k, v) in dist_top.iter() {
        a.push(v);
    }

    let cnt_74 = a.iter().filter(|&x| **x >= 74).count();
    let cnt_24 = a.iter().filter(|&x| **x >= 24).count();
    let cnt_2 = a.iter().filter(|&x| **x >= 2).count();
    let cnt_4 = a.iter().filter(|&x| **x >= 4).count();
    let cnt_14 = a.iter().filter(|&x| **x >= 14).count();

    // 74
    ans += cnt_74;

    // 24 * 2
    ans += cnt_24 * (cnt_2 - 1);

    // 14 * 4
    ans += cnt_14 * (cnt_4 - 1);

    // 2 * 4 * 4
    ans += cnt_4 * (cnt_4 - 1) / 2 * (cnt_2 - 2);

    println!("{}", ans)
}
