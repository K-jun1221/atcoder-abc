
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
    let a: String = read();
    let b: String = read();
    let mut ans: String = "".to_string();
    if a == "H" {
        if b == "H" {
            ans = "H".to_string();
        } else { 
            ans = "D".to_string();
        }
    } else {
        if b == "H" {
            ans = "D".to_string();
        } else { 
            ans = "H".to_string();
        }
    }
    println!("{}", ans);
}
