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
    let s1: String = read();
    let s2: String = read();
    let s3: String = read();

    let mut ans: String = "".to_string();

    ans.push(s1.to_uppercase().chars().nth(0).unwrap());
    ans.push(s2.to_uppercase().chars().nth(0).unwrap());
    ans.push(s3.to_uppercase().chars().nth(0).unwrap());
    println!("{}", ans);
}
