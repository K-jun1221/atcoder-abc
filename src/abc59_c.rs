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

use std::cmp::min;

fn main() {
    let n: usize = read();
    let mut a: Vec<i64> = vec![];
    for _ in 0..n {
        let ai = read();
        a.push(ai);
    }

    // first is -
    let mut flag = -1;
    let mut sum = 0;
    let mut ans1 = 0;
    for i in 0..a.len() {
        let ai = a[i];
        sum += ai;
        if flag == -1 && sum >= 0 {
            ans1 += (sum + 1).abs();
            sum = -1;
        }
        if flag == 1 && sum <= 0 {
            ans1 += (1 - sum).abs();
            sum = 1;
        }
        flag = -flag;
    }
    let mut flag = 1;
    let mut sum = 0;
    let mut ans2 = 0;
    for i in 0..a.len() {
        let ai = a[i];
        sum += ai;
        if flag == -1 && sum >= 0 {
            ans2 += (sum + 1).abs();
            sum = -1;
        }
        if flag == 1 && sum <= 0 {
            ans2 += (1 - sum).abs();
            sum = 1;
        }
        flag = -flag;
    }
    println!("{}", min(ans2, ans1));

    // first is +
}
