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
    let o: String = read();
    let e: String = read();
    let ov: Vec<char> = o.chars().collect();
    let ev: Vec<char> = e.chars().collect();
    let mut ans: String = "".to_string();

    for i in 0..o.len() {
        ans += &ov[i].to_string();
        if i < e.len() {
            ans += &ev[i].to_string();
        }
    }
    println!("{}", ans);
}
