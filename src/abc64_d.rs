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

use std::collections::VecDeque;

fn main() {
    let n: usize = read();
    let s: String = read();
    let mut stack: VecDeque<char> = VecDeque::new();
    let sc: Vec<char> = s.chars().collect();
    let mut err_cnt = 0;

    for i in 0..sc.len() {
        let si = sc[i];
        if si == '(' {
            stack.push_front(si);
        }
        if si == ')' {
            if stack.len() > 0 {
                let top = stack.pop_front();
            } else {
                err_cnt += 1;
            }
        }
    }
    let mut ans = "".to_string();
    for _ in 0..err_cnt {
        ans += &"(";
    }
    ans += &s;
    for _ in 0..stack.len() {
        ans += &")";
    }
    println!("{}", ans);
}
